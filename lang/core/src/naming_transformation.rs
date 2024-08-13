use super::syntax::{Covariable, Name, Statement, Var};
use super::traits::free_vars::{fresh_covar, fresh_var, FreeV};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Default)]
pub struct TransformState {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covariable>,
}

impl TransformState {
    pub fn add_vars<T: FreeV>(&mut self, t: T) {
        let free_vars = FreeV::free_vars(&t);
        let _ = free_vars.iter().map(|v| self.used_vars.insert(v.clone()));
    }

    pub fn fresh_var(&mut self) -> Var {
        let new_v = fresh_var(&self.used_vars);
        self.used_vars.insert(new_v.clone());
        new_v
    }

    pub fn add_covars<T: FreeV>(&mut self, t: T) {
        let free_covars = FreeV::free_covars(&t);
        let _ = free_covars
            .iter()
            .map(|cv| self.used_covars.insert(cv.clone()));
    }

    pub fn fresh_covar(&mut self) -> Covariable {
        let new_cv = fresh_covar(&self.used_covars);
        self.used_covars.insert(new_cv.clone());
        new_cv
    }
}

pub trait NamingTransformation {
    fn transform(self, st: &mut TransformState) -> Self;
}

impl<T: NamingTransformation + Clone> NamingTransformation for Rc<T> {
    fn transform(self, st: &mut TransformState) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).transform(st))
    }
}

impl<T: NamingTransformation> NamingTransformation for Vec<T> {
    fn transform(self, st: &mut TransformState) -> Self {
        self.into_iter().map(|x| x.transform(st)).collect()
    }
}

pub trait Bind: Sized {
    fn bind<F>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement;

    fn bind_many<F>(_arg: Vec<Self>, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Vec<Name>) -> Statement,
    {
        todo!("not implemented")
    }
}
