//! This module defines the variational constraints of type inference, the choices between the
//! overloads of a function they depend on, and the state accumulating both.

use std::collections::HashMap;

use derivative::Derivative;

use crate::{
    syntax::Ty,
    typing::{Error, SymbolTable},
};

/// Identifies one overloaded call site, i.e., one choice between the overloads of a function.
pub type ChoiceId = u32;

/// Selects one alternative of a choice, i.e., the index of one overload of the called function.
pub type Alternative = usize;

/// A partial selection of alternatives under which a constraint or solution holds. A constraint
/// with an empty condition holds in every world.
pub type Condition = HashMap<ChoiceId, Alternative>;

/// A complete selection of alternatives, one for each choice of the program.
pub type World = HashMap<ChoiceId, Alternative>;

/// A choice between the overloads of a function, created for one overloaded call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverloadChoice {
    /// The identifier of the choice
    pub id: ChoiceId,
    /// The number of alternatives, i.e., the number of overloads of the called function
    pub alternatives: usize,
}

/// This struct holds everything type inference accumulates while collecting constraints.
pub struct TypeInferenceState {
    /// The symbol table of the program
    pub symbol_table: SymbolTable,
    /// The generator for fresh type variables and choice identifiers
    pub fresh_names: FreshNames,
    /// The constraints collected so far
    pub constraints: Vec<VariationalConstraint>,
    /// The choices created so far, one for each overloaded call site
    pub choices: Vec<OverloadChoice>,
}

impl TypeInferenceState {
    /// This function creates the initial state for the given symbol table, without any
    /// constraints or choices.
    pub fn new(symbol_table: SymbolTable) -> Self {
        TypeInferenceState {
            symbol_table,
            fresh_names: FreshNames::default(),
            constraints: Vec::new(),
            choices: Vec::new(),
        }
    }

    /// This function creates a new choice with the given number of alternatives and returns its
    /// identifier.
    pub fn new_choice(&mut self, alternatives: usize) -> ChoiceId {
        let id = self.fresh_names.fresh_id();
        self.choices.push(OverloadChoice { id, alternatives });
        id
    }
}

/// This struct generates fresh names for type variables and fresh choice identifiers. Both share
/// one counter, so all generated names and identifiers are distinct.
pub struct FreshNames {
    internal_counter: u32,
}

impl FreshNames {
    /// This function creates a new generator starting at zero.
    pub fn new() -> Self {
        FreshNames {
            internal_counter: 0,
        }
    }

    /// This function returns a fresh name.
    fn fresh_name(&mut self) -> String {
        let new_name = self.internal_counter.to_string();
        self.internal_counter += 1;
        new_name
    }

    /// This function returns a fresh type variable.
    pub fn fresh_ty_var(&mut self) -> Ty {
        let name = self.fresh_name();
        Ty::mk_ty_var(&name)
    }

    /// This function returns a fresh identifier.
    pub fn fresh_id(&mut self) -> u32 {
        let new_id = self.internal_counter;
        self.internal_counter += 1;
        new_id
    }
}

impl Default for FreshNames {
    fn default() -> Self {
        Self::new()
    }
}

/// A combination of alternatives that cannot be selected together, together with the error
/// explaining why.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct ChoiceConflict {
    /// The alternatives that cannot be selected together
    pub condition: Condition,
    /// The type error arising when these alternatives are selected
    pub error: Error,
}

/// A constraint of variational type inference. It only holds in the worlds satisfying its
/// condition.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum VariationalConstraint {
    /// The two types must be equal under the condition.
    Equal {
        lhs: Ty,
        rhs: Ty,
        condition: Condition,
    },
    /// The condition can never be satisfied.
    Impossible(ChoiceConflict),
}

impl VariationalConstraint {
    /// Creates an equality constraint, with lhs = rhs, based on the given condition
    pub fn equal_under(lhs: Ty, rhs: Ty, condition: Condition) -> Self {
        VariationalConstraint::Equal {
            lhs,
            rhs,
            condition,
        }
    }

    /// Creates an equality constraint, with lhs = rhs, holding in every world
    pub fn equal(lhs: Ty, rhs: Ty) -> Self {
        VariationalConstraint::equal_under(lhs, rhs, Condition::default())
    }

    /// Creates an equality constraint with lhs = rhs based on a single alternative of a choice
    pub fn equal_if(lhs: Ty, rhs: Ty, choice: ChoiceId, alternative: Alternative) -> Self {
        VariationalConstraint::equal_under(lhs, rhs, Condition::from([(choice, alternative)]))
    }

    /// Creates a constraint marking a single alternative of a choice as generally not viable
    pub fn impossible_if(choice: ChoiceId, alternative: Alternative, error: Error) -> Self {
        VariationalConstraint::Impossible(ChoiceConflict {
            condition: Condition::from([(choice, alternative)]),
            error,
        })
    }
}
