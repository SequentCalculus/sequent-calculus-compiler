//! This module defines a clause in a pattern or copattern match in Core.

use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{COMMA, FAT_ARROW},
    util::BracesExt,
};

use super::{Cns, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        Covar, FsStatement, Name, Statement, TypingContext, Var,
        context::{Chirality, ContextBinding},
        fresh_name,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines a clause in a match or a comatch in Core. It consists of the information
/// that determines whether it is in a match (if `T` is instantiated with [`Cns`]) or a comatch
/// (if `T` is instantiated with [`Prd`]), of a name of the corresponding xtor, of the context it
/// binds for the arguments, and of the body. The type parameter `S` determines whether the body
/// statement is unfocused ([`Statement`]) or focused ([`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T: PrdCns, S> {
    /// Whether we have a clause of a match or comatch
    pub prdcns: T,
    /// The name of the xtor
    pub xtor: Name,
    /// The bindings to which the arguments of the xtor are bound
    pub context: TypingContext,
    /// The body of the pattern, either unfocused ([`Statement`]) or focused ([`FsStatement`])
    pub body: Rc<S>,
}

impl<T: PrdCns, S: Print> Print for Clause<T, S> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let xtor = if self.prdcns.is_prd() {
            alloc.dtor(&self.xtor)
        } else {
            alloc.ctor(&self.xtor)
        };
        xtor.append(self.context.print(cfg, alloc))
            .append(alloc.space())
            .append(FAT_ARROW)
            .append(alloc.line())
            .append(self.body.print(cfg, alloc))
            .nest(cfg.indent)
            .group()
    }
}

pub fn print_clauses<'a, T: Print>(
    clauses: &'a [T],
    cfg: &printer::PrintCfg,
    alloc: &'a printer::Alloc<'a>,
) -> printer::Builder<'a> {
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
                .append(alloc.intersperse(clauses.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}

impl<T: PrdCns> Subst for Clause<T, Statement> {
    type Target = Clause<T, Statement>;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Clause<T, Statement> {
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

impl<T: PrdCns> TypedFreeVars for Clause<T, Statement> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        let mut vars_body = BTreeSet::new();
        self.body.typed_free_vars(&mut vars_body);

        for binding in &self.context.bindings {
            vars_body.remove(binding);
        }

        vars.extend(vars_body);
    }
}

impl<T: PrdCns> Uniquify for Clause<T, Statement> {
    fn uniquify(
        mut self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> Clause<T, Statement> {
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

impl<T: PrdCns> Focusing for Clause<T, Statement> {
    type Target = Clause<T, FsStatement>;
    // focus(X_i(x_{i,j}) => s_i ) = X_i(x_{i,j}) => focus(s_i)
    fn focus(self, used_vars: &mut HashSet<Var>) -> Clause<T, FsStatement> {
        Clause {
            prdcns: self.prdcns,
            xtor: self.xtor,
            context: self.context,
            body: self.body.focus(used_vars),
        }
    }
}

impl<T: PrdCns> SubstVar for Clause<T, FsStatement> {
    type Target = Clause<T, FsStatement>;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Clause<T, FsStatement> {
        self.body = self.body.subst_sim(subst);
        self
    }
}

impl<T: PrdCns> TypedFreeVars for Clause<T, FsStatement> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        // all binders in focused terms are unique, so we do not need a fresh set under binders
        self.body.typed_free_vars(vars);
        for binding in &self.context.bindings {
            vars.remove(binding);
        }
    }
}
