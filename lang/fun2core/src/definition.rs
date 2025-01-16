use core_lang::syntax::{
    fresh_var,
    term::{Cns, Prd},
    CodataDeclaration, Ty,
};
use fun::syntax::{Covariable, Variable};

use std::{collections::HashSet, rc::Rc};

#[derive(Default)]
pub struct CompileState<'a> {
    pub used_vars: HashSet<Variable>,
    pub codata_types: &'a [CodataDeclaration],
}

impl CompileState<'_> {
    pub fn fresh_var(&mut self) -> Variable {
        fresh_var(&mut self.used_vars, "x")
    }

    pub fn fresh_covar(&mut self) -> Covariable {
        fresh_var(&mut self.used_vars, "a")
    }
}

/// A trait for compiling items from the surface language `Fun` to the
/// intermediate language `Core`. For terms you should use the trait `CompileWithCont`
/// that implements an optimized translation which does not generate administrative redexes.
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
pub trait CompileWithCont: Sized {
    /// An optimized version of the `compile` function of the `Compile` trait which does not
    /// generate administrative redexes.
    ///
    /// There is a default implementation which implements the following translation:
    /// ```text
    /// 〚t〛= μ a. 〚t〛_{a}  (a fresh)
    /// ```
    /// This translation is always correct, but generates an eta-redex if it is used for
    /// non-computations, e.g.:
    /// ```text
    /// 〚5〛= μ a. 〚5〛_{a} = μ a. < 5 | a > =η 5
    /// ```
    /// Therefore, an optimized version of this function is implemented for non-computations.
    ///
    /// In comments we write `〚t〛` for `compile_opt`.
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::term::Term<Prd> {
        let new_covar = state.fresh_covar();
        let new_statement = self.compile_with_cont(
            core_lang::syntax::term::XVar {
                prdcns: Cns,
                var: new_covar.clone(),
                ty: ty.clone(),
            }
            .into(),
            state,
        );
        core_lang::syntax::term::Mu {
            prdcns: Prd,
            variable: new_covar,
            ty,
            statement: Rc::new(new_statement),
        }
        .into()
    }

    /// Compile a term to a producer. This function takes a continuation as an additional argument
    /// in order not to generate superfluous administrative redexes.
    ///
    /// In comments we write `〚t〛_{c}` for this function.
    fn compile_with_cont(
        self,
        _: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement;
}

impl<T: CompileWithCont + Clone> CompileWithCont for Rc<T> {
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::term::Term<Prd> {
        Rc::unwrap_or_clone(self).compile_opt(state, ty)
    }

    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        Rc::unwrap_or_clone(self).compile_with_cont(cont, state)
    }
}
