use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{
        Constructor, Covar, Covariable, Cut, Mu, MuTilde, Name, Producer, Statement, Var, Variable,
    },
};
use std::rc::Rc;

impl NamingTransformation for Constructor {
    type Target = Producer;

    ///N(K(p_i; c_j)) = μa.bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | a⟩]]
    fn transform(self, state: &mut TransformState) -> Producer {
        let new_covar = state.fresh_covar();
        let new_statement =
            Bind::bind_many(self.producers, |vars: Vec<Var>, _: &mut TransformState| {
                Bind::bind_many(
                    self.consumers,
                    |covars: Vec<Covar>, _: &mut TransformState| {
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
                                Covariable {
                                    covar: new_covar.clone(),
                                }
                                .into(),
                            ),
                        }
                        .into()
                    },
                )
            });
        Mu {
            covariable: new_covar,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Bind for Constructor {
    ///bind(K (pi ; c j )) [k] =  bind(p i ) [λas.bind(c j ) [λbs.⟨K (as; bs) | μx  ̃ .k (x)⟩]]
    fn bind<K>(self, k: K, state: &mut TransformState) -> Statement
    where
        K: FnOnce(Name, &mut TransformState) -> Statement,
    {
        let new_var = state.fresh_var();
        let cont = |vars: Vec<Var>, _: &mut TransformState| {
            Bind::bind_many(
                self.consumers,
                |covars: Vec<Covar>, state: &mut TransformState| {
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
                },
            )
        };
        Bind::bind_many(self.producers, cont)
    }
}
