use printer::{
    tokens::{LANGLE, PIPE, RANGLE},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term},
        types::{Ty, Typed},
    },
    traits::{
        focus::{bind_many, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        uniquify::Uniquify,
        used_binders::UsedBinders,
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

impl From<Cut> for Statement {
    fn from(value: Cut) -> Self {
        Statement::Cut(value)
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

impl UsedBinders for Cut {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.producer.used_binders(used);
        self.consumer.used_binders(used);
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

impl Uniquify for Cut {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Cut {
        Cut {
            producer: self.producer.uniquify(seen_vars, used_vars),
            consumer: self.consumer.uniquify(seen_vars, used_vars),
            ..self
        }
    }
}

impl Focusing for Cut {
    type Target = crate::syntax_var::Statement;
    fn focus(self, state: &mut FocusingState) -> crate::syntax_var::Statement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            // N(⟨K(t_i) | c⟩) = bind(t_i)[λas.⟨K(as) | N(c)⟩]
            (Term::Xtor(constructor), consumer) => bind_many(
                constructor.args.into(),
                Box::new(|vars, state: &mut FocusingState| {
                    crate::syntax_var::statement::Cut {
                        ty: self.ty.focus(state),
                        producer: Rc::new(
                            crate::syntax_var::term::Xtor {
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
            // N(⟨p | D(t_i)⟩) = bind(t_i)[λas⟨D(as) | N(p)⟩]
            (producer, Term::Xtor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|args, state: &mut FocusingState| {
                    crate::syntax_var::statement::Cut {
                        ty: self.ty.focus(state),
                        producer: Rc::new(
                            crate::syntax_var::term::Xtor {
                                id: destructor.id,
                                args: args.into_iter().collect(),
                            }
                            .into(),
                        ),
                        consumer: Rc::new(producer.focus(state)),
                    }
                    .into()
                }),
                state,
            ),
            // N(⟨p | c⟩) = ⟨N(p) | N(c)⟩ OR ⟨N(c) | N(p)⟩
            (producer, consumer) => {
                if self.ty.is_codata(state.codata_types) {
                    crate::syntax_var::statement::Cut {
                        ty: self.ty.focus(state),
                        producer: Rc::new(consumer.focus(state)),
                        consumer: Rc::new(producer.focus(state)),
                    }
                    .into()
                } else {
                    crate::syntax_var::statement::Cut {
                        ty: self.ty.focus(state),
                        producer: Rc::new(producer.focus(state)),
                        consumer: Rc::new(consumer.focus(state)),
                    }
                    .into()
                }
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
        term::{Literal, XVar, Xtor},
        types::Ty,
    };
    use crate::syntax_var::Chirality;
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
            Ty::Decl("FunIntInt".to_owned()),
        );
        Cut::new(
            XVar::var("x", Ty::Decl("FunIntInt".to_owned())),
            ap,
            Ty::Decl("FunIntInt".to_owned()),
        )
    }
    fn example_dtor_var() -> crate::syntax_var::statement::Cut {
        let ap = crate::syntax_var::term::Xtor {
            id: "Ap".to_string(),
            args: vec!["y".to_string(), "a".to_string()],
        };
        crate::syntax_var::statement::Cut::new(
            crate::syntax_var::Ty::Decl("FunIntInt".to_owned()),
            ap,
            crate::syntax_var::term::XVar::var("x"),
        )
    }

    fn example_other() -> Cut {
        Cut::new(
            XVar::var("x", Ty::Int()),
            XVar::covar("a", Ty::Int()),
            Ty::Int(),
        )
    }
    fn example_other_var() -> crate::syntax_var::statement::Cut {
        crate::syntax_var::statement::Cut::new(
            crate::syntax_var::Ty::Int,
            crate::syntax_var::term::XVar::var("x"),
            crate::syntax_var::term::XVar::covar("a"),
        )
    }

    #[test]
    // this illustrates the problem
    fn transform_ctor() {
        let result = example_ctor().focus(&mut Default::default());
        let expected = crate::syntax_var::statement::Cut {
            producer: Rc::new(crate::syntax_var::term::Literal::new(1).into()),
            ty: crate::syntax_var::Ty::Int,
            consumer: Rc::new(
                crate::syntax_var::term::Mu {
                    chi: Chirality::Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        crate::syntax_var::statement::Cut {
                            producer: Rc::new(
                                crate::syntax_var::term::Xtor {
                                    id: "Nil".to_string(),
                                    args: vec![],
                                }
                                .into(),
                            ),
                            ty: crate::syntax_var::Ty::Decl("ListInt".to_owned()),
                            consumer: Rc::new(
                                crate::syntax_var::term::Mu {
                                    chi: Chirality::Cns,
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        crate::syntax_var::statement::Cut {
                                            producer: Rc::new(
                                                crate::syntax_var::term::Xtor {
                                                    id: "Cons".to_string(),
                                                    args: vec!["x0".to_string(), "x1".to_string()],
                                                }
                                                .into(),
                                            ),
                                            ty: crate::syntax_var::Ty::Decl("ListInt".to_owned()),
                                            consumer: Rc::new(
                                                crate::syntax_var::term::XVar::covar("a").into(),
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
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected);
    }

    #[test]
    fn transform_dtor() {
        let result = example_dtor().focus(&mut Default::default());
        let expected = example_dtor_var().into();
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = example_other().focus(&mut Default::default());
        let expected = example_other_var().into();
        assert_eq!(result, expected);
    }
}
