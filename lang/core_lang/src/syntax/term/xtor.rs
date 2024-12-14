use printer::{theme::ThemeExt, DocAllocator, Print};

use super::{Cns, FsTerm, Prd, PrdCns, Term};
use crate::{
    syntax::{statement::FsCut, term::FsMu, Covar, FsStatement, Name, Substitution, Ty, Var},
    traits::*,
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<T: PrdCns> {
    pub prdcns: T,
    pub id: Name,
    pub args: Substitution,
    pub ty: Ty,
}

impl Xtor<Prd> {
    /// Create a new constructor
    #[must_use]
    pub fn ctor(name: &str, subst: Substitution, ty: Ty) -> Self {
        Xtor {
            prdcns: Prd,
            id: name.to_string(),
            args: subst,
            ty,
        }
    }
}
impl Xtor<Cns> {
    /// Create a new destructor
    #[must_use]
    pub fn dtor(name: &str, subst: Substitution, ty: Ty) -> Self {
        Xtor {
            prdcns: Cns,
            id: name.to_string(),
            args: subst,
            ty,
        }
    }
}

impl<T: PrdCns> Typed for Xtor<T> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T: PrdCns> Print for Xtor<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc.ctor(&self.id).append(args)
        } else {
            alloc.dtor(&self.id).append(args)
        }
    }
}

impl<T: PrdCns> From<Xtor<T>> for Term<T> {
    fn from(value: Xtor<T>) -> Self {
        Term::Xtor(value)
    }
}

impl<T: PrdCns> FreeV for Xtor<T> {
    fn free_vars(&self) -> HashSet<Var> {
        self.args.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.args.free_covars()
    }
}

impl<T: PrdCns> UsedBinders for Xtor<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.used_binders(used);
    }
}

impl<T: PrdCns> Subst for Xtor<T> {
    type Target = Xtor<T>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Xtor {
            prdcns: self.prdcns.clone(),
            id: self.id.clone(),
            args: self.args.subst_sim(prod_subst, cons_subst),
            ty: self.ty.clone(),
        }
    }
}

impl<T: PrdCns> Uniquify for Xtor<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Xtor<T> {
        Xtor {
            args: self.args.uniquify(seen_vars, used_vars),
            ..self
        }
    }
}

impl<T: PrdCns> Focusing for Xtor<T> {
    type Target = FsTerm<T>;
    fn focus(self, _: &mut FocusingState) -> Self::Target {
        panic!("Constructors and destructors should always be focused in cuts directly");
    }
}

impl Bind for Xtor<Prd> {
    ///bind(C(t_i))[k] = bind(t_i)[λas.⟨C(as) | ~μx.k(x)⟩]
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        let new_var = state.fresh_var();
        bind_many(
            self.args.into(),
            Box::new(|vars, state: &mut FocusingState| {
                FsCut::new(
                    FsTerm::Xtor(FsXtor {
                        prdcns: self.prdcns,
                        id: self.id,
                        args: vars.into_iter().collect(),
                    }),
                    FsMu::tilde_mu(&new_var.clone(), k(new_var, state)),
                    self.ty,
                )
                .into()
            }),
            state,
        )
    }
}
impl Bind for Xtor<Cns> {
    ///bind(D(t_i))[k] = bind(t_i)[λas.⟨μa.k(a) | D(as)⟩]
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        let new_covar = state.fresh_covar();
        bind_many(
            self.args.into(),
            Box::new(|vars, state: &mut FocusingState| {
                FsCut::new(
                    FsMu::mu(&new_covar.clone(), k(new_covar, state)),
                    FsTerm::Xtor(FsXtor {
                        prdcns: self.prdcns,
                        id: self.id,
                        args: vars.into_iter().collect(),
                    }),
                    self.ty,
                )
                .into()
            }),
            state,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsXtor<T: PrdCns> {
    pub prdcns: T,
    pub id: Name,
    pub args: Vec<Var>,
}

impl FsXtor<Prd> {
    /// Create a new constructor
    #[must_use]
    pub fn ctor(name: &str, args: Vec<Var>) -> Self {
        FsXtor {
            prdcns: Prd,
            id: name.to_string(),
            args,
        }
    }
}
impl FsXtor<Cns> {
    /// Create a new destructor
    #[must_use]
    pub fn dtor(name: &str, args: Vec<Var>) -> Self {
        FsXtor {
            prdcns: Cns,
            id: name.to_string(),
            args,
        }
    }
}

impl<T: PrdCns> Print for FsXtor<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc.ctor(&self.id).append(args)
        } else {
            alloc.dtor(&self.id).append(args)
        }
    }
}

impl<T: PrdCns> From<FsXtor<T>> for FsTerm<T> {
    fn from(value: FsXtor<T>) -> Self {
        FsTerm::Xtor(value)
    }
}

impl<T: PrdCns> SubstVar for FsXtor<T> {
    type Target = FsXtor<T>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        FsXtor {
            prdcns: self.prdcns,
            id: self.id,
            args: self.args.subst_sim(subst),
        }
    }
}

#[cfg(test)]
mod xtor_tests {
    use super::{FreeV, Subst, Xtor};
    use crate::syntax::{
        substitution::SubstitutionBinding,
        term::{Prd, XVar},
        types::Ty,
    };
    use printer::Print;
    use std::collections::HashSet;

    fn example() -> Xtor<Prd> {
        Xtor::ctor(
            "Cons",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("x", Ty::Int).into()),
                SubstitutionBinding::ProducerBinding(
                    XVar::var("xs", Ty::Decl("ListInt".to_string())).into(),
                ),
            ],
            Ty::Decl("ListInt".to_string()),
        )
    }

    #[test]
    fn display_const() {
        assert_eq!(example().print_to_string(None), "Cons(x, xs)")
    }

    #[test]
    fn free_vars_const() {
        assert_eq!(
            example().free_vars(),
            HashSet::from(["x".to_string(), "xs".to_string()])
        )
    }

    #[test]
    fn free_covars_const() {
        assert!(example().free_covars().is_empty())
    }

    #[test]
    fn subst_const() {
        let result = example().subst_sim(
            &vec![(XVar::var("y", Ty::Int).into(), "x".to_string())],
            &vec![(XVar::covar("b", Ty::Int).into(), "a".to_string())],
        );
        let expected = Xtor::ctor(
            "Cons",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("y", Ty::Int).into()),
                SubstitutionBinding::ProducerBinding(
                    XVar::var("xs", Ty::Decl("ListInt".to_string())).into(),
                ),
            ],
            Ty::Decl("ListInt".to_string()),
        );
        assert_eq!(result, expected)
    }
}
