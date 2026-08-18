//! This module defines a clause in a pattern or copattern match in Core.

use printer::tokens::{COMMA, FAT_ARROW};
use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet, collect_type_flow};
use crate::mono::errors::MonoError;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;

use std::collections::BTreeSet;
use std::rc::Rc;

/// This struct defines a clause in a match or a comatch in Core. It consists of the information
/// that determines whether it is in a match (if `C` is instantiated with [`Cns`]) or a comatch
/// (if `C` is instantiated with [`Prd`]), of a name of the corresponding xtor, of the context it
/// binds for the arguments, and of the body. The type parameter `S` determines whether the body
/// statement is unfocused (if `S` is instantiated with [`Statement`], which is the default) or
/// focused (if `S` is instantiated with [`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<C: Chi, S = Statement> {
    /// Whether we have a clause of a match or comatch
    pub prdcns: C,
    /// The name of the xtor
    pub xtor: Identifier,
    /// The type parameters of the xtor
    pub type_params: Vec<Identifier>,
    /// The bindings to which the arguments of the xtor are bound
    pub context: TypingContext,
    /// The body of the pattern, either unfocused ([`Statement`]) or focused ([`FsStatement`])
    pub body: Rc<S>,
}

#[allow(type_alias_bounds)]
pub type FsClause<C: Chi> = Clause<C, FsStatement>;

impl<C: Chi, S: Print> Print for Clause<C, S> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let context = if self.context.bindings.is_empty() {
            alloc.nil()
        } else {
            self.context.print(cfg, alloc).parens()
        };

        let xtor = if self.prdcns.is_prd() {
            alloc.dtor(&self.xtor.name.print_to_string(Some(cfg)))
        } else {
            alloc.ctor(&self.xtor.name.print_to_string(Some(cfg)))
        };
        xtor.append(context.group())
            .append(alloc.space())
            .append(FAT_ARROW)
            .align()
            .append(alloc.line())
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
    }
}

pub fn print_clauses<'a, T: Print>(
    clauses: &'a [T],
    cfg: &PrintCfg,
    alloc: &'a Alloc<'a>,
) -> Builder<'a> {
    match clauses.len() {
        0 => alloc.space().braces_anno(),
        1 => alloc
            .line()
            .append(clauses[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(
                    alloc.intersperse(
                        clauses
                            .iter()
                            .map(|clauses| clauses.print(cfg, alloc).group()),
                        sep,
                    ),
                )
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}

impl<C: Chi> Subst for Clause<C> {
    type Target = Clause<C>;
    fn subst_sim(
        mut self,
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
    ) -> Clause<C> {
        let mut prod_subst_reduced: Vec<(Identifier, Term<Prd>)> = Vec::new();
        let mut cons_subst_reduced: Vec<(Identifier, Term<Cns>)> = Vec::new();
        for subst in prod_subst {
            if !self.context.vars().contains(&subst.0) {
                prod_subst_reduced.push(subst.clone());
            }
        }
        for subst in cons_subst {
            if !self.context.vars().contains(&subst.0) {
                cons_subst_reduced.push(subst.clone());
            }
        }

        self.body = self
            .body
            .subst_sim(prod_subst_reduced.as_slice(), cons_subst_reduced.as_slice());
        self
    }
}

impl<C: Chi> SubstVar for FsClause<C> {
    type Target = FsClause<C>;
    fn subst_sim(mut self, subst: &[(ID, Identifier)]) -> FsClause<C> {
        self.body = self.body.subst_sim(subst);
        self
    }
}

impl<C: Chi> TypedFreeVars for Clause<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        let mut vars_body = BTreeSet::new();
        self.body.typed_free_vars(&mut vars_body);

        for binding in &self.context.bindings {
            vars_body.remove(binding);
        }

        vars.extend(vars_body);
    }
}

impl<C: Chi> TypedFreeVars for FsClause<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        // all binders in focused terms are unique in each path through the program, so we do not
        // need a fresh set under binders
        self.body.typed_free_vars(vars);
        for binding in &self.context.bindings {
            vars.remove(binding);
        }
    }
}

