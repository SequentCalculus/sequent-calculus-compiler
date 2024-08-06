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
