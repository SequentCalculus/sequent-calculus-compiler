use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Cocase, Name, Statement},
};

impl NamingTransformation for Cocase {
    fn transform(self, st: &mut TransformState) -> Cocase {
        Cocase {
            cocases: self.cocases.transform(st),
        }
    }
}

impl Bind for Cocase {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
