//! This module defines a trait for the translation of a typechecked [Fun](fun) program into a
//! [Core](core_lang) program. The trait has two methods, [`compile`](Compile::compile) for
//! translating producers to producers, and [`compile_with_cont`](Compile::compile_with_cont) for
//! translating producers to statements. The latter uses an additional consumer input to avoid
//! administrative redexes.

use core_lang::syntax::{
    CodataDeclaration, Def, Ty,
    context::Chirality,
    names::Ident,
    statements::Cut,
    terms::{Cns, Mu, Prd},
};
use core_lang::traits::{Typed, TypedFreeVars};
use fun::syntax::names::{Covar, Name, Var, fresh_covar, fresh_ident, fresh_var};

use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    rc::Rc,
};

/// This struct defines the state of the translation from [Fun](fun) into [Core](core_lang). It
/// consists of the variable names used in the top-level function we are currently in needed for
/// generating fresh variable names, the codata type declarations, the labels of top-level
/// functions used in the program and the label of the function we are currently in needed to
/// generate fresh labels, and a list of top-level definitions containing statements within the
/// current function that have been lifted to the top-level.
pub struct CompileState<'a> {
    /// The names used in the top-level definition
    pub used_vars: HashSet<Var>,
    /// The codata types in the program
    pub codata_types: &'a [CodataDeclaration],
    /// The labels for top-level functions used in the program
    pub used_labels: &'a mut HashSet<Name>,
    /// The name of the definition being currently compiled
    pub current_label: &'a str,
    /// A list of already lifted statements
    pub lifted_statements: &'a mut VecDeque<Def>,
}

impl CompileState<'_> {
    /// This function generates a fresh variable with base name `"x"`.
    pub fn fresh_var(&mut self) -> Var {
        fresh_var(&mut self.used_vars)
    }

    /// This function generates a fresh covariable with base name `"a"`.
    pub fn fresh_covar(&mut self) -> Covar {
        fresh_covar(&mut self.used_vars)
    }
}

/// This trait provides two methods for the translation from the surface language [Fun](fun) into
/// the intermediate language [Core](core_lang).
pub trait Compile: Sized {
    /// This method translates a term from the surface language [Fun](fun) into the intermediate
    /// language [Core](core_lang). It is used for producers in [Fun](fun) that are translated to
    /// statements in [Core](core_lang). It uses an additional consumer input to avoid
    /// administrative redexes
    /// - `consumer` is the consumer input.
    /// - `state` is the [state](CompileState) threaded through the translation.
    fn compile_with_cont(
        self,
        consumer: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement;

    /// This method translates a term from the surface language [Fun](fun) into the intermediate
    /// language [Core](core_lang). It is used for producers in [Fun](fun) that are translated to
    /// producers in [Core](core_lang).
    /// - `state` is the [state](CompileState) threaded through the translation.
    /// - `ty` is the type of the translated term.
    ///
    /// There is a default implementation that works as follows:
    /// ```text
    /// 〚t〛= μ a. 〚t〛_{a}  (a fresh)
    /// ```
    /// This translation is always correct, but generates an η-redex if it is used for
    /// non-computations, e.g.,
    /// ```text
    /// 〚5〛= μ a. 〚5〛_{a} = μ a. < 5 | a > =η 5
    /// ```
    /// Therefore, an optimized version of this function is implemented for non-computations.
    fn compile(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        let new_covar = state.fresh_covar();
        let new_statement = self.compile_with_cont(
            core_lang::syntax::terms::XVar {
                prdcns: Cns,
                var: Ident::new_with_zero(&new_covar),
                ty: ty.clone(),
            }
            .into(),
            state,
        );
        Mu {
            prdcns: Prd,
            variable: Ident::new_with_zero(&new_covar),
            ty,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

impl<T: Compile + Clone> Compile for Rc<T> {
    fn compile(self, state: &mut CompileState, ty: Ty) -> core_lang::syntax::terms::Term<Prd> {
        Rc::unwrap_or_clone(self).compile(state, ty)
    }

    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        Rc::unwrap_or_clone(self).compile_with_cont(cont, state)
    }
}

/// This function lifts a consumer to the top-level for sharing, in order to avoid exponential
/// blowup by duplication. It returns a consumer that calls the lifted consumer.
pub fn share(
    cont: core_lang::syntax::Term<Cns>,
    state: &mut CompileState,
) -> core_lang::syntax::Term<Cns> {
    // if the consumer is a mu-tilde, we simply lift its body
    let (var, ty, body) = if let core_lang::syntax::Term::Mu(mu) = cont {
        (mu.variable, mu.ty, Rc::unwrap_or_clone(mu.statement))
    } else {
        // otherwise we abstract a fresh variable and lift the cut of the this variable with the
        // consumer
        let var = state.fresh_var();
        let ty = cont.get_type();
        let body = Cut::new(
            core_lang::syntax::terms::XVar::var(Ident::new_with_zero(&var), ty.clone()),
            cont,
            ty.clone(),
        )
        .into();

        (Ident::new_with_zero(&var), ty, body)
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
                    core_lang::syntax::terms::XVar::var(binding.var, binding.ty).into();
                term.into()
            }
            Chirality::Cns => {
                let term: core_lang::syntax::terms::Term<Cns> =
                    core_lang::syntax::terms::XVar::covar(binding.var, binding.ty).into();
                term.into()
            }
        })
        .collect();
    let args = core_lang::syntax::arguments::Arguments { entries: bindings };

    let name = fresh_ident(
        state.used_labels,
        &("share_".to_string() + state.current_label + "_"),
    );

    state.lifted_statements.push_front(core_lang::syntax::Def {
        name: Ident::new_with_zero(&name),
        context,
        body,
        used_vars: state
            .used_vars
            .iter()
            .map(|id| Ident::new_with_zero(id))
            .collect(),
    });

    Mu::tilde_mu::<core_lang::syntax::Statement>(
        var,
        core_lang::syntax::statements::Call {
            name: Ident::new_with_zero(&name),
            args,
            ty: ty.clone(),
        }
        .into(),
        ty,
    )
    .into()
}
