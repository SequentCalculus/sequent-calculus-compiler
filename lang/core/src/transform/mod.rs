use super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Consumer, Def, Name, Producer, Prog, Statement},
};

impl<T> NamingTransformation for Prog<T> {
    fn transform(self: Prog<T>, _: &mut TransformState) -> Prog<T> {
        todo!("not implemented")
    }
}

impl<T> Bind for Prog<T> {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not implemented")
    }
}

impl<T> NamingTransformation for Def<T> {
    fn transform(self: Def<T>, _: &mut TransformState) -> Def<T> {
        todo!("not implemented")
    }
}
impl<T> Bind for Def<T> {
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("Not implemented")
    }
}

impl NamingTransformation for Statement {
    fn transform(self: Statement, _: &mut TransformState) -> Statement {
        todo!("not implemented")
    }
}

impl Bind for Statement {
    fn bind<F>(self, _k: F, _str: &mut TransformState) -> Statement {
        todo!("not implemented")
    }
}

impl NamingTransformation for Producer {
    fn transform(self: Producer, _: &mut TransformState) -> Producer {
        todo!("not implemented")
    }
}

impl Bind for Producer {
    fn bind<F>(self, _k: F, _: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not implemented")
    }
}

impl NamingTransformation for Consumer {
    fn transform(self: Consumer, _: &mut TransformState) -> Consumer {
        todo!("not implemented")
    }
}

impl Bind for Consumer {
    fn bind<F>(self, _k: F, _: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not implemented")
    }
}
