use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{IfZ, Name, Statement},
};

impl NamingTransformation for IfZ {
    fn transform(self, _st: &mut TransformState) -> IfZ {
        todo!("nor implemented")
    }
}

impl Bind for IfZ {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
