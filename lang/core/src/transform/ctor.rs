use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{
        Constructor, Covar, Covariable, Cut, Mu, MuTilde, Name, Producer, Statement, Var, Variable,
    },
};
use std::rc::Rc;

impl NamingTransformation for Constructor {
    type Target = Producer;

    ///N (K (pi ; c j )) = μα .bind(pi ) [λas.bind(c j ) [λbs.⟨K (as; bs) | α⟩]]
    fn transform(self, st: &mut TransformState) -> Producer {
        let new_cv = st.fresh_covar();
        let new_st = Bind::bind_many(self.producers, |ns: Vec<Var>| {
            |_: &mut TransformState| {
                Bind::bind_many(self.consumers, |bs: Vec<Covar>| {
                    |_: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Constructor {
                                    id: self.id,
                                    producers: ns
                                        .into_iter()
                                        .map(|n| Variable { var: n }.into())
                                        .collect(),
                                    consumers: bs
                                        .into_iter()
                                        .map(|b| Covariable { covar: b }.into())
                                        .collect(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                Covariable {
                                    covar: new_cv.clone(),
                                }
                                .into(),
                            ),
                        }
                        .into()
                    }
                })
            }
        });
        Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
        .into()
    }
}

impl Bind for Constructor {
    ///bind(K (pi ; c j )) [k] =  bind(p i ) [λas.bind(c j ) [λbs.⟨K (as; bs) | μx  ̃ .k (x)⟩]]
    fn bind<F, K>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: FnOnce(Name) -> K,
        K: FnOnce(&mut TransformState) -> Statement,
    {
        let new_v = st.fresh_var();
        let cont = |ns: Vec<Var>| {
            |_: &mut TransformState| {
                Bind::bind_many(self.consumers, |bs: Vec<Covar>| {
                    |st: &mut TransformState| {
                        Cut {
                            producer: Rc::new(
                                Constructor {
                                    id: self.id,
                                    producers: ns
                                        .into_iter()
                                        .map(|n| Variable { var: n }.into())
                                        .collect(),
                                    consumers: bs
                                        .into_iter()
                                        .map(|b| Covariable { covar: b }.into())
                                        .collect(),
                                }
                                .into(),
                            ),
                            consumer: Rc::new(
                                MuTilde {
                                    variable: new_v.clone(),
                                    statement: Rc::new(k(new_v)(st)),
                                }
                                .into(),
                            ),
                        }
                        .into()
                    }
                })
            }
        };
        Bind::bind_many(self.producers, cont)
    }
}
