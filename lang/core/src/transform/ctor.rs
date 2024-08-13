use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Constructor, Name, Statement},
};

impl NamingTransformation for Constructor {
    fn transform(self, _st: &mut TransformState) -> Constructor {
        todo!("not implemented")
    }
}

impl Bind for Constructor {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
