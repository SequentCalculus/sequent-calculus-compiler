use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{
        Consumer, Covar, Covariable, Cut, Destructor, Mu, MuTilde, Name, Statement, Var, Variable,
    },
};
use std::rc::Rc;

impl NamingTransformation for Destructor {
    type Target = Consumer;
    ///N(D(p_i; cj)) =  ~μx.bind(p_i)[λas.bind(c_j)[λbs.⟨x | D(as; bs)⟩]]
    fn transform(self, state: &mut TransformState) -> Consumer {
        let new_var = state.fresh_var();
        let new_statement =
            Bind::bind_many(self.producers, |vars: Vec<Var>, _: &mut TransformState| {
                Bind::bind_many(
                    self.consumers,
                    |covars: Vec<Covar>, _: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Variable {
                                    var: new_var.clone(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                Destructor {
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
                        }
                        .into()
                    },
                )
            });
        MuTilde {
            variable: new_var,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Bind for Destructor {
    ///bind(D(p_i; c_j))[k] = bind(p_i)[λas.bind(c_j)[λbs.⟨μa.k(a) | D(as; bs)⟩]]
    fn bind<K>(self, k: K, state: &mut TransformState) -> Statement
    where
        K: FnOnce(Name, &mut TransformState) -> Statement,
    {
        let new_covar = state.fresh_covar();
        let cont = |vars: Vec<Var>, _: &mut TransformState| {
            Bind::bind_many(
                self.consumers,
                |covars: Vec<Covar>, state: &mut TransformState| {
                    Cut {
                        producer: Rc::new(
                            Mu {
                                covariable: new_covar.clone(),
                                statement: Rc::new(k(new_covar, state)),
                            }
                            .into(),
                        ),
                        consumer: Rc::new(
                            Destructor {
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
                    }
                    .into()
                },
            )
        };
        Bind::bind_many(self.producers, cont)
    }
}
