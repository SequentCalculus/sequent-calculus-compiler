use super::syntax::{Consumer, Covariable, Def, Producer, Prog, Statement, Var};
use super::traits::free_vars::FreeV;
use std::collections::HashSet;

pub struct TransformState {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covariable>,
}

impl TransformState {
    pub fn add_vars<T: FreeV>(&mut self, t: T) -> () {
        let free_vars = FreeV::free_vars(&t);
        let _ = free_vars.iter().map(|v| self.used_vars.insert(v.clone()));
    }

    pub fn add_covars<T: FreeV>(&mut self, t: T) -> () {
        let free_covars = FreeV::free_covars(&t);
        let _ = free_covars
            .iter()
            .map(|cv| self.used_covars.insert(cv.clone()));
    }
}

pub trait NamingTransformation {
    fn transform(self, st: &mut TransformState) -> Self;
}

impl<T> NamingTransformation for Prog<T> {
    fn transform(self: Prog<T>, _: &mut TransformState) -> Prog<T> {
        panic!("")
    }
}
impl<T> NamingTransformation for Def<T> {
    fn transform(self: Def<T>, _: &mut TransformState) -> Def<T> {
        panic!("")
    }
}

impl NamingTransformation for Statement {
    fn transform(self: Statement, _: &mut TransformState) -> Statement {
        panic!("")
    }
}

impl NamingTransformation for Producer {
    fn transform(self: Producer, _: &mut TransformState) -> Producer {
        panic!("")
    }
}

impl NamingTransformation for Consumer {
    fn transform(self: Consumer, _: &mut TransformState) -> Consumer {
        panic!("")
    }
}

pub trait Bind {
    type Name;
    fn bind(self, k: &dyn Fn(Self::Name) -> Statement, st: &mut TransformState) -> Statement;
}
