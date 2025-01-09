use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFZ, SEMI},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        statement::FsStatement,
        term::{Cns, Prd, Term},
        types::Ty,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Rc<Term<Prd>>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl IfZ {
    pub fn new<T, U, V>(ifc: T, thenc: U, elsec: V) -> IfZ
    where
        T: Into<Term<Prd>>,
        U: Into<Statement>,
        V: Into<Statement>,
    {
        IfZ {
            ifc: Rc::new(ifc.into()),
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }
}

impl Typed for IfZ {
    fn get_type(&self) -> Ty {
        self.thenc.get_type()
    }
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFZ).append(
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

impl Uniquify for IfZ {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> IfZ {
        let ifc = self.ifc.uniquify(seen_vars, used_vars);
        let mut seen_vars_thenc = seen_vars.clone();
        let mut used_vars_thenc = used_vars.clone();
        let thenc = self
            .thenc
            .uniquify(&mut seen_vars_thenc, &mut used_vars_thenc);
        let elsec = self.elsec.uniquify(seen_vars, used_vars);
        seen_vars.extend(seen_vars_thenc);
        used_vars.extend(used_vars_thenc);

        IfZ { ifc, thenc, elsec }
    }
}

impl Focusing for IfZ {
    type Target = FsStatement;
    ///N(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, N(s_1), N(s_2))]
    fn focus(self, state: &mut FocusingState) -> FsStatement {
        let cont = Box::new(|var, state: &mut FocusingState| {
            FsIfZ {
                ifc: var,
                thenc: self.thenc.focus(state),
                elsec: self.elsec.focus(state),
            }
            .into()
        });

        Rc::unwrap_or_clone(self.ifc).bind(cont, state)
    }
}

/// Focused IfZ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsIfZ {
    pub ifc: Var,
    pub thenc: Rc<FsStatement>,
    pub elsec: Rc<FsStatement>,
}

impl Print for FsIfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFZ).append(
            alloc
                .text(&self.ifc)
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

impl From<FsIfZ> for FsStatement {
    fn from(value: FsIfZ) -> Self {
        FsStatement::IfZ(value)
    }
}

impl SubstVar for FsIfZ {
    type Target = FsIfZ;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsIfZ {
        FsIfZ {
            ifc: self.ifc.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::statement::{FsCut, FsIfZ, FsStatement};
    use crate::syntax::term::Mu;
    use crate::syntax::{
        statement::{Cut, IfZ},
        term::{Literal, XVar},
        types::Ty,
        Statement,
    };
    use std::rc::Rc;

    #[test]
    fn transform_ifz1() {
        let result = IfZ {
            ifc: Rc::new(Literal::new(1).into()),
            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(Statement::Done(Ty::I64)),
        }
        .focus(&mut Default::default());
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu(
                "x0",
                FsIfZ {
                    ifc: "x0".to_string(),
                    thenc: Rc::new(
                        FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into(),
                    ),
                    elsec: Rc::new(FsStatement::Done()),
                },
                Ty::I64,
            ),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ifz2() {
        let result = IfZ {
            ifc: Rc::new(XVar::var("x", Ty::I64).into()),
            thenc: Rc::new(Statement::Done(Ty::I64)),
            elsec: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = FsIfZ {
            ifc: "x".to_string(),
            thenc: Rc::new(FsStatement::Done()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
