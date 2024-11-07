use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term, Xtor},
        types::Ty,
    },
    traits::{
        focus::{bind_many, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        typed::Typed,
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub producer: Rc<Term<Prd>>,
    pub ty: Ty,
    pub consumer: Rc<Term<Cns>>,
}

impl Cut {
    pub fn new<T: Into<Term<Prd>>, S: Into<Term<Cns>>>(prd: T, ty: Ty, cns: S) -> Self {
        Cut {
            producer: Rc::new(prd.into()),
            ty,
            consumer: Rc::new(cns.into()),
        }
    }
}

impl Typed for Cut {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Cut {
            producer,
            ty,
            consumer,
        } = self;
        write!(f, "<{} | {} | {}>", producer, ty, consumer)
    }
}

impl FreeV for Cut {
    fn free_vars(&self) -> HashSet<Var> {
        let Cut {
            producer,
            ty: _,
            consumer,
        } = self;
        let mut free_vars = producer.free_vars();
        free_vars.extend(consumer.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let Cut {
            producer,
            ty: _,
            consumer,
        } = self;
        let mut free_covars = producer.free_covars();
        free_covars.extend(consumer.free_covars());
        free_covars
    }
}

impl From<Cut> for Statement {
    fn from(value: Cut) -> Self {
        Statement::Cut(value)
    }
}

impl Subst for Cut {
    type Target = Cut;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Cut {
            producer: self.producer.subst_sim(prod_subst, cons_subst),
            ty: self.ty.clone(),
            consumer: self.consumer.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Focusing for Cut {
    type Target = Statement;
    fn focus(self, state: &mut FocusingState) -> Statement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            // N(⟨K(p_i; c_j) | c⟩) = bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | N(c)⟩]]
            (Term::Xtor(constructor), consumer) => bind_many(
                constructor.args.into(),
                Box::new(|vars, state: &mut FocusingState| {
                    Cut {
                        producer: Rc::new(
                            Xtor {
                                prdcns: Prd,
                                id: constructor.id,
                                args: vars.into_iter().collect(),
                                ty: constructor.ty.clone(),
                            }
                            .into(),
                        ),
                        ty: constructor.ty,
                        consumer: Rc::new(consumer.focus(state)),
                    }
                    .into()
                }),
                state,
            ),
            // N(⟨p | D(p_i; c_j)⟩) = bind(p_i)[λas.bind(c_j)[λbs.⟨N(p) | D(as; bs)⟩]]
            (producer, Term::Xtor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|args, state: &mut FocusingState| {
                    Cut {
                        producer: Rc::new(producer.focus(state)),
                        ty: self.ty,
                        consumer: Rc::new(
                            Xtor {
                                prdcns: Cns,
                                id: destructor.id,
                                args: args.into_iter().collect(),
                                ty: destructor.ty,
                            }
                            .into(),
                        ),
                    }
                    .into()
                }),
                state,
            ),
            // N(⟨p | c⟩) = ⟨N(p) | N(c)⟩
            (producer, consumer) => Cut {
                producer: Rc::new(producer.focus(state)),
                ty: self.ty,
                consumer: Rc::new(consumer.focus(state)),
            }
            .into(),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        statement::Cut,
        substitution::SubstitutionBinding,
        term::{Literal, Mu, XVar, Xtor},
        types::Ty,
    };

    fn example_ctor() -> Cut {
        let cons = Xtor::ctor(
            "Cons",
            vec![
                SubstitutionBinding::ProducerBinding {
                    prd: Literal { lit: 1 }.into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ProducerBinding {
                    prd: Xtor::ctor("Nil", vec![], Ty::Decl("ListInt".to_owned())).into(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("a", Ty::Decl("ListInt".to_owned())).into(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
            ],
            Ty::Decl("ListInt".to_owned()),
        );
        Cut::new(cons, Ty::Int(), XVar::covar("a", Ty::Int()))
    }

    fn example_dtor() -> Cut {
        let ap = Xtor::dtor(
            "Ap",
            vec![
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("y", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("a", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
            ],
            Ty::Decl("FunIntInt".to_owned()),
        );
        Cut::new(
            XVar::var("x", Ty::Decl("FunIntInt".to_owned())),
            Ty::Decl("FunIntInt".to_owned()),
            ap,
        )
    }

    fn example_other() -> Cut {
        Cut::new(
            XVar::var("x", Ty::Int()),
            Ty::Int(),
            XVar::covar("a", Ty::Int()),
        )
    }

    #[test]
    fn transform_ctor() {
        let result = example_ctor().focus(&mut Default::default());
        let expected = Cut::new(
            Literal::new(1),
            Ty::Int(),
            Mu::tilde_mu(
                "x0",
                Ty::Int(),
                Cut::new(
                    Xtor::ctor("Nil", vec![], Ty::Decl("ListInt".to_owned())),
                    Ty::Decl("ListInt".to_owned()),
                    Mu::tilde_mu(
                        "x1",
                        Ty::Decl("ListInt".to_owned()),
                        Cut::new(
                            Xtor::ctor(
                                "Cons",
                                vec![
                                    SubstitutionBinding::ProducerBinding {
                                        prd: XVar::var("x0", Ty::Int()).into(),
                                        ty: Ty::Int(),
                                    },
                                    SubstitutionBinding::ProducerBinding {
                                        prd: XVar::var("x1", Ty::Int()).into(),
                                        ty: Ty::Decl("ListInt".to_owned()),
                                    },
                                    SubstitutionBinding::ConsumerBinding {
                                        cns: XVar::covar("a", Ty::Int()).into(),
                                        ty: Ty::Decl("ListInt".to_owned()),
                                    },
                                ],
                                Ty::Decl("ListInt".to_owned()),
                            ),
                            Ty::Decl("ListInt".to_owned()),
                            XVar::covar("a", Ty::Int()),
                        ),
                    ),
                ),
            ),
        )
        .into();

        assert_eq!(result, expected);
    }

    #[test]
    fn transform_dtor() {
        let result = example_dtor().focus(&mut Default::default());
        let expected = example_dtor().into();
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = example_other().focus(&mut Default::default());
        let expected = example_other().into();
        assert_eq!(result, expected);
    }
}
