use super::syntax::{Covar, Name, Statement, Var};
use super::traits::free_vars::{fresh_covar, fresh_var};
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

#[derive(Default)]
pub struct TransformState {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covar>,
}

impl TransformState {
    pub fn fresh_var(&mut self) -> Var {
        let new_var = fresh_var(&self.used_vars);
        self.used_vars.insert(new_var.clone());
        new_var
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

pub type Continuation = Box<dyn FnOnce(Name, &mut TransformState) -> Statement>;
pub type ContinuationVec = Box<dyn FnOnce(VecDeque<Name>, &mut TransformState) -> Statement>;

pub trait Bind: Sized {
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement;
}

pub fn bind_many<T: Bind + 'static>(
    mut args: VecDeque<T>,
    k: ContinuationVec,
    state: &mut TransformState,
) -> Statement {
    match args.pop_front() {
        None => k(VecDeque::new(), state),
        Some(t) => t.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(name);
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
    }
}
