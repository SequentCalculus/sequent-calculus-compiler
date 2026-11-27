//! This module implements type inference with overload resolution for Fun. It proceeds in three phases:
//! - [`TypeInference::collect_constraints`] collects variational constraints, where each call of
//!   an overloaded function introduces a choice between its overloads ([`constraints`]),
//! - [`unify_variational`] solves these constraints into conditional solutions and conflicts
//!   ([`unification`]),
//! - [`select_world`] selects the unique combination of overloads without conflicts
//!   ([`resolution`]).

use std::{collections::HashMap, rc::Rc};

use crate::{
    syntax::{Name, Ty, TypingContext},
    typing::{Error, SymbolTable},
};

pub mod constraints;
pub mod resolution;
pub mod solve;
pub mod unification;

pub use constraints::*;
pub use resolution::*;
pub use solve::*;
pub use unification::*;

/// Maps the names of type variables to the types inferred for them.
pub type TypeVarMapping = HashMap<Name, Ty>;

/// This trait provides type inference for the terms of Fun. Inference happens in two passes: the
/// first collects the constraints of all terms, the second inserts the types computed from the
/// solution of these constraints.
pub trait TypeInference: Sized {
    /// This function collects the constraints for a term in the given typing context.
    /// - `state` is the inference state, which receives the constraints and choices.
    /// - `context` is the typing context in which the term is typed.
    /// - `expected_ty` is the type the term must have, often a fresh type variable.
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error>;

    /// This function replaces the type variables in the annotations of a term with the inferred
    /// types and resolves overloaded calls according to the selected world.
    /// - `type_vars` maps type variables to their inferred types.
    /// - `symbol_table` is used to create the monomorphic instances of the inferred types.
    /// - `world` is the selected alternative for each choice.
    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error>;
}

impl<T: TypeInference + Clone> TypeInference for Rc<T> {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        Rc::make_mut(self).collect_constraints(state, context, expected_ty)
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        Rc::make_mut(self).insert_inferred_types(type_vars, symbol_table, world)
    }
}

impl<T: TypeInference> TypeInference for Option<T> {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.collect_constraints(state, context, expected_ty),
        }
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        match self {
            None => Ok(()),
            Some(t) => t.insert_inferred_types(type_vars, symbol_table, world),
        }
    }
}
