use super::{Cns, Mu, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        statement::Cut, stringify_and_join, substitution::Substitution, types::Ty, Covar, Name,
        Statement, Var,
    },
    traits::{
        focus::{bind_many, Bind, Continuation, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        typed::Typed,
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<T: PrdCns> {
    pub prdcns: T,
    pub id: Name,
    pub args: Substitution,
    pub ty: Ty,
}

impl Xtor<Prd> {
    /// Create a new constructor
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
impl<T: PrdCns> std::fmt::Display for Xtor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(f, "{}({})", self.id, args_joined)
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

impl<T: PrdCns> From<Xtor<T>> for Term<T> {
    fn from(value: Xtor<T>) -> Self {
        Term::Xtor(value)
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

impl Focusing for Xtor<Prd> {
    type Target = Term<Prd>;
    ///N(K(p_i; c_j)) = μa.bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | a⟩]]
    fn focus(self, st: &mut FocusingState) -> Self::Target {
        let new_covar = st.fresh_covar();
        let new_covar_clone = new_covar.clone();
        let ty = self.ty.clone();
        let new_statement = bind_many(
            self.args.into(),
            Box::new(|vars, _: &mut FocusingState| {
                Cut::new(
                    Term::Xtor(Xtor {
                        prdcns: self.prdcns,
                        id: self.id,
                        args: vars.into_iter().collect(),
                        ty: ty.clone(),
                    }),
                    ty,
                    Term::XVar(XVar {
                        prdcns: Cns,
                        var: new_covar,
                        ty: Ty::Int(),
                    }),
                )
                .into()
            }),
            st,
        );

        Mu {
            prdcns: Prd,
            variable: new_covar_clone,
            var_ty: self.ty,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Focusing for Xtor<Cns> {
    type Target = Term<Cns>;
    ///N(D(p_i; cj)) =  ~μx.bind(p_i)[λas.bind(c_j)[λbs.⟨x | D(as; bs)⟩]]
    fn focus(self, state: &mut FocusingState) -> Term<Cns> {
        let new_var = state.fresh_var();
        let new_var_clone = new_var.clone();
        let ty = self.ty.clone();
        let new_statement = bind_many(
            self.args.into(),
            Box::new(|args, _: &mut FocusingState| {
                Cut::new(
                    Term::XVar(XVar {
                        prdcns: Prd,
                        var: new_var,
                        ty: Ty::Int(),
                    }),
                    ty.clone(),
                    Term::Xtor(Xtor {
                        prdcns: Cns,
                        id: self.id,
                        args: args.into_iter().collect(),
                        ty,
                    }),
                )
                .into()
            }),
            state,
        );
        Mu {
            prdcns: Cns,
            variable: new_var_clone,
            var_ty: self.ty,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Bind for Xtor<Prd> {
    fn bind(self, k: Continuation, st: &mut FocusingState) -> Statement {
        let new_var = st.fresh_var();
        bind_many(
            self.args.into(),
            Box::new(|vars, state: &mut FocusingState| {
                Cut::new(
                    Term::Xtor(Xtor {
                        prdcns: Prd,
                        id: self.id,
                        args: vars.into_iter().collect(),
                        ty: self.ty.clone(),
                    }),
                    self.ty.clone(),
                    Term::Mu(Mu {
                        prdcns: Cns,
                        var_ty: self.ty,
                        variable: new_var.clone(),
                        statement: Rc::new(k(new_var, state)),
                    }),
                )
                .into()
            }),
            st,
        )
    }
}

impl Bind for Xtor<Cns> {
    ///bind(D(p_i; c_j))[k] = bind(p_i)[λas.bind(c_j)[λbs.⟨μa.k(a) | D(as; bs)⟩]]
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        let new_covar = state.fresh_covar();
        bind_many(
            self.args.into(),
            Box::new(|args, state: &mut FocusingState| {
                Cut::new(
                    Term::Mu(Mu {
                        prdcns: Prd,
                        variable: new_covar.clone(),
                        var_ty: self.ty.clone(),
                        statement: Rc::new(k(new_covar, state)),
                    }),
                    self.ty.clone(),
                    Term::Xtor(Xtor {
                        prdcns: Cns,
                        id: self.id,
                        args: args.into_iter().collect(),
                        ty: self.ty,
                    }),
                )
                .into()
            }),
            state,
        )
    }
}

#[cfg(test)]
mod xtor_tests {
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
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("x", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("xs", Ty::Decl("ListInt".to_owned())).into(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("a", Ty::Decl("ListInt".to_owned())).into(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
            ],
            Ty::Decl("ListInt".to_owned()),
        )
    }

    fn example_destructor() -> Xtor<Cns> {
        Xtor::dtor(
            "Hd",
            vec![
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("x", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("a", Ty::Decl("StreamInt".to_owned())).into(),
                    ty: Ty::Decl("StreamInt".to_owned()),
                },
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
        let result = format!("{}", example_constructor());
        let expected = "Cons(x, xs, 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_dest() {
        let result = format!("{}", example_destructor());
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
        let expected = HashSet::from(["a".to_owned()]);
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
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("y", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("xs", Ty::Int()).into(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("b", Ty::Int()).into(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
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
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("y", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("b", Ty::Int()).into(),
                    ty: Ty::Decl("StreamInt".to_owned()),
                },
            ],
            Ty::Decl("StreamInt".to_owned()),
        );
        assert_eq!(result, expected)
    }
}
