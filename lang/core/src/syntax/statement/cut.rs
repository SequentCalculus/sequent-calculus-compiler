use super::{Covar, Statement, Var};
use crate::{
    syntax::term::{Cns, Prd, Term, Xtor},
    traits::{
        focus::{bind_many, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};
use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub producer: Rc<Term<Prd>>,
    pub consumer: Rc<Term<Cns>>,
}

impl Cut {
    pub fn mk<T: Into<Term<Prd>>, S: Into<Term<Cns>>>(prd: T, cns: S) -> Self {
        Cut {
            producer: Rc::new(prd.into()),
            consumer: Rc::new(cns.into()),
        }
    }
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Cut { producer, consumer } = self;
        write!(f, "<{} | {}>", producer, consumer)
    }
}

impl FreeV for Cut {
    fn free_vars(&self) -> HashSet<Var> {
        let Cut { producer, consumer } = self;
        let mut free_vars = producer.free_vars();
        free_vars.extend(consumer.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let Cut { producer, consumer } = self;
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
                            }
                            .into(),
                        ),
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
                        consumer: Rc::new(
                            Xtor {
                                prdcns: Cns,
                                id: destructor.id,
                                args: args.into_iter().collect(),
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
        term::{Cns, Literal, Mu, XVar, Xtor},
    };
    use std::rc::Rc;

    fn example_ctor() -> Cut {
        let cons = Xtor::ctor(
            "Cons",
            vec![
                SubstitutionBinding::ProducerBinding(Literal { lit: 1 }.into()),
                SubstitutionBinding::ProducerBinding(Xtor::ctor("Nil", vec![]).into()),
                SubstitutionBinding::ConsumerBinding(XVar::covar("a").into()),
            ],
        );
        Cut::mk(cons, XVar::covar("a"))
    }

    fn example_dtor() -> Cut {
        let ap = Xtor::dtor(
            "Ap",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("y").into()),
                SubstitutionBinding::ConsumerBinding(XVar::covar("a").into()),
            ],
        );
        Cut::mk(XVar::var("x"), ap)
    }

    fn example_other() -> Cut {
        Cut::mk(XVar::var("x"), XVar::covar("a"))
    }

    #[test]
    // this illustrates the problem
    fn transform_ctor() {
        let result = example_ctor().focus(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal::new(1).into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(Xtor::ctor("Nil", vec![]).into()),
                            consumer: Rc::new(
                                Mu {
                                    prdcns: Cns,
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        Cut {
                                            producer: Rc::new(
                                                Xtor::ctor(
                                                    "Cons",
                                                    vec![
                                                        SubstitutionBinding::ProducerBinding(
                                                            XVar::var("x0").into(),
                                                        ),
                                                        SubstitutionBinding::ProducerBinding(
                                                            XVar::var("x1").into(),
                                                        ),
                                                        SubstitutionBinding::ConsumerBinding(
                                                            XVar::covar("a").into(),
                                                        ),
                                                    ],
                                                )
                                                .into(),
                                            ),
                                            consumer: Rc::new(XVar::covar("a").into()),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
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
