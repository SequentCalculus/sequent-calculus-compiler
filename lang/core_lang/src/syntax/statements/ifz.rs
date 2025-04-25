use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{ELSE, EQQ, IF, NEQ, ZERO},
    util::BracesExt,
};

use super::{ContextBinding, Covar, Statement, Var};
use crate::{
    syntax::{
        FsStatement,
        context::Chirality,
        terms::{Cns, Prd, Term},
        types::Ty,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfZSort {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub sort: IfZSort,
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
            sort: IfZSort::Equal,
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
        let comparison = match self.sort {
            IfZSort::Equal => EQQ,
            IfZSort::NotEqual => NEQ,
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.ifc.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(ZERO)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl Subst for IfZ {
    type Target = IfZ;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.ifc = self.ifc.subst_sim(prod_subst, cons_subst);

        self.thenc = self.thenc.subst_sim(prod_subst, cons_subst);
        self.elsec = self.elsec.subst_sim(prod_subst, cons_subst);

        self
    }
}

impl TypedFreeVars for IfZ {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.ifc.typed_free_vars(vars);
        self.thenc.typed_free_vars(vars);
        self.elsec.typed_free_vars(vars);
    }
}

impl Uniquify for IfZ {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> IfZ {
        self.ifc = self.ifc.uniquify(seen_vars, used_vars);

        let mut seen_vars_thenc = seen_vars.clone();
        let mut used_vars_thenc = used_vars.clone();
        self.thenc = self
            .thenc
            .uniquify(&mut seen_vars_thenc, &mut used_vars_thenc);
        self.elsec = self.elsec.uniquify(seen_vars, used_vars);
        seen_vars.extend(seen_vars_thenc);
        used_vars.extend(used_vars_thenc);

        self
    }
}

impl Focusing for IfZ {
    type Target = FsStatement;
    ///N(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, N(s_1), N(s_2))]
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        let cont = Box::new(
            move |binding: ContextBinding, used_vars: &mut HashSet<Var>| {
                FsIfZ {
                    sort: self.sort,
                    ifc: binding.var,
                    thenc: self.thenc.focus(used_vars),
                    elsec: self.elsec.focus(used_vars),
                }
                .into()
            },
        );

        Rc::unwrap_or_clone(self.ifc).bind(cont, used_vars)
    }
}

/// Focused IfZ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsIfZ {
    pub sort: IfZSort,
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
        let comparison = match self.sort {
            IfZSort::Equal => EQQ,
            IfZSort::NotEqual => NEQ,
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.ifc.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(ZERO)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
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
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsIfZ {
        self.ifc = self.ifc.subst_sim(subst);
        self.thenc = self.thenc.subst_sim(subst);
        self.elsec = self.elsec.subst_sim(subst);
        self
    }
}

impl TypedFreeVars for FsIfZ {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.ifc.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
        self.thenc.typed_free_vars(vars);
        self.elsec.typed_free_vars(vars);
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        statements::{Cut, Exit, FsCut, FsExit, FsIfZ, IfZ, IfZSort},
        terms::{Literal, Mu, XVar},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn transform_ifz1() {
        let result = IfZ {
            sort: IfZSort::Equal,
            ifc: Rc::new(Literal::new(1).into()),
            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(Exit::exit(XVar::var("x", Ty::I64), Ty::I64).into()),
        }
        .focus(&mut Default::default());
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu(
                "x0",
                FsIfZ {
                    sort: IfZSort::Equal,
                    ifc: "x0".to_string(),
                    thenc: Rc::new(
                        FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into(),
                    ),
                    elsec: Rc::new(FsExit::exit("x").into()),
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
            sort: IfZSort::Equal,
            ifc: Rc::new(XVar::var("x", Ty::I64).into()),
            thenc: Rc::new(Exit::exit(XVar::var("y", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = FsIfZ {
            sort: IfZSort::Equal,
            ifc: "x".to_string(),
            thenc: Rc::new(FsExit::exit("y").into()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
