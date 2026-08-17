//! This module defines the conditionals comparing two integers in Core.

use printer::tokens::{ELSE, EQQ, GT, GTE, IF, LT, LTE, NEQ, ZERO};
use printer::*;

use crate::mono::constraints::{ConstraintCollector, FlowConstraintSet};
use crate::mono::errors::MonoError;
use crate::mono::specialize::{Specialize, SpecializeContext};
use crate::splitting::labeling::{DeclSignatures, LabelAndUnify, SplitState};
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::{LocatedTypeError, TypeError};
use crate::{bail, syntax::*};

use std::collections::BTreeSet;
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
/// (if `P` and `C` is instantiated with [`Identifier`] and [`FsStatement`]).
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

pub type FsIfC = IfC<Identifier, FsStatement>;

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
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
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
    fn subst_sim(mut self, subst: &[(ID, Identifier)]) -> FsIfC {
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
    fn uniquify(mut self, max_id: &mut ID) -> IfC {
        self.fst = self.fst.uniquify(max_id);
        self.snd = self.snd.uniquify(max_id);
        self.thenc = self.thenc.uniquify(max_id);
        self.elsec = self.elsec.uniquify(max_id);
        self
    }
}

impl Focusing for IfC {
    type Target = FsStatement;
    // focus(ifc(p_1, p_2, s_1, s_2)) = bind(p_1)[λa1.bind(p_1)[λa2.ifc(a_1, a_2, focus(s_1), focus(s_2))]] OR
    // focus(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, focus(s_1), focus(s_2))]
    fn focus(self, max_id: &mut ID) -> FsStatement {
        Rc::unwrap_or_clone(self.fst).bind(
            Box::new(
                move |binding_fst: ContextBinding, max_id: &mut ID| match self.snd {
                    None => FsIfC {
                        sort: self.sort,
                        fst: binding_fst.var,
                        snd: None,
                        thenc: self.thenc.focus(max_id),
                        elsec: self.elsec.focus(max_id),
                    }
                    .into(),
                    Some(snd) => Rc::unwrap_or_clone(snd).bind(
                        Box::new(move |binding_snd, max_id: &mut ID| {
                            FsIfC {
                                sort: self.sort,
                                fst: binding_fst.var,
                                snd: Some(binding_snd.var),
                                thenc: self.thenc.focus(max_id),
                                elsec: self.elsec.focus(max_id),
                            }
                            .into()
                        }),
                        max_id,
                    ),
                },
            ),
            max_id,
        )
    }
}

impl ConstraintCollector for IfC {
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        // collect constraints from the first term, then the second term if it exists, then the then-branch and else-branch
        let mut constraints = self.fst.collect_constraints(env)?;
        if let Some(ref snd) = self.snd {
            constraints.extend(snd.collect_constraints(env)?);
        }
        constraints.extend(self.thenc.collect_constraints(env)?);
        constraints.extend(self.elsec.collect_constraints(env)?);
        Ok(constraints)
    }
}

impl Specialize for IfC {
    fn specialize(&self, context: &SpecializeContext) -> Self {
        IfC {
            sort: self.sort,
            fst: self.fst.specialize(context),
            snd: self.snd.specialize(context),
            thenc: self.thenc.specialize(context),
            elsec: self.elsec.specialize(context),
        }
    }
}

impl Checked for IfC {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        if let Ty::I64 = self.fst.get_type() {
        } else {
            bail!(TypeError::TypeMismatch {
                expected: Ty::I64.print_to_string(None),
                got: self.fst.get_type().print_to_string(None),
                msg: Some("Operands in if condition must be i64".to_string()),
            });
        }

        // check that the second term of the comparison has type i64 if it exists
        if let Some(ref snd) = self.snd {
            if let Ty::I64 = snd.get_type() {
            } else {
                bail!(TypeError::TypeMismatch {
                    expected: Ty::I64.print_to_string(None),
                    got: snd.get_type().print_to_string(None),
                    msg: Some("Operands in if condition must be i64".to_string()),
                });
            }
        }

