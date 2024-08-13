use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Mu, Name, Statement},
};

impl NamingTransformation for Mu {
    fn transform(self, st: &mut TransformState) -> Mu {
        Mu {
            covariable: self.covariable,
            statement: self.statement.transform(st),
        }
    }
}

impl Bind for Mu {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
