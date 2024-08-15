pub mod case;
pub mod clause;
pub mod cocase;
pub mod consumer;
pub mod ctor;
pub mod cut;
pub mod dtor;
pub mod fun;
pub mod ifz;
pub mod lit;
pub mod mu;
pub mod mutilde;
pub mod op;
pub mod producer;

use super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::{Def, Prog, Statement},
};

impl<T> NamingTransformation for Prog<T> {
    type Target = Prog<T>;
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
    type Target = Def<T>;
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
    type Target = Statement;
    fn transform(self: Statement, st: &mut TransformState) -> Statement {
        match self {
            Statement::Cut(cut) => cut.transform(st),
            Statement::Op(op) => op.transform(st),
            Statement::IfZ(ifz) => ifz.transform(st),
            Statement::Fun(fun) => fun.transform(st),
            Statement::Done() => Statement::Done(),
        }
    }
}
