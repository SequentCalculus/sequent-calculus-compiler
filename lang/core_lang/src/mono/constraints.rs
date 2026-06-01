use std::collections::HashSet;

use printer::tokens::COMMA;
use printer::{Alloc, Anno, Builder, DocAllocator, Print, PrintCfg};

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
impl Print for FlowConstraint {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.from
            .print(cfg, alloc)
            .append(alloc.text(" ⊑ "))
            .append(self.to.print(cfg, alloc))
    }
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

impl Print for FlowConstraintSet {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        // fallback for empty sets
        if self.constraints.is_empty() {
            return alloc
                .text("{")
                .annotate(Anno::BraceOpen)
                .append(alloc.text("}").annotate(Anno::BraceClose));
        }

        // sort for deterministic output
        let mut sorted: Vec<&FlowConstraint> = self.constraints.iter().collect();
        sorted.sort();

        let sep = if cfg.allow_linebreaks {
            alloc.text(COMMA).append(alloc.line())
        } else {
            alloc.text(COMMA).append(alloc.space())
        };

        let body = alloc.intersperse(sorted.into_iter().map(|x| x.print(cfg, alloc).group()), sep);

        if cfg.allow_linebreaks {
            alloc
                .text("{")
                .annotate(Anno::BraceOpen)
                .append(alloc.line().append(body).nest(cfg.indent))
                .append(alloc.line())
                .append(alloc.text("}").annotate(Anno::BraceClose))
                .group()
        } else {
            alloc
                .text("{ ")
                .annotate(Anno::BraceOpen)
                .append(body)
                .append(alloc.text(" }").annotate(Anno::BraceClose))
        }
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
