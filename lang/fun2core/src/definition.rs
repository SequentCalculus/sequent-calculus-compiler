use core::traits::free_vars::{fresh_covar, FreeV};
use fun::syntax::Covariable;
use std::{collections::HashSet, rc::Rc};

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
/// intermediate language `Core`. For terms you should use the trait `CompileWithCont`
/// which implements a optimized translation which does not generate administrative redexes.
pub trait Compile {
    type Target;
    /// If you want a translation of terms which does not produce administrative redexes
    /// then you should use the function `compile_opt` from the `CompileWithCont` trait.
    fn compile(self, state: &mut CompileState) -> Self::Target;
}

impl<T: Compile + Clone> Compile for Rc<T> {
    type Target = Rc<T::Target>;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).compile(state))
    }
}

/// A trait for compiling terms(!) from the surface language `Fun` to the intermediate
/// language `Core`. The generated expressions do not contain administrative redexes.
pub trait CompileWithCont {
    /// An optimized version of the `compile` function of the `Compile` trait which does not
    /// generate administrative redexes.
    fn compile_opt(self, _: &mut CompileState) -> core::syntax::Producer;

    /// Compile a term to a producer. This function takes a continuation as an additional argument
    /// in order to not generate superfluous administrative redexes.
    fn compile_with_cont(
        self,
        _: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement;
}

impl<T: CompileWithCont + Clone> CompileWithCont for Rc<T> {
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        Rc::unwrap_or_clone(self).compile_opt(st)
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        Rc::unwrap_or_clone(self).compile_with_cont(cont, st)
    }
}
