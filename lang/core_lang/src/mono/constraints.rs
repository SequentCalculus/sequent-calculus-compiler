use std::collections::BTreeSet;

use printer::tokens::COMMA;
use printer::{Alloc, Anno, Builder, DocAllocator, Print, PrintCfg};

use crate::mono::errors::MonoError;
use crate::syntax::{Identifier, Ty};
use crate::typing::env::GlobalEnv;

/// A flow constraint describing how a concrete type reaches a polymorphic type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FlowConstraint {
    /// The concrete type that reaches the polymorphic type parameter.
    pub from: Vec<Ty>,
    /// The name of the polymorphic type parameter.
    pub to: Vec<Identifier>,
}

impl From<(Vec<Ty>, Vec<Identifier>)> for FlowConstraint {
    fn from((from, to): (Vec<Ty>, Vec<Identifier>)) -> Self {
        FlowConstraint { from, to }
    }
}

impl Print for FlowConstraint {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let sep = || alloc.text(",").append(alloc.space());

        let from_body = alloc.intersperse(self.from.iter().map(|t| t.print(cfg, alloc)), sep());
        let from_part = alloc
            .text("[")
            .append(from_body)
            .append(alloc.text("]"))
            .group();

        let to_body = alloc.intersperse(self.to.iter().map(|id| id.print(cfg, alloc)), sep());
        let to_part = alloc
            .text("[")
            .append(to_body)
            .append(alloc.text("]"))
            .group();

        from_part.append(alloc.text(" ⊑ ")).append(to_part)
    }
}

/// A set of flow constraints. This is the main output of the constraint collection phase and the main input to the
/// constraint solving phase.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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

impl Print for FlowConstraintSet {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        // fallback for empty sets
        if self.constraints.is_empty() {
            return alloc
                .text("{")
                .annotate(Anno::BraceOpen)
                .append(alloc.text("}").annotate(Anno::BraceClose));
        }

        let sep = if cfg.allow_linebreaks {
            alloc.text(COMMA).append(alloc.line())
        } else {
            alloc.text(COMMA).append(alloc.space())
        };

        let body = alloc.intersperse(
            self.constraints.iter().map(|x| x.print(cfg, alloc).group()),
            sep,
        );

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
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError>;
}

/// This function collects flow constraints from a concrete type reaching a polymorphic type parameter. It is used as a helper function in the implementation of the `ConstraintCollector` trait for various syntax elements.
pub fn collect_type_flow(
    sources: &[Ty],
    targets: &[Identifier],
) -> Result<FlowConstraintSet, MonoError> {
    let mut constraints = FlowConstraintSet::new();

    if sources.len() != targets.len() {
        return Err(MonoError::TypeMismatch {
            expected: format!("Sequence of {} types", targets.len()),
            got: format!("Sequence of {} types", sources.len()),
            msg: Some("Arity mismatch in type sequences".to_string()),
        });
    }

    // avoid generating constraints for empty source types, as they do not contribute to the flow.
    if sources.is_empty() {
        return Ok(constraints);
    }

    constraints.insert(FlowConstraint {
        from: sources.to_vec(),
        to: targets.to_vec(),
    });
    Ok(constraints)
}
