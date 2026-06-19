use std::collections::HashSet;

use crate::{
    mono::constraint_graph::is_ground,
    syntax::{Identifier, Ty},
};

/// One position within a constraint's `from` vector, classified for graph
/// construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    /// The position is already a concrete ground type, independent of any
    /// variable's solution.
    Ground(Ty),
    /// The position contains one or more type variables, nested arbitrarily
    /// deep. `template` is the original type with variables still in place,
    /// e.g. `Pair[A, B]` or `Box[List[A]]`. `vars` lists every distinct
    /// variable appearing in it, in first-occurrence order. At resolution
    /// time, each occurrence in `template` is substituted by its resolved
    /// ground value.
    Variable { template: Ty, vars: Vec<Identifier> },
}

impl Position {
    /// Classifies a single type into a [`Position`].
    pub fn classify(ty: &Ty) -> Position {
        if is_ground(ty) {
            return Position::Ground(ty.clone());
        }
        Position::Variable {
            template: ty.clone(),
            vars: collect_vars(ty),
        }
    }

    /// Returns all variables this position depends on.
    pub fn vars(&self) -> &[Identifier] {
        match self {
            Position::Ground(_) => &[],
            Position::Variable { vars, .. } => vars,
        }
    }

    /// Returns the original type this position represents, whether ground
    /// or still containing variables. Used for display purposes.
    pub fn as_ty(&self) -> &Ty {
        match self {
            Position::Ground(ty) => ty,
            Position::Variable { template, .. } => template,
        }
    }
}

/// Recursively collects every distinct type variable appearing anywhere in
/// the given type, in first-occurrence (pre-order) order.
///
/// Handles arbitrary nesting, e.g. `Box[List[A]]` correctly yields `[A]`,
/// and a multi-argument declaration like `Pair[A, B]` yields `[A, B]`.
fn collect_vars(ty: &Ty) -> Vec<Identifier> {
    let mut vars = Vec::new();
    let mut seen = HashSet::new();
    collect_vars_into(ty, &mut vars, &mut seen);
    vars
}

fn collect_vars_into(ty: &Ty, vars: &mut Vec<Identifier>, seen: &mut HashSet<Identifier>) {
    match ty {
        Ty::I64 => {}
        Ty::Var(id) => {
            if seen.insert(id.clone()) {
                vars.push(id.clone());
            }
        }
        Ty::Decl { type_args, .. } => {
            for arg in &type_args.args {
                collect_vars_into(arg, vars, seen);
            }
        }
    }
}
