use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Name, Op, Statement},
};

impl NamingTransformation for Op {
    fn transform(self, _st: &mut TransformState) -> Op {
        todo!("nor implemented")
    }
}

impl Bind for Op {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
