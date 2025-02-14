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

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T: PrdCns, S> {
    pub prdcns: T,
    pub xtor: Name,
    pub context: TypingContext,
    pub rhs: Rc<S>,
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
            .append(self.rhs.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
    }
}

pub fn print_clauses<'a, T: Print>(
    cases: &'a [T],
    cfg: &printer::PrintCfg,
    alloc: &'a printer::Alloc<'a>,
) -> printer::Builder<'a> {
    match cases.len() {
        0 => alloc.space().braces_anno(),
        1 => alloc
            .line()
            .append(cases[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(alloc.intersperse(cases.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}

impl<T: PrdCns> Subst for Clause<T, Statement> {
    type Target = Clause<T, Statement>;
    fn subst_sim(
        self: &Clause<T, Statement>,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Clause<T, Statement> {
        let mut prod_subst_reduced: Vec<(Term<Prd>, Var)> = Vec::new();
        let mut cons_subst_reduced: Vec<(Term<Cns>, Covar)> = Vec::new();

        for subst in prod_subst {
            if !self.context.vars().contains(&subst.1) {
                prod_subst_reduced.push(subst.clone());
            }
        }
        for subst in cons_subst {
            if !self.context.vars().contains(&subst.1) {
                cons_subst_reduced.push(subst.clone());
            }
        }

        Clause {
            prdcns: self.prdcns.clone(),
            xtor: self.xtor.clone(),
            context: self.context.clone(),
            rhs: self
                .rhs
                .subst_sim(prod_subst_reduced.as_slice(), cons_subst_reduced.as_slice()),
        }
    }
}

impl<T: PrdCns> Uniquify for Clause<T, Statement> {
    fn uniquify(
        self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> Clause<T, Statement> {
        let mut new_context: TypingContext = TypingContext::default();
        let mut var_subst: Vec<(Term<Prd>, Var)> = Vec::new();
        let mut covar_subst: Vec<(Term<Cns>, Covar)> = Vec::new();

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
                        XVar {
                            prdcns: Prd,
                            var: new_var,
                            ty: binding.ty,
                        }
                        .into(),
                        binding.var,
                    ));
                } else {
                    covar_subst.push((
                        XVar {
                            prdcns: Cns,
                            var: new_var,
                            ty: binding.ty,
                        }
                        .into(),
                        binding.var,
                    ));
                }
            } else {
                seen_vars.insert(binding.var.clone());
                new_context.bindings.push(ContextBinding {
                    var: binding.var,
                    chi: binding.chi,
                    ty: binding.ty,
                });
            }
        }

        let new_statement = if var_subst.is_empty() && covar_subst.is_empty() {
            self.rhs
        } else {
            self.rhs.subst_sim(&var_subst, &covar_subst)
        };

        Clause {
            rhs: new_statement.uniquify(seen_vars, used_vars),
            context: new_context,
            ..self
        }
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
            rhs: self.rhs.focus(used_vars),
        }
    }
}

impl<T: PrdCns> SubstVar for Clause<T, FsStatement> {
    type Target = Clause<T, FsStatement>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause<T, FsStatement> {
        Clause {
            rhs: self.rhs.subst_sim(subst),
            ..self
        }
    }
}
