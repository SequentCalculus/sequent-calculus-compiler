use std::collections::HashSet;

use crate::mono::errors::Error;
use crate::syntax::{CodataDeclaration, DataDeclaration, Ty};

/// A flow constraint describing how a concrete type reaches a polymorphic type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FlowConstraint {
    /// The concrete type that reaches the polymorphic type parameter.
    pub from: Ty,
    /// The name of the polymorphic type parameter.
    pub to: Ty,
}

/// A set of flow constraints. This is the main output of the constraint collection phase and the main input to the
/// constraint solving phase.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FlowConstraintSet {
    pub constraints: HashSet<FlowConstraint>,
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
pub fn collect_type_flow(actual: &Ty, expected: &Ty) -> Result<FlowConstraintSet, Error> {
    fn collect_type_flow_into(
        actual: &Ty,
        expected: &Ty,
        constraints: &mut FlowConstraintSet,
    ) -> Result<(), Error> {
        match (actual, expected) {
            (_, Ty::Var(param)) => {
                let target = Ty::Var(param.clone());
                constraints.insert(FlowConstraint {
                    from: actual.clone(),
                    to: target.clone(),
                });

                if let Ty::Decl { type_args, .. } = actual {
                    for arg in &type_args.args {
                        collect_type_flow_into(arg, &target, constraints)?;
                    }
                }
                Ok(())
            }
            (Ty::I64, Ty::I64) => Ok(()),
            (Ty::Decl { .. }, Ty::Decl { .. }) => Err(Error::TypeMismatch {
                expected: expected.clone(),
                got: actual.clone(),
                msg: Some("Expected a polymorphic type parameter on the right-hand side of the flow constraint, but got a concrete type declaration.".to_string()),
            }),
            _ => Err(Error::TypeMismatch {
                expected: expected.clone(),
                got: actual.clone(),
                msg: None,
            }),
        }
    }

    let mut constraints = FlowConstraintSet::new();
    collect_type_flow_into(actual, expected, &mut constraints)?;
    Ok(constraints)
}
