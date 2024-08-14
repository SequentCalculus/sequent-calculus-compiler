use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Covar, Covariable, Fun, Statement, Var, Variable},
};

impl NamingTransformation for Fun {
    type Target = Statement;
    ///N (f (pi ; c j )) = bind(pi ) [λas.bind(c j ) [λbs.f (as; bs)]]
    fn transform(self, _st: &mut TransformState) -> Statement {
        let cont = |ns: Vec<Var>| {
            |_: &mut TransformState| {
                Bind::bind_many(self.consumers, |bs: Vec<Covar>| {
                    |_: &mut TransformState| {
                        Fun {
                            name: self.name,
                            producers: ns.into_iter().map(|n| Variable { var: n }.into()).collect(),
                            consumers: bs
                                .into_iter()
                                .map(|b| Covariable { covar: b }.into())
                                .collect(),
                        }
                        .into()
                    }
                })
            }
        };
        Bind::bind_many(self.producers, cont)
    }
}
