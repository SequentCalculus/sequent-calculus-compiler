use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Fun, Name, Statement},
};

impl NamingTransformation for Fun {
    fn transform(self, _st: &mut TransformState) -> Fun {
        todo!("nor implemented")
    }
}

impl Bind for Fun {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
