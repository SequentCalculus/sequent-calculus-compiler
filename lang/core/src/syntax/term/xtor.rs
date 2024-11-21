use printer::{theme::ThemeExt, DocAllocator, Print};

use super::{Cns, Prd, PrdCns, Term};
use crate::{
    syntax::{
        substitution::Substitution,
        types::{Ty, Typed},
        Covar, Name, Var,
    },
    traits::{
        focus::{bind_many, Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        uniquify::Uniquify,
        used_binders::UsedBinders,
    },
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
    type Target = crate::syntax_var::Term;
    fn focus(self, _: &mut FocusingState) -> Self::Target {
        panic!("Constructors and destructors should always be focused in cuts directly");
    }
}

impl<T: PrdCns> Bind for Xtor<T> {
    ///bind(C(t_i))[k] = bind(t_i)[λas.⟨C(as) | ~μx.k(x)⟩]
    ///AND bind(D(t_i))[k] = bind(t_i)[λas.⟨D(as) | ~μx.k(x)⟩]
    fn bind(self, k: Continuation, state: &mut FocusingState) -> crate::syntax_var::Statement {
        let new_var = state.fresh_var();
        bind_many(
            self.args.into(),
            Box::new(|vars, state: &mut FocusingState| {
                crate::syntax_var::statement::Cut::new(
                    self.ty.focus(state),
                    crate::syntax_var::term::Term::Xtor(crate::syntax_var::term::Xtor {
                        id: self.id,
                        args: vars.into_iter().collect(),
                    }),
                    crate::syntax_var::term::Mu::tilde_mu(&new_var.clone(), k(new_var, state)),
                )
                .into()
            }),
            state,
        )
    }
}

#[cfg(test)]
mod xtor_tests {
    use printer::Print;

    use super::{FreeV, Subst, Term, Xtor};
    use crate::syntax::{
        substitution::SubstitutionBinding,
        term::{Cns, Prd, XVar},
        types::Ty,
        Covar, Var,
    };
    use std::collections::HashSet;

    fn example_constructor() -> Xtor<Prd> {
        Xtor::ctor(
            "Cons",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("x", Ty::Int()).into()),
                SubstitutionBinding::ProducerBinding(
                    XVar::var("xs", Ty::Decl("ListInt".to_owned())).into(),
                ),
            ],
            Ty::Decl("ListInt".to_owned()),
        )
    }

    fn example_destructor() -> Xtor<Cns> {
        Xtor::dtor(
            "Hd",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("x", Ty::Int()).into()),
                SubstitutionBinding::ConsumerBinding(
                    XVar::covar("a", Ty::Decl("ListInt".to_owned())).into(),
                ),
            ],
            Ty::Decl("StreamInt".to_owned()),
        )
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(XVar::var("y", Ty::Int()).into(), "x".to_owned())]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(XVar::covar("b", Ty::Int()).into(), "a".to_owned())]
    }
    #[test]
    fn display_const() {
        let result = example_constructor().print_to_string(None);
        let expected = "Cons(x, xs)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_dest() {
        let result = example_destructor().print_to_string(None);
        let expected = "Hd(x, 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_const() {
        let result = example_constructor().free_vars();
        let expected = HashSet::from(["x".to_owned(), "xs".to_owned()]);
        assert_eq!(result, expected)
    }
    #[test]
    fn free_vars_dest() {
        let result = example_destructor().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_const() {
        let result = example_constructor().free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }
    #[test]
    fn free_covars_dest() {
        let result = example_destructor().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_const() {
        let result = example_constructor().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Xtor::ctor(
            "Cons",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("y", Ty::Int()).into()),
                SubstitutionBinding::ProducerBinding(
                    XVar::var("xs", Ty::Decl("ListInt".to_owned())).into(),
                ),
            ],
            Ty::Decl("ListInt".to_owned()),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_dest() {
        let result = example_destructor().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Xtor::dtor(
            "Hd",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("y", Ty::Int()).into()),
                SubstitutionBinding::ConsumerBinding(XVar::covar("b", Ty::Int()).into()),
            ],
            Ty::Decl("StreamInt".to_owned()),
        );
        assert_eq!(result, expected)
    }
}
