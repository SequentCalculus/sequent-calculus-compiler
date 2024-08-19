use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Covar, Covariable, Fun, Statement, Var, Variable},
};

impl NamingTransformation for Fun {
    type Target = Statement;
    ///N(f(p_i; c_j)) = bind(p_i)[λas.bind(c_j)[λbs.f(as; bs)]]
    fn transform(self, _state: &mut TransformState) -> Statement {
        let cont = |vars: Vec<Var>, _: &mut TransformState| {
            Bind::bind_many(
                self.consumers,
                |covars: Vec<Covar>, _: &mut TransformState| {
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
                },
            )
        };
        Bind::bind_many(self.producers, cont)
    }
}
