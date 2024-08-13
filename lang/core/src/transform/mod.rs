pub mod cut;
pub mod dtor;
pub mod fun;
pub mod ifz;
pub mod mu;
pub mod op;
use super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Consumer, Def, Name, Producer, Prog, Statement},
};

impl<T> NamingTransformation for Prog<T> {
    fn transform(self: Prog<T>, st: &mut TransformState) -> Prog<T> {
        let mut new_defs = vec![];
        for def in self.prog_defs.into_iter() {
            new_defs.push(def.transform(st));
        }
        Prog {
            prog_defs: new_defs,
        }
    }
}

impl<T> NamingTransformation for Def<T> {
    fn transform(self: Def<T>, st: &mut TransformState) -> Def<T> {
        Def {
            name: self.name,
            pargs: self.pargs,
            cargs: self.cargs,
            body: self.body.transform(st),
        }
    }
}

impl NamingTransformation for Statement {
    fn transform(self: Statement, st: &mut TransformState) -> Statement {
        match self {
            Statement::Cut(cut) => cut.transform(st).into(),
            Statement::Op(op) => op.transform(st).into(),
            Statement::IfZ(ifz) => ifz.transform(st).into(),
            Statement::Fun(fun) => fun.transform(st).into(),
            Statement::Done() => Statement::Done(),
        }
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
