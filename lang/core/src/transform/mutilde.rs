use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{MuTilde, Name, Statement},
};

impl NamingTransformation for MuTilde {
    fn transform(self, st: &mut TransformState) -> MuTilde {
        MuTilde {
            variable: self.variable,
            statement: self.statement.transform(st),
        }
    }
}

impl Bind for MuTilde {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
