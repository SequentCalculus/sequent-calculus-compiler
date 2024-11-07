use printer::{
    theme::ThemeExt,
    tokens::{COMMA, SEMI},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::term::{Cns, Prd, Term, XVar},
    traits::{
        focus::{Bind, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};
use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Rc<Term<Prd>>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword("IfZ").append(
            self.ifc
                .print(cfg, alloc)
                .append(SEMI)
                .append(alloc.space())
                .append(self.thenc.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(self.elsec.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeV for IfZ {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.ifc.free_vars();
        free_vars.extend(self.thenc.free_vars());
        free_vars.extend(self.elsec.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.ifc.free_covars();
        free_covars.extend(self.thenc.free_covars());
        free_covars.extend(self.elsec.free_covars());
        free_covars
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        IfZ {
            ifc: self.ifc.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Focusing for IfZ {
    type Target = Statement;
    ///N(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, N(s_1), N(s_2))]
    fn focus(self, state: &mut FocusingState) -> Statement {
        let then_transformed = self.thenc.focus(state);
        let else_transformed = self.elsec.focus(state);
        let cont = Box::new(|var, _: &mut FocusingState| {
            IfZ {
                ifc: Rc::new(XVar { prdcns: Prd, var }.into()),
                thenc: then_transformed,
                elsec: else_transformed,
            }
            .into()
        });

        Rc::unwrap_or_clone(self.ifc).bind(cont, state)
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        statement::{Cut, IfZ},
        term::{Cns, Literal, Mu, XVar},
        Statement,
    };
    use std::rc::Rc;

    fn example_ifz1() -> IfZ {
        IfZ {
            ifc: Rc::new(Literal::new(1).into()),
            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a")).into()),
            elsec: Rc::new(Statement::Done()),
        }
    }
    fn example_ifz2() -> IfZ {
        IfZ {
            ifc: Rc::new(XVar::var("x").into()),
            thenc: Rc::new(Statement::Done()),
            elsec: Rc::new(Cut::new(XVar::var("x"), XVar::covar("a")).into()),
        }
    }

    #[test]
    fn transform_ifz1() {
        let result = example_ifz1().focus(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        IfZ {
                            ifc: Rc::new(XVar::var("x0").into()),
                            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a")).into()),
                            elsec: Rc::new(Statement::Done()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ifz2() {
        let result = example_ifz2().focus(&mut Default::default());
        let expected = example_ifz2().into();
        assert_eq!(result, expected)
    }
}
