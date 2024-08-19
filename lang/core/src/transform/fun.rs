use super::super::{
    naming_transformation::{bind_many, NamingTransformation, TransformState},
    syntax::{Covariable, Fun, Statement, Variable},
};

impl NamingTransformation for Fun {
    type Target = Statement;
    ///N(f(p_i; c_j)) = bind(p_i)[λas.bind(c_j)[λbs.f(as; bs)]]
    fn transform(self, state: &mut TransformState) -> Statement {
        bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, _: &mut TransformState| {
                        Fun {
                            name: self.name,
                            producers: vars
                                .into_iter()
                                .map(|var| Variable { var }.into())
                                .collect(),
                            consumers: covars
                                .into_iter()
                                .map(|covar| Covariable { covar }.into())
                                .collect(),
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
