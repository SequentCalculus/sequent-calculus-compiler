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

/// A trait for compiling items from the surface language `Fun` to the
/// intermediate language `Core`.
pub trait Compile {
    type Target;
    /// Applying this `compile` function to terms results in a "naive" compilation
    /// result which contains administrative redexes. If you want a translation
    /// which does not produce administrative redexes then you should use the
    /// function `compile_opt` from the `CompileWithCont` trait.
    fn compile(self, state: &mut CompileState) -> Self::Target;
}

/// A trait for compiling terms(!) from the surface language `Fun` to the intermediate
/// language `Core`. The generated expressions do not contain administrative redexes.
pub trait CompileWithCont {
    type Target;
    type TargetInner;

    /// An optimized version of the `compile` function of the `Compile` trait which does not
    /// generate administrative redexes.
    fn compile_opt(self, _: &mut CompileState) -> Self::Target;

    /// Compile a term to a producer. This function takes a continuation as an additional argument
    /// in order to not generate superfluous administrative redexes.
    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> Self::TargetInner;
}
