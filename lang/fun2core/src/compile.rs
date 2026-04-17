//! This module defines a trait for the translation of a typechecked [Fun](fun) program into a
//! [Core](core_lang) program. The trait has two methods, [`compile`](Compile::compile) for
//! translating producers to producers, and [`compile_with_cont`](Compile::compile_with_cont) for
//! translating producers to statements. The latter uses an additional consumer input to avoid
//! administrative redexes.

use core_lang::syntax::{
    CodataDeclaration, Def, Statement, Ty,
    arguments::Argument,
    context::Chirality,
    names::Identifier,
    statements::Cut,
    terms::{Cns, Mu, Prd, XVar},
};
use core_lang::traits::{IsCoValue, Typed, TypedFreeVars};
use fun::syntax::names::{Covar, Name, Var, fresh_covar, fresh_name, fresh_var};

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
                var: Identifier::new(new_covar.clone()),
                ty: ty.clone(),
            }
            .into(),
            state,
        );
        Mu {
            prdcns: Prd,
            variable: Identifier::new(new_covar),
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
            core_lang::syntax::terms::XVar::var(Identifier::new(var.clone()), ty.clone()),
            cont,
            ty.clone(),
        )
        .into();

        (Identifier::new(var), ty, body)
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

    let name = fresh_name(
        state.used_labels,
        &("share_".to_string() + state.current_label + "_"),
    );

    state.lifted_statements.push_front(core_lang::syntax::Def {
        name: Identifier::new(name.clone()),
        context,
        body,
    });

    Mu::tilde_mu::<core_lang::syntax::Statement>(
        var,
        core_lang::syntax::statements::Call {
            name: Identifier::new(name),
            args,
            ty: ty.clone(),
        }
        .into(),
        ty,
    )
    .into()
}

/// This is a type alias for a meta-level continuation that abstracts over an argument that has
/// been lifted out of a statement. When the continuation is applied to a term, it returns the
/// statement with the term in the place of the argument that was lifted. The continuation also
/// expects the current state of the translation.
pub type Continuation = Box<dyn FnOnce(Argument, &mut CompileState) -> Statement>;
/// This is a type alias for a meta-level continuation similar to [Continuation], but it abstracts
/// over many arguments at once.
pub type ContinuationVec = Box<dyn FnOnce(VecDeque<Argument>, &mut CompileState) -> Statement>;

/// This function is used during the translation from [Fun](fun) into [Core](core_lang) to avoid
/// administrative redexes in the destructor case. It takes a term that is to be lifted out of
/// argument position if it is not a (co)value and additionally a meta-level
/// [continuation](Continuation) that contains the statement from which the term is lifted. It
/// eventually yields the resulting statement.
/// - `continuation` is the continuation containing the statement from which the term has been
///   lifted.
/// - `state` is the [state](CompileState) threaded through the translation.
fn bind(arg: Argument, k: Continuation, state: &mut CompileState) -> Statement {
    if arg.is_co_value(state.codata_types) {
        k(arg, state)
    } else {
        let ty = arg.get_type();
        match arg {
            Argument::Producer(prd) => {
                let new_var = Identifier::new(state.fresh_var());
                let new_binding = XVar::var(new_var.clone(), ty.clone());
                Cut::new(
                    prd,
                    Mu::tilde_mu(
                        new_var,
                        k(Argument::Producer(new_binding.into()), state),
                        ty.clone(),
                    ),
                    ty,
                )
                .into()
            }
            Argument::Consumer(cns) => {
                let new_covar = Identifier::new(state.fresh_covar());
                let new_binding = XVar::covar(new_covar.clone(), ty.clone());
                Cut::new(
                    Mu::mu(
                        new_covar,
                        k(Argument::Consumer(new_binding.into()), state),
                        ty.clone(),
                    ),
                    cns,
                    ty,
                )
                .into()
            }
        }
    }
}

/// This function is used during the translation from [Fun](fun) into [Core](core_lang) to avoid
/// administrative redexes in the destructor case. It is similar to the [`bind_co_value`]-function,
/// but for a whole list of lifted terms.
/// - `args` is the list of lifted terms.
/// - `continuation` is the continuation containing the statement from which the terms have been
///   lifted.
/// - `state` is the [state](CompileState) threaded through the translation.
pub fn bind_many(
    mut args: VecDeque<Argument>,
    k: ContinuationVec,
    state: &mut CompileState,
) -> Statement {
    match args.pop_front() {
        None => k(VecDeque::new(), state),
        Some(arg) => bind(
            arg,
            Box::new(|binding, state| {
                bind_many(
                    args,
                    Box::new(|mut bindings, state| {
                        bindings.push_front(binding);
                        k(bindings, state)
                    }),
                    state,
                )
            }),
            state,
        ),
    }
}
