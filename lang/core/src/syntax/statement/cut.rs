use printer::{
    tokens::{LANGLE, PIPE, RANGLE},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term, Xtor},
        types::{Ty, Typed},
    },
    traits::{
        focus::{bind_many, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};
use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub producer: Rc<Term<Prd>>,
    pub ty: Ty,
    pub consumer: Rc<Term<Cns>>,
}

impl Cut {
    pub fn new<T: Into<Term<Prd>>, S: Into<Term<Cns>>>(prd: T, cns: S, ty: Ty) -> Self {
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

impl Print for Cut {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let Cut {
            producer,
            consumer,
            ty: _,
        } = self;
        alloc.text(LANGLE).append(
            producer
                .print(cfg, alloc)
                .append(alloc.space())
                .append(alloc.text(PIPE))
                .append(alloc.space())
                .append(consumer.print(cfg, alloc))
                .append(alloc.text(RANGLE)),
        )
    }
}

impl FreeV for Cut {
    fn free_vars(&self) -> HashSet<Var> {
        let Cut {
            producer,
            consumer,
            ty: _,
        } = self;
        let mut free_vars = producer.free_vars();
        free_vars.extend(consumer.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let Cut {
            producer,
            consumer,
            ty: _,
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
            (Term::Xtor(constructor), consumer) => {
                let ty = constructor.get_type();

                bind_many(
                    constructor.args.into(),
                    Box::new(|vars, state: &mut FocusingState| {
                        Cut {
                            producer: Rc::new(
                                Xtor {
                                    prdcns: Prd,
                                    id: constructor.id,
                                    args: vars.into_iter().collect(),
                                    ty: ty.clone(),
                                }
                                .into(),
                            ),
                            ty,
                            consumer: Rc::new(consumer.focus(state)),
                        }
                        .into()
                    }),
                    state,
                )
            }
            // N(⟨p | D(p_i; c_j)⟩) = bind(p_i)[λas.bind(c_j)[λbs.⟨N(p) | D(as; bs)⟩]]
            (producer, Term::Xtor(destructor)) => {
                let ty = destructor.get_type();
                bind_many(
                    destructor.args.into(),
                    Box::new(|args, state: &mut FocusingState| {
                        Cut {
                            producer: Rc::new(producer.focus(state)),
                            ty: ty.clone(),
                            consumer: Rc::new(
                                Xtor {
                                    prdcns: Cns,
                                    id: destructor.id,
                                    args: args.into_iter().collect(),
                                    ty,
                                }
                                .into(),
                            ),
                        }
                        .into()
                    }),
                    state,
                )
            }
            // N(⟨p | c⟩) = ⟨N(p) | N(c)⟩
            (producer, consumer) => {
                let ty = producer.get_type();

                Cut {
                    producer: Rc::new(producer.focus(state)),
                    ty,
                    consumer: Rc::new(consumer.focus(state)),
                }
                .into()
            }
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
        types::Ty,
    };
    use std::rc::Rc;

    fn example_ctor() -> Cut {
        let cons = Xtor::ctor(
            "Cons",
            vec![
                SubstitutionBinding::ProducerBinding(Literal { lit: 1 }.into()),
                SubstitutionBinding::ProducerBinding(
                    Xtor::ctor("Nil", vec![], Ty::Decl("ListInt".to_owned())).into(),
                ),
            ],
            Ty::Decl("ListInt".to_owned()),
        );
        Cut::new(
            cons,
            XVar::covar("a", Ty::Decl("ListInt".to_owned())),
            Ty::Decl("ListInt".to_owned()),
        )
    }

    fn example_dtor() -> Cut {
        let ap = Xtor::dtor(
            "Ap",
            vec![
                SubstitutionBinding::ProducerBinding(XVar::var("y", Ty::Int()).into()),
                SubstitutionBinding::ConsumerBinding(XVar::covar("a", Ty::Int()).into()),
            ],
            Ty::Decl("FUnIntnt".to_owned()),
        );
        Cut::new(
            XVar::var("x", Ty::Decl("FunIntInt".to_owned())),
            ap,
            Ty::Decl("FunIntInt".to_owned()),
        )
    }

    fn example_other() -> Cut {
        Cut::new(
            XVar::var("x", Ty::Int()),
            XVar::covar("a", Ty::Int()),
            Ty::Int(),
        )
    }

    #[test]
    // this illustrates the problem
    fn transform_ctor() {
        let result = example_ctor().focus(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal::new(1).into()),
            ty: Ty::Int(),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(
                                Xtor::ctor("Nil", vec![], Ty::Decl("ListInt".to_owned())).into(),
                            ),
                            ty: Ty::Decl("ListInt".to_owned()),
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
                                                            XVar::var("x0", Ty::Int()).into(),
                                                        ),
                                                        SubstitutionBinding::ProducerBinding(
                                                            XVar::var(
                                                                "x1",
                                                                Ty::Decl("ListInt".to_owned()),
                                                            )
                                                            .into(),
                                                        ),
                                                    ],
                                                    Ty::Decl("ListInt".to_owned()),
                                                )
                                                .into(),
                                            ),
                                            ty: Ty::Decl("ListInt".to_owned()),
                                            consumer: Rc::new(
                                                XVar::covar("a", Ty::Decl("ListInt".to_owned()))
                                                    .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                    ty: Ty::Decl("ListInt".to_owned()),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
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
