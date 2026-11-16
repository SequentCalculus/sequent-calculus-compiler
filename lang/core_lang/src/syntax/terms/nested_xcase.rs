use std::rc::Rc;

use crate::syntax::{AnyPattern, Chi, Statement, Ty};

/// An ordered match clause with one outer pattern and a statement body.
///
/// The body is shared through `Rc`. Pattern binders scope over the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedClause {
    /// The single outer pattern of this clause.
    pub pattern: AnyPattern,
    /// The statement executed when this clause is selected.
    pub body: Rc<Statement>,
}

/// The syntax representation of a nested match or comatch.
///
/// `Cns` represents case^N and `Prd` represents new^N.
/// Outer clause patterns must have the opposite chirality to the terms.
/// This struct alone does not enforce that invariant or exhaustiveness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedXCase<C: Chi> {
    /// The chirality of the match term itself.
    pub prdcns: C,
    /// Clauses in source order, which must be preserved for first-match semantics.
    pub clauses: Vec<NestedClause>,
    /// The type consumed by case^N or produced by new^N.
    pub ty: Ty,
}
