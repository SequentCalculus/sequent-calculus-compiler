use printer::{
    tokens::{COMMA, FAT_ARROW},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{
    context::{context_covars, context_vars, ContextBinding, TypingContext},
    statement::FsStatement,
    term::{Cns, Prd, Term, XVar},
    Covar, Name, Statement, Var,
};
use crate::{
    syntax_var::FsTypingContext,
    traits::{
        focus::{Focusing, FocusingState},
        free_vars::{fresh_var, FreeV},
        substitution::{Subst, SubstVar},
        uniquify::Uniquify,
        used_binders::UsedBinders,
    },
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub rhs: Rc<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsClause {
    pub xtor: Name,
    pub context: FsTypingContext,
    pub case: Rc<FsStatement>,
}

impl Print for FsClause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let params = if self.context.is_empty() {
            alloc.nil()
        } else {
            self.context.print(cfg, alloc).parens()
        };
        let prefix = alloc
            .text(&self.xtor)
            .append(params)
            .append(alloc.space())
            .append(FAT_ARROW);
        let tail = alloc
            .line()
            .append(self.case.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
    }
}

impl SubstVar for FsClause {
    type Target = FsClause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsClause {
        FsClause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl Print for Clause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let params = if self.context.is_empty() {
            alloc.nil()
        } else {
            self.context.print(cfg, alloc).parens()
        };
        let prefix = alloc
            .text(&self.xtor)
            .append(params)
            .append(alloc.space())
            .append(FAT_ARROW);
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

impl FreeV for Clause {
    fn free_vars(self: &Clause) -> HashSet<Var> {
        let mut free_vars = self.rhs.free_vars();
        for bnd in &self.context {
            if let ContextBinding::VarBinding { var, ty: _ } = bnd {
                free_vars.remove(var);
            }
        }
        free_vars
    }
    fn free_covars(self: &Clause) -> HashSet<Covar> {
        let mut free_covars = self.rhs.free_covars();
        for bnd in &self.context {
            if let ContextBinding::CovarBinding { covar, ty: _ } = bnd {
                free_covars.remove(covar);
            }
        }
        free_covars
    }
}

impl UsedBinders for Clause {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for binding in &self.context {
            match binding {
                ContextBinding::VarBinding { var, .. } => {
                    used.insert(var.clone());
                }
                ContextBinding::CovarBinding { covar, .. } => {
                    used.insert(covar.clone());
                }
            }
        }
        self.rhs.used_binders(used);
    }
}

impl Subst for Clause {
    type Target = Clause;
    fn subst_sim(
        self: &Clause,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Clause {
        let mut prod_subst_reduced: Vec<(Term<Prd>, Var)> = Vec::new();
        let mut cons_subst_reduced: Vec<(Term<Cns>, Covar)> = Vec::new();

        let context_vars = context_vars(&self.context);
        let context_covars = context_covars(&self.context);

        for subst in prod_subst {
            if !context_vars.contains(&subst.1) {
                prod_subst_reduced.push(subst.clone());
            }
        }
        for subst in cons_subst {
            if !context_covars.contains(&subst.1) {
                cons_subst_reduced.push(subst.clone());
            }
        }

        Clause {
            xtor: self.xtor.clone(),
            context: self.context.clone(),
            rhs: self
                .rhs
                .subst_sim(prod_subst_reduced.as_slice(), cons_subst_reduced.as_slice()),
        }
    }
}

impl Uniquify for Clause {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Clause {
        let mut new_context: TypingContext = Vec::new();
        let mut var_subst: Vec<(Term<Prd>, Var)> = Vec::new();
        let mut covar_subst: Vec<(Term<Cns>, Covar)> = Vec::new();

        for binding in self.context {
            match binding {
                ContextBinding::VarBinding { var, ty } => {
                    if seen_vars.contains(&var) {
                        let new_var: Var = fresh_var(used_vars, &var);
                        seen_vars.insert(new_var.clone());
                        new_context.push(ContextBinding::VarBinding {
                            var: new_var.clone(),
                            ty: ty.clone(),
                        });
                        var_subst.push((
                            XVar {
                                prdcns: Prd,
                                var: new_var,
                                ty,
                            }
                            .into(),
                            var,
                        ));
                    } else {
                        seen_vars.insert(var.clone());
                        new_context.push(ContextBinding::VarBinding { var, ty });
                    }
                }
                ContextBinding::CovarBinding { covar, ty } => {
                    if seen_vars.contains(&covar) {
                        let new_covar: Covar = fresh_var(used_vars, &covar);
                        seen_vars.insert(new_covar.clone());
                        new_context.push(ContextBinding::CovarBinding {
                            covar: new_covar.clone(),
                            ty: ty.clone(),
                        });
                        covar_subst.push((
                            XVar {
                                prdcns: Cns,
                                var: new_covar,
                                ty,
                            }
                            .into(),
                            covar.clone(),
                        ));
                    } else {
                        seen_vars.insert(covar.clone());
                        new_context.push(ContextBinding::CovarBinding { covar, ty });
                    }
                }
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

impl Focusing for Clause {
    type Target = crate::syntax::clause::FsClause;
    ///N(K_i(x_{i,j}) => s_i ) = K_i(x_{i,j}) => N(s_i)
    fn focus(self, state: &mut FocusingState) -> crate::syntax::clause::FsClause {
        state.add_context(&self.context);
        crate::syntax::clause::FsClause {
            xtor: self.xtor,
            context: self.context.focus(state),
            case: self.rhs.focus(state),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::Chirality;
    use crate::syntax::{
        context::ContextBinding,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Clause,
    };
    use std::rc::Rc;

    fn example_clause1() -> Clause {
        Clause {
            xtor: "Tup".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::VarBinding {
                    var: "y".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            rhs: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }
    fn example_clause1_var() -> crate::syntax::clause::FsClause {
        crate::syntax::clause::FsClause {
            xtor: "Tup".to_owned(),
            context: vec![
                crate::syntax_var::FsContextBinding {
                    chi: Chirality::Prd,
                    var: "x".to_owned(),
                    ty: crate::syntax::Ty::Int(),
                },
                crate::syntax_var::FsContextBinding {
                    chi: Chirality::Prd,
                    var: "y".to_owned(),
                    ty: crate::syntax::Ty::Int(),
                },
                crate::syntax_var::FsContextBinding {
                    chi: Chirality::Cns,
                    var: "a".to_owned(),
                    ty: crate::syntax::Ty::Int(),
                },
            ],
            case: Rc::new(
                crate::syntax::statement::cut::FsCut {
                    producer: Rc::new(
                        crate::syntax::term::xvar::FsXVar {
                            chi: Chirality::Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    ty: crate::syntax::Ty::Int(),
                    consumer: Rc::new(
                        crate::syntax::term::xvar::FsXVar {
                            chi: Chirality::Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }

    fn example_clause2() -> Clause {
        Clause {
            xtor: "Ap".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            rhs: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }
    fn example_clause2_var() -> crate::syntax::clause::FsClause {
        crate::syntax::clause::FsClause {
            xtor: "Ap".to_owned(),
            context: vec![
                crate::syntax_var::FsContextBinding {
                    chi: Chirality::Prd,
                    var: "x".to_owned(),
                    ty: crate::syntax::Ty::Int(),
                },
                crate::syntax_var::FsContextBinding {
                    chi: Chirality::Cns,
                    var: "a".to_owned(),
                    ty: crate::syntax::Ty::Int(),
                },
            ],
            case: Rc::new(
                crate::syntax::statement::cut::FsCut {
                    producer: Rc::new(
                        crate::syntax::term::xvar::FsXVar {
                            chi: Chirality::Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    ty: crate::syntax::Ty::Int(),
                    consumer: Rc::new(
                        crate::syntax::term::xvar::FsXVar {
                            chi: Chirality::Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn transform_clause1() {
        let result = example_clause1().focus(&mut Default::default());
        let expected = example_clause1_var();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_clause2() {
        let result = example_clause2().focus(&mut Default::default());
        let expected = example_clause2_var();
        assert_eq!(result, expected)
    }
}