impl<C: Chi> Uniquify for Clause<C> {
    fn uniquify(mut self, max_id: &mut ID) -> Clause<C> {
        let mut new_context = TypingContext::default();
        let mut var_subst: Vec<(Identifier, Term<Prd>)> = Vec::new();
        let mut covar_subst: Vec<(Identifier, Term<Cns>)> = Vec::new();

        for binding in self.context.bindings {
            if binding.var.id == 0 {
                let new_var = fresh_identifier(max_id, &binding.var.name);
                new_context.bindings.push(ContextBinding {
                    var: new_var.clone(),
                    chi: binding.chi.clone(),
                    ty: binding.ty.clone(),
                });

                if binding.chi == Chirality::Prd {
                    var_subst.push((
                        binding.var,
                        XVar {
                            prdcns: Prd,
                            var: new_var,
                            ty: binding.ty,
                        }
                        .into(),
                    ));
                } else {
                    covar_subst.push((
                        binding.var,
                        XVar {
                            prdcns: Cns,
                            var: new_var,
                            ty: binding.ty,
                        }
                        .into(),
                    ));
                }
            } else {
                new_context.bindings.push(binding);
            }
        }

        self.context = new_context;

        self.body = if var_subst.is_empty() && covar_subst.is_empty() {
            self.body.uniquify(max_id)
        } else {
            self.body
                .subst_sim(&var_subst, &covar_subst)
                .uniquify(max_id)
        };

        self
    }
}

impl<C: Chi> Focusing for Clause<C> {
    type Target = FsClause<C>;
    // focus(X_i(x_{i,j}) => s_i ) = X_i(x_{i,j}) => focus(s_i)
    fn focus(self, max_id: &mut ID) -> FsClause<C> {
        Clause {
            prdcns: self.prdcns,
            xtor: self.xtor,
            type_params: self.type_params,
            context: self.context,
            body: self.body.focus(max_id),
        }
    }
}

impl<C: Chi> ConstraintCollector for Clause<C> {
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        let xtor_params: Vec<Ty> = if self.prdcns.is_cns() {
            env.lookup_xtor_for_data_decl(&self.xtor)?
                .type_params
                .into_iter()
                .map(Ty::Var)
                .collect()
        } else {
            env.lookup_xtor_for_codata_decl(&self.xtor)?
                .type_params
                .into_iter()
                .map(Ty::Var)
                .collect()
        };

        let mut constraints = collect_type_flow(&xtor_params, &self.type_params)?;

        // collect constraints from the body of the clause
        constraints.extend(self.body.collect_constraints(env)?);
        Ok(constraints)
    }
}

impl<C: Chi> Checked for Clause<C> {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        // extend the type parameters with the type parameters of the clause
        let extended_type_params = [type_params, &self.type_params].concat();
        self.context.check(&extended_type_params, context, env)?;

        // extend the context of the clause with the bindings of the clause
        let mut extended_context = context.clone();
        for binding in &self.context.bindings {
            extended_context.bindings.push(binding.clone());
        }

        // check the body of the clause under the extended context of the clause
        self.body
            .check(&extended_type_params, &extended_context, env)?;

        Ok(())
    }
}

#[cfg(test)]
mod label_and_unify_tests {
    use crate::splitting::labeling::{
        DeclSignature, DeclSignatures, SplitState, label_and_unify_clause,
    };
    use crate::syntax::*;
    extern crate self as core_lang;
    use core_macros::{bind, clause, covar, cut, id, prd, ty, var};

    #[test]
    fn label_and_unify_merges_binding_with_declared_field() {
        let mut state = SplitState::default();
        let field_label = state.label_ty(&ty!(id!("Box")));

        let mut sigs = DeclSignatures::new();
        sigs.insert(
            id!("Cons"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![field_label.clone()],
            },
        );

        // `a` is the outer continuation the clause body cuts against; it must already be in
        // scope, exactly like it would be for a real match's surrounding context.
        let mut scope = TypingContext::default();
        scope.bindings.push(ContextBinding {
            var: id!("a"),
            chi: Chirality::Cns,
            ty: Ty::I64,
        });

        let example = clause!(
            Cns,
            id!("Cons"),
            [],
            [bind!(id!("x"), prd!(), ty!(id!("Box")))],
            cut!(
                var!(id!("x"), ty!(id!("Box"))),
                covar!(id!("a")),
                ty!(id!("Box"))
            )
        );

        let result: Clause<Cns> = label_and_unify_clause(&example, &mut state, &sigs, &scope, &[]);
        let binding_ty = result.context.bindings[0].ty.clone();

        let (
            Ty::Decl {
                name: field_name, ..
            },
            Ty::Decl {
                name: binding_name, ..
            },
        ) = (&field_label, &binding_ty)
        else {
            panic!("expected Ty::Decl on both sides");
        };
        assert_eq!(state.uf.find(field_name), state.uf.find(&binding_name));

        // scope threading: the body's occurrence of `x` must carry exactly the binding's label
        let Statement::Cut(body_cut) = result.body.as_ref() else {
            panic!("expected a Cut");
        };
        let Term::XVar(producer) = body_cut.producer.as_ref() else {
            panic!("expected an XVar producer");
        };
        assert_eq!(producer.ty, binding_ty);
    }
}
