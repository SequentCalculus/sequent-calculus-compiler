use printer::{
    theme::ThemeExt,
    tokens::{COMMA, FAT_ARROW},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{Cns, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        context::{Chirality, ContextBinding},
        fresh_name, Covar, FsStatement, Name, Statement, TypingContext, Var,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T: PrdCns, S> {
    pub prdcns: T,
    pub xtor: Name,
    pub context: TypingContext,
    pub body: Rc<S>,
}

impl<T: PrdCns, S: Print> Print for Clause<T, S> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let prefix = if self.prdcns.is_prd() {
            alloc
                .dtor(&self.xtor)
                .append(self.context.print(cfg, alloc))
                .append(alloc.space())
                .append(FAT_ARROW)
        } else {
            alloc
                .ctor(&self.xtor)
                .append(self.context.print(cfg, alloc))
                .append(alloc.space())
                .append(FAT_ARROW)
        };
        let tail = alloc
            .line()
            .append(self.body.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
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
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        let mut vars_body = BTreeSet::new();
        self.body.typed_free_vars(&mut vars_body, state);

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
    ///N(K_i(x_{i,j}) => s_i ) = K_i(x_{i,j}) => N(s_i)
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
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        self.body.typed_free_vars(vars, state);
        for binding in &self.context.bindings {
            vars.remove(binding);
        }
    }
}
