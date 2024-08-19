use super::super::{
    naming_transformation::{bind_many, Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Consumer, Covariable, Cut, Destructor, Mu, MuTilde, Statement, Variable},
};
use std::rc::Rc;

impl NamingTransformation for Destructor {
    type Target = Consumer;
    ///N(D(p_i; cj)) =  ~μx.bind(p_i)[λas.bind(c_j)[λbs.⟨x | D(as; bs)⟩]]
    fn transform(self, state: &mut TransformState) -> Consumer {
        let new_var = state.fresh_var();
        let new_var_clone = new_var.clone();
        let new_statement = bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, _: &mut TransformState| {
                        Cut {
                            producer: Rc::new(Variable { var: new_var }.into()),
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
                    }),
                    state,
                )
            }),
            state,
        );
        MuTilde {
            variable: new_var_clone,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl Bind for Destructor {
    ///bind(D(p_i; c_j))[k] = bind(p_i)[λas.bind(c_j)[λbs.⟨μa.k(a) | D(as; bs)⟩]]
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_covar = state.fresh_covar();
        bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, state: &mut TransformState| {
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
                    }),
                    state,
                )
            }),
            state,
        )
    }
}
