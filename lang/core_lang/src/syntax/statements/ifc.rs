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

pub type FsIfC = IfC<Ident, FsStatement>;

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
        prod_subst: &[(Ident, Term<Prd>)],
        cons_subst: &[(Ident, Term<Cns>)],
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
    fn subst_sim(mut self, subst: &[(Ident, Ident)]) -> FsIfC {
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
    fn uniquify(mut self, seen_vars: &mut HashSet<Ident>, used_vars: &mut HashSet<Ident>) -> IfC {
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
    fn focus(self, used_vars: &mut HashSet<Ident>) -> FsStatement {
        Rc::unwrap_or_clone(self.fst).bind(
            Box::new(
                move |binding_fst: ContextBinding, used_vars: &mut HashSet<Ident>| match self.snd {
                    None => FsIfC {
                        sort: self.sort,
                        fst: binding_fst.var,
                        snd: None,
                        thenc: self.thenc.focus(used_vars),
                        elsec: self.elsec.focus(used_vars),
                    }
                    .into(),
                    Some(snd) => Rc::unwrap_or_clone(snd).bind(
                        Box::new(move |binding_snd, used_vars: &mut HashSet<Ident>| {
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
    use crate::traits::*;
    use core_macros::{covar, cut, exit, fs_cut, fs_exit, fs_ife, fs_mutilde, id, ife, lit, var};
    extern crate self as core_lang;

    #[test]
    fn transform_ife1() {
        let result = ife!(
            lit!(2),
            lit!(1),
            cut!(lit!(1), covar!(id!("a"))),
            exit!(var!(id!("x")))
        )
        .focus(&mut Default::default());

        let expected = fs_cut!(
            lit!(2),
            fs_mutilde!(
                id!("x", 1),
                fs_cut!(
                    lit!(1),
                    fs_mutilde!(
                        id!("x", 2),
                        fs_ife!(
                            id!("x", 1),
                            id!("x", 2),
                            fs_cut!(lit!(1), covar!(id!("a"))),
                            fs_exit!(id!("x"))
                        )
                    )
                )
            )
        )
        .into();

        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ife2() {
        let result = ife!(
            var!(id!("x")),
            var!(id!("x")),
            exit!(var!(id!("y"))),
            cut!(var!(id!("x")), covar!(id!("a")))
        )
        .focus(&mut Default::default());
        let expected = fs_ife!(
            id!("x"),
            id!("x"),
            fs_exit!(id!("y")),
            fs_cut!(var!(id!("x")), covar!(id!("a")))
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_ifz1() {
        let result = ife!(
            lit!(1),
            cut!(lit!(1), covar!(id!("a"))),
            exit!(var!(id!("x")))
        )
        .focus(&mut Default::default());
        let expected = fs_cut!(
            lit!(1),
            fs_mutilde!(
                id!("x", 1),
                fs_ife!(
                    id!("x", 1),
                    fs_cut!(lit!(1), covar!(id!("a"))),
                    fs_exit!(id!("x"))
                )
            )
        )
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ifz2() {
        let result = ife!(
            var!(id!("x")),
            exit!(var!(id!("y"))),
            cut!(var!(id!("x")), covar!(id!("a")))
        )
        .focus(&mut Default::default());
        let expected = fs_ife!(
            id!("x"),
            fs_exit!(id!("y")),
            fs_cut!(var!(id!("x")), covar!(id!("a")))
        )
        .into();
        assert_eq!(result, expected)
    }
}