        // check well-formedness of the terms
        self.fst.check(type_params, context, env)?;
        if let Some(ref snd) = self.snd {
            snd.check(type_params, context, env)?;
        }
        self.thenc.check(type_params, context, env)?;
        self.elsec.check(type_params, context, env)?;

        Ok(())
    }
}

impl LabelAndUnify for IfC {
    type Target = IfC;
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self::Target {
        IfC {
            sort: self.sort,
            fst: self.fst.label_and_unify(state, sigs, scope),
            snd: self.snd.label_and_unify(state, sigs, scope),
            thenc: self.thenc.label_and_unify(state, sigs, scope),
            elsec: self.elsec.label_and_unify(state, sigs, scope),
        }
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
            cut!(lit!(1), covar!(id!("a", 1))),
            exit!(var!(id!("x", 2)))
        )
        .focus(&mut 2);

        let expected = fs_cut!(
            lit!(2),
            fs_mutilde!(
                id!("x", 3),
                fs_cut!(
                    lit!(1),
                    fs_mutilde!(
                        id!("x", 4),
                        fs_ife!(
                            id!("x", 3),
                            id!("x", 4),
                            fs_cut!(lit!(1), covar!(id!("a", 1))),
                            fs_exit!(id!("x", 2))
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
            var!(id!("x", 1)),
            var!(id!("x", 1)),
            exit!(var!(id!("y", 2))),
            cut!(var!(id!("x", 1)), covar!(id!("a", 3)))
        )
        .focus(&mut 3);
        let expected = fs_ife!(
            id!("x", 1),
            id!("x", 1),
            fs_exit!(id!("y", 2)),
            fs_cut!(var!(id!("x", 1)), covar!(id!("a", 3)))
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

#[cfg(test)]
mod check_tests {

    use crate::{
        syntax::{Statement, TypingContext},
        typing::{check::Checked, env::GlobalEnv},
    };
    extern crate self as core_lang;
    use core_macros::{ctor, ctor_sig, data, exit, id, ife, lit, ty};

    #[test]
    fn ifc_check_ok_binary() {
        let stmt: Statement = ife!(
            lit!(1),
            lit!(2),
            exit!(lit!(1), ty!("int")),
            exit!(lit!(2), ty!("int"))
        )
        .into();
        assert!(
            stmt.check(&[], &TypingContext::default(), &GlobalEnv::default())
                .is_ok()
        );
    }

    #[test]
    fn ifc_check_branch_type_mismatch() {
        let stmt: Statement = ife!(
            lit!(1),
            lit!(2),
            exit!(lit!(1), ty!(id!("List"))),
            exit!(lit!(2), ty!("int"))
        )
        .into();
        assert!(
            stmt.check(&[], &TypingContext::default(), &GlobalEnv::default())
                .is_err()
        );
    }

    #[test]
    fn ifc_check_fst_not_i64() {
        let fst = ctor!(id!("Nil"), [], [], ty!(id!("List")));
        let stmt: Statement = ife!(
            fst,
            lit!(1),
            exit!(lit!(0), ty!("int")),
            exit!(lit!(0), ty!("int"))
        )
        .into();
        assert!(
            stmt.check(&[], &TypingContext::default(), &GlobalEnv::default())
                .is_err()
        );
    }

    #[test]
    fn ifc_check_snd_not_i64() {
        let list = data!(id!("List"), [ctor_sig!(id!("Nil"), [], [])], []);
        let snd = ctor!(id!("Nil"), [], [], ty!(id!("List")));
        let stmt: Statement = ife!(
            lit!(1),
            snd,
            exit!(lit!(0), ty!("int")),
            exit!(lit!(0), ty!("int"))
        )
        .into();
        assert!(
            stmt.check(
                &[],
                &TypingContext::default(),
                &GlobalEnv::new(&vec![list], &vec![], &vec![])
            )
            .is_err()
        );
    }
}
