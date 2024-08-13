use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Destructor, Name, Statement},
};

impl NamingTransformation for Destructor {
    fn transform(self, _st: &mut TransformState) -> Destructor {
        todo!("not implemented")
    }
}

impl Bind for Destructor {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
