//! This module defines the conditionals comparing two integers in Core.

use printer::tokens::{ELSE, EQQ, GT, GTE, IF, LT, LTE, NEQ, ZERO};
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This enum encodes the comparison operation used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSort {
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterOrEqual,
}

impl Print for IfSort {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            IfSort::Equal => alloc.text(EQQ),
            IfSort::NotEqual => alloc.text(NEQ),
            IfSort::Less => alloc.text(LT),
            IfSort::LessOrEqual => alloc.text(LTE),
            IfSort::Greater => alloc.text(GT),
            IfSort::GreaterOrEqual => alloc.text(GTE),
        }
    }
}

/// This struct defines the conditionals comparing either two terms or one term to zero in Core. It
/// consists of the comparison operation, the first term and an optional second term, and the
/// then-branch and else-branch, and after typechecking also of the inferred type. The type
/// parameters `P` and `S` determine whether this is the unfocused variant (if `P` and `S` are
/// instantiated with [`Term<Prd>`] and [`Statement`], which is the default) or the focused variant
/// (if `P` and `C` is instantiated with [`Var`] and [`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfC<P = Rc<Term<Prd>>, S = Statement> {
    /// The comparison operation
    pub sort: IfSort,
    /// The first term of the comparison
    pub fst: P,
    /// The optional second term of the comparison
    pub snd: Option<P>,
    /// The then-branch
    pub thenc: Rc<S>,
    /// The else-branch
    pub elsec: Rc<S>,
}

pub type FsIfC = IfC<Var, FsStatement>;

impl IfC {
    /// This function creates a conditional with `==` comparison for given operands and then- and
    /// else-statements.
    pub fn ife<T, U, V, W>(fst: T, snd: U, thenc: V, elsec: W) -> IfC
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
        V: Into<Statement>,
        W: Into<Statement>,
    {
        IfC {
            sort: IfSort::Equal,
            fst: Rc::new(fst.into()),
            snd: Some(Rc::new(snd.into())),
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }

    /// This function creates a conditional with `<` comparison for given operands and then- and
    /// else-statements.
    pub fn ifl<T, U, V, W>(fst: T, snd: U, thenc: V, elsec: W) -> IfC
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
        V: Into<Statement>,
        W: Into<Statement>,
    {
        IfC {
            sort: IfSort::Less,
            fst: Rc::new(fst.into()),
            snd: Some(Rc::new(snd.into())),
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }

    /// This function creates a conditional with `== 0` comparison for given operands and then- and
    /// else-statements.
    pub fn ifz<T, V, W>(fst: T, thenc: V, elsec: W) -> IfC
    where
        T: Into<Term<Prd>>,
        V: Into<Statement>,
        W: Into<Statement>,
    {
        IfC {
            sort: IfSort::Equal,
            fst: Rc::new(fst.into()),
            snd: None,
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }
}

impl Typed for IfC {
    fn get_type(&self) -> Ty {
        self.thenc.get_type()
    }
}

impl<P, S> Print for IfC<P, S>
where
    P: Print,
    S: Print,
{
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let snd = match self.snd {
            None => alloc.text(ZERO),
            Some(ref snd) => snd.print(cfg, alloc),
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(self.sort.print(cfg, alloc))
            .append(alloc.space())
            .append(snd)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc).group())
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
                    .append(self.elsec.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
    }
}

impl From<IfC> for Statement {
    fn from(value: IfC) -> Self {
        Statement::IfC(value)
    }
}

impl From<FsIfC> for FsStatement {
    fn from(value: FsIfC) -> Self {
        FsStatement::IfC(value)
    }
}

impl Subst for IfC {
    type Target = IfC;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.fst = self.fst.subst_sim(prod_subst, cons_subst);
        self.snd = self.snd.subst_sim(prod_subst, cons_subst);

        self.thenc = self.thenc.subst_sim(prod_subst, cons_subst);
        self.elsec = self.elsec.subst_sim(prod_subst, cons_subst);

        self
    }
}

impl SubstVar for FsIfC {
    type Target = FsIfC;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsIfC {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self.thenc = self.thenc.subst_sim(subst);
        self.elsec = self.elsec.subst_sim(subst);

        self
    }
}

impl TypedFreeVars for IfC {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.fst.typed_free_vars(vars);
        self.snd.typed_free_vars(vars);
        self.thenc.typed_free_vars(vars);
        self.elsec.typed_free_vars(vars);
    }
}

