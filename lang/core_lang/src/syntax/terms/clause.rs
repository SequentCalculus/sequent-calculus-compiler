//! This module defines a clause in a pattern or copattern match in Core.

use printer::tokens::{COMMA, FAT_ARROW};
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};
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
    pub xtor: Name,
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
            alloc.dtor(&self.xtor)
        } else {
            alloc.ctor(&self.xtor)
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
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Clause<C> {
        let mut prod_subst_reduced: Vec<(Var, Term<Prd>)> = Vec::new();
        let mut cons_subst_reduced: Vec<(Covar, Term<Cns>)> = Vec::new();

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
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsClause<C> {
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
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Clause<C> {
        let mut new_context = TypingContext::default();
        let mut var_subst: Vec<(Var, Term<Prd>)> = Vec::new();
        let mut covar_subst: Vec<(Covar, Term<Cns>)> = Vec::new();

        for binding in self.context.bindings {
            if seen_vars.contains(&binding.var) {
                let new_var: Var = fresh_name(used_vars, &binding.var);
                seen_vars.insert(new_var.clone());
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
                seen_vars.insert(binding.var.clone());
                new_context.bindings.push(binding);
            }
        }

        self.context = new_context;

        self.body = if var_subst.is_empty() && covar_subst.is_empty() {
            self.body.uniquify(seen_vars, used_vars)
        } else {
            self.body
                .subst_sim(&var_subst, &covar_subst)
                .uniquify(seen_vars, used_vars)
        };

        self
    }
}

impl<C: Chi> Focusing for Clause<C> {
    type Target = FsClause<C>;
    // focus(X_i(x_{i,j}) => s_i ) = X_i(x_{i,j}) => focus(s_i)
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsClause<C> {
        Clause {
            prdcns: self.prdcns,
            xtor: self.xtor,
            context: self.context,
            body: self.body.focus(used_vars),
        }
    }
}
