use super::syntax::{Covar, Name, Statement, Var};
use super::traits::free_vars::{fresh_covar, fresh_var, FreeV};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Default)]
pub struct TransformState {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covar>,
}

impl TransformState {
    pub fn add_vars<T: FreeV>(&mut self, t: &T) {
        self.used_vars.extend(FreeV::free_vars(t));
    }

    pub fn fresh_var(&mut self) -> Var {
        let new_var = fresh_var(&self.used_vars);
        self.used_vars.insert(new_var.clone());
        new_var
    }

    pub fn add_covars<T: FreeV>(&mut self, t: &T) {
        self.used_covars.extend(FreeV::free_covars(t));
    }

    pub fn fresh_covar(&mut self) -> Covar {
        let new_covar = fresh_covar(&self.used_covars);
        self.used_covars.insert(new_covar.clone());
        new_covar
    }
}

pub trait NamingTransformation {
    type Target;
    fn transform(self, state: &mut TransformState) -> Self::Target;
}

impl<T: NamingTransformation + Clone> NamingTransformation for Rc<T> {
    type Target = Rc<T::Target>;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).transform(state))
    }
}

impl<T: NamingTransformation> NamingTransformation for Vec<T> {
    type Target = Vec<T::Target>;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        self.into_iter().map(|x| x.transform(state)).collect()
    }
}

pub trait Bind: Sized {
    fn bind<K>(self, k: K, state: &mut TransformState) -> Statement
    where
        K: FnOnce(Name, &mut TransformState) -> Statement;

    fn bind_many<K>(_arg: Vec<Self>, _k: K) -> Statement
    where
        K: FnOnce(Vec<Name>, &mut TransformState) -> Statement,
    {
        todo!("not implemented")
    }
}
