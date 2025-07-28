///! Defines the [Compile] and [CompileWithCont] traits
use core_lang::syntax::{
    CodataDeclaration, Def, Ty,
    context::Chirality,
    fresh_covar, fresh_name, fresh_var,
    statements::Cut,
    terms::{Cns, Mu, Prd},
};
use core_lang::traits::{Typed, TypedFreeVars};
use fun::syntax::{Covar, Name, Var};

use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    rc::Rc,
};

/// Internal state used for the compilation from [fun] to [core]
pub struct CompileState<'a> {
    /// Keeps track of used names
    pub used_vars: HashSet<Var>,
    /// Keeps track of codata types in the program
    /// Needed because some terms are compiled differently depending on type Polarity
    pub codata_types: &'a [CodataDeclaration],
    /// Keeps track of the used labels
    pub used_labels: &'a mut HashSet<Name>,
    /// Contains the name of the definition being currently compiled
    pub current_label: &'a str,
    /// Contains a list of already lifted statements
    pub lifted_statements: &'a mut VecDeque<Def>,
}

impl CompileState<'_> {
    /// Generate a fresh name for a variable
    pub fn fresh_var(&mut self) -> Var {
        fresh_var(&mut self.used_vars)
    }

    /// Generate a fresh name for a covariable
    pub fn fresh_covar(&mut self) -> Covar {
        fresh_covar(&mut self.used_vars)
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
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        let new_covar = state.fresh_covar();
        let new_statement = self.compile_with_cont(
            core_lang::syntax::terms::XVar {
                prdcns: Cns,
                var: new_covar.clone(),
                ty: ty.clone(),
            }
            .into(),
            state,
        );
        Mu {
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
        _: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement;
}

impl<T: CompileWithCont + Clone> CompileWithCont for Rc<T> {
    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        Rc::unwrap_or_clone(self).compile_opt(state, ty)
    }

    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        Rc::unwrap_or_clone(self).compile_with_cont(cont, state)
    }
}

/// Lifts a term to the front in order to reduce duplication
/// Helper function used to compile certain terms
pub fn share(
    cont: core_lang::syntax::Term<Cns>,
    state: &mut CompileState,
) -> core_lang::syntax::Term<Cns> {
    let (var, ty, body) = if let core_lang::syntax::Term::Mu(mu) = cont {
        (mu.variable, mu.ty, Rc::unwrap_or_clone(mu.statement))
    } else {
        let var = state.fresh_var();
        let ty = cont.get_type();
        let body = Cut::new(
            core_lang::syntax::terms::XVar::var(&var, ty.clone()),
            cont,
            ty.clone(),
        )
        .into();

        (var, ty, body)
    };

    // the free variables of the shared statement ...
    let mut typed_free_vars = BTreeSet::new();
    body.typed_free_vars(&mut typed_free_vars);
    let bindings: Vec<_> = typed_free_vars.into_iter().collect();
    // ... become the signature of the lifted label ...
    let context = core_lang::syntax::context::TypingContext {
        bindings: bindings.clone(),
    };
    // ... and the arguments of the call to it
    let bindings = bindings
        .into_iter()
        .map(|binding| match binding.chi {
            Chirality::Prd => {
                let term: core_lang::syntax::terms::Term<Prd> =
                    core_lang::syntax::terms::XVar::var(&binding.var, binding.ty).into();
                term.into()
            }
            Chirality::Cns => {
                let term: core_lang::syntax::terms::Term<Cns> =
                    core_lang::syntax::terms::XVar::covar(&binding.var, binding.ty).into();
                term.into()
            }
        })
        .collect();
    let args = core_lang::syntax::substitution::Substitution { bindings };

    let name = fresh_name(
        state.used_labels,
        &("share_".to_string() + state.current_label + "_"),
    );

    state.lifted_statements.push_front(core_lang::syntax::Def {
        name: name.clone(),
        context,
        body,
        used_vars: state.used_vars.clone(),
    });

    Mu::tilde_mu::<core_lang::syntax::Statement>(
        &var,
        core_lang::syntax::statements::Call {
            name,
            args,
            ty: ty.clone(),
        }
        .into(),
        ty,
    )
    .into()
}
