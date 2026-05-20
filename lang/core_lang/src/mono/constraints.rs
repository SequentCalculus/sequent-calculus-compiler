use std::collections::BTreeSet;

use crate::mono::errors::Error;
use crate::syntax::{CodataDeclaration, DataDeclaration, Ty};

/// A flow constraint describing how a concrete type reaches a polymorphic type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct FlowConstraint {
    /// The concrete type that reaches the polymorphic type parameter.
    pub from: Ty,
    /// The name of the polymorphic type parameter.
    pub to: Ty,
}

/// A set of flow constraints. This is the main output of the constraint collection phase and the main input to the
/// constraint solving phase.
#[derive(Debug, Clone, PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct FlowConstraintSet {
    pub constraints: BTreeSet<FlowConstraint>,
}

impl FlowConstraintSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, constraint: FlowConstraint) {
        self.constraints.insert(constraint);
    }

    pub fn extend(&mut self, other: FlowConstraintSet) {
        self.constraints.extend(other.constraints);
    }
}

/// This trait defines the interface for collecting flow constraints from a syntax element.
pub trait ConstraintCollector {
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, Error>;
}

/// This function collects flow constraints from a concrete type reaching a polymorphic type parameter. It is used as a helper function in the implementation of the `ConstraintCollector` trait for various syntax elements.
pub fn collect_type_flow(
    actual: &Ty,
    expected: &Ty,
    constraints: &mut FlowConstraintSet,
) -> Result<(), Error> {
    match (actual, expected) {
        (_, Ty::Var(param)) => {
            constraints.insert(FlowConstraint {
                from: actual.clone(),
                to: Ty::Var(param.clone()),
            });
            Ok(())
        }
        (Ty::I64, Ty::I64) => Ok(()),
        (
            Ty::Decl {
                name: actual_name,
                type_args: actual_args,
            },
            Ty::Decl {
                name: expected_name,
                type_args: expected_args,
            },
        ) if actual_name == expected_name => {
            if actual_args.args.len() != expected_args.args.len() {
                return Err(Error::ArityMismatch {
                    expected: expected_args.args.len(),
                    got: actual_args.args.len(),
                });
            }

            for (actual_arg, expected_arg) in actual_args.args.iter().zip(expected_args.args.iter())
            {
                collect_type_flow(actual_arg, expected_arg, constraints)?;
            }
            Ok(())
        }
        _ => Err(Error::TypeMismatch {
            expected: expected.clone(),
            got: actual.clone(),
        }),
    }
}
