use core::traits::free_vars::{fresh_covar, FreeV};
use fun::syntax::Covariable;
use std::collections::HashSet;

pub struct CompileState {
    pub covars: HashSet<Covariable>,
}

impl CompileState {
    pub fn add_covars<T: FreeV>(&mut self, new_cv: &T) {
        let fr_cv = FreeV::free_covars(new_cv);
        self.covars.extend(fr_cv);
    }

    pub fn free_covar_from_state(&mut self) -> Covariable {
        let new_cv: Covariable = fresh_covar(&self.covars);
        self.covars.insert(new_cv.clone());
        new_cv
    }
}

pub trait Compile {
    type Target;

    fn compile(self, state: &mut CompileState) -> Self::Target;
}

pub trait CompileNew {
    type Target;
    type TargetInner;
    type Continuation;

    fn compile_new(self, _: &mut CompileState) -> Self::Target;
    fn compile_inner(self, _: Self::Continuation, st: &mut CompileState) -> Self::TargetInner;
}