impl TypedFreeVars for FsIfC {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.fst.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
        if let Some(ref snd) = self.snd {
            vars.insert(ContextBinding {
                var: snd.clone(),
                chi: Chirality::Prd,
                ty: Ty::I64,
            });
        }
        self.thenc.typed_free_vars(vars);
        self.elsec.typed_free_vars(vars);
    }
}

impl Uniquify for IfC {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> IfC {
        self.fst = self.fst.uniquify(seen_vars, used_vars);
        self.snd = self.snd.uniquify(seen_vars, used_vars);

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

impl Focusing for IfC {
    type Target = FsStatement;
    // focus(ifc(p_1, p_2, s_1, s_2)) = bind(p_1)[λa1.bind(p_1)[λa2.ifc(a_1, a_2, focus(s_1), focus(s_2))]] OR
    // focus(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, focus(s_1), focus(s_2))]
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        Rc::unwrap_or_clone(self.fst).bind(
            Box::new(
                move |binding_fst: ContextBinding, used_vars: &mut HashSet<Var>| match self.snd {
                    None => FsIfC {
                        sort: self.sort,
                        fst: binding_fst.var,
                        snd: None,
                        thenc: self.thenc.focus(used_vars),
                        elsec: self.elsec.focus(used_vars),
                    }
                    .into(),
                    Some(snd) => Rc::unwrap_or_clone(snd).bind(
                        Box::new(move |binding_snd, used_vars: &mut HashSet<Var>| {
                            FsIfC {
                                sort: self.sort,
                                fst: binding_fst.var,
                                snd: Some(binding_snd.var),
                                thenc: self.thenc.focus(used_vars),
                                elsec: self.elsec.focus(used_vars),
                            }
                            .into()
                        }),
                        used_vars,
                    ),
                },
            ),
            used_vars,
        )
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::syntax::*;
    use crate::traits::*;
    use macros::{cut, ifc, ifcz, ty};
    use std::rc::Rc;
    extern crate self as core_lang;

    #[test]
    fn transform_ife1() {
        let result = ifc!(
            IfSort::Equal,
            Literal::new(2),
            Literal::new(1),
            cut!(Literal::new(1), XVar::covar("a", ty!("int")),),
            Exit::exit(XVar::var("x", ty!("int")), ty!("int"))
        )
        .focus(&mut Default::default());

        let expected = FsCut::new(
            Literal::new(2),
            Mu::tilde_mu(
                "x0",
                FsCut::new(
                    Literal::new(1),
                    Mu::tilde_mu(
                        "x1",
                        FsIfC {
                            sort: IfSort::Equal,
                            fst: "x0".to_string(),
                            snd: Some("x1".to_string()),
                            thenc: Rc::new(
                                FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64)
                                    .into(),
                            ),
                            elsec: Rc::new(FsExit::exit("x").into()),
                        },
                        Ty::I64,
                    ),
                    Ty::I64,
                ),
                Ty::I64,
            ),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ife2() {
        let result = ifc!(
            IfSort::Equal,
            XVar::var("x", ty!("int")),
            XVar::var("x", ty!("int")),
            Exit::exit(XVar::var("y", ty!("int")), ty!("int")),
            cut!(XVar::var("x", ty!("int")), XVar::covar("a", ty!("int")),)
        )
        .focus(&mut Default::default());
        let expected = FsIfC {
            sort: IfSort::Equal,
            fst: "x".to_string(),
            snd: Some("x".to_string()),
            thenc: Rc::new(FsExit::exit("y").into()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_ifz1() {
        let result = ifcz!(
            IfSort::Equal,
            Literal::new(1),
            cut!(Literal::new(1), XVar::covar("a", ty!("int")), ty!("int")),
            Exit::exit(XVar::var("x", ty!("int")), ty!("int"))
        )
        .focus(&mut Default::default());
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu(
                "x0",
                FsIfC {
                    sort: IfSort::Equal,
                    fst: "x0".to_string(),
                    snd: None,
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
        let result = ifcz!(
            IfSort::Equal,
            XVar::var("x", ty!("int")),
            Exit::exit(XVar::var("y", ty!("int")), ty!("int")),
            cut!(
                XVar::var("x", ty!("int")),
                XVar::covar("a", ty!("int")),
                ty!("int")
            )
        )
        .focus(&mut Default::default());
        let expected = FsIfC {
            sort: IfSort::Equal,
            fst: "x".to_string(),
            snd: None,
            thenc: Rc::new(FsExit::exit("y").into()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
