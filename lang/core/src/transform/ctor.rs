use crate::{
    naming_transformation::{bind_many, Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Constructor, Covariable, Cut, Mu, MuTilde, Producer, Statement, Variable},
};
use std::rc::Rc;

impl NamingTransformation for Constructor {
    type Target = Producer;

    ///N(K(p_i; c_j)) = μa.bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | a⟩]]
    fn transform(self, state: &mut TransformState) -> Producer {
        let new_covar = state.fresh_covar();
        let new_covar_clone = new_covar.clone();
        let new_statement = bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, _: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Constructor {
                                    id: self.id,
                                    producers: vars
                                        .into_iter()
                                        .map(|var| Variable { var }.into())
                                        .collect(),
                                    consumers: covars
                                        .into_iter()
                                        .map(|covar| Covariable { covar }.into())
                                        .collect(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(Covariable { covar: new_covar }.into()),
                        }
                        .into()
                    }),
                    state,
                )
            }),
            state,
        );
        Mu {
            covariable: new_covar_clone,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Bind for Constructor {
    ///bind(K(p_i; c_j))[k] = bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | ~μx.k(x)⟩]]
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_var = state.fresh_var();
        bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, state: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Constructor {
                                    id: self.id,
                                    producers: vars
                                        .into_iter()
                                        .map(|var| Variable { var }.into())
                                        .collect(),
                                    consumers: covars
                                        .into_iter()
                                        .map(|covar| Covariable { covar }.into())
                                        .collect(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                MuTilde {
                                    variable: new_var.clone(),
                                    statement: Rc::new(k(new_var, state)),
                                }
                                .into(),
                            ),
                        }
                        .into()
                    }),
                    state,
                )
            }),
            state,
        )
    }
}
