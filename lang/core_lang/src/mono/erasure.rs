//! This module defines metadata tracking which type declarations have had their own type
//! parameters erased in order to break a growing cycle during constraint solving, as part of
//! total monomorphization of polymorphic recursion.

use std::collections::HashSet;

use crate::{
    mono::constraints::{FlowConstraint, FlowConstraintSet},
    syntax::{Identifier, Ty, types::TypeArgs},
};

/// The set of type declarations (data or codata) whose own declared type parameters have been
/// erased to break a growing cycle in the constraint graph.
///
/// For an erased declaration, its own type arguments are dropped to just its head name at every
/// constraint position (e.g. `Box[Int]` becomes `Box`). Consequently the declaration itself is
/// never duplicated during specialization; instead, the (now finite) flow into its own type
/// parameters is redirected onto each of its own constructors/destructors, exactly as if that
/// parameter had been declared on the xtor itself. This is the naming table's and
/// specialization's single source of truth for distinguishing an "ordinary" (never erased)
/// declaration from a "widened" one, so the two phases cannot diverge on this point.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ErasedDecls(pub HashSet<Identifier>);

impl ErasedDecls {
    pub fn is_erased(&self, name: &Identifier) -> bool {
        self.0.contains(name)
    }
}

/// Erases the type arguments of every occurrence of any of the given declaration names within a
/// constraint set's `from` types, wherever they appear returning a new, equivalent constraint set.
/// `to` sides are always bare type variables and are left untouched, since there is nothing to erase there.
pub fn erase_constraints(
    constraints: &FlowConstraintSet,
    targets: &HashSet<Identifier>,
) -> FlowConstraintSet {
    let mut result = FlowConstraintSet::new();
    for constraint in &constraints.constraints {
        result.insert(FlowConstraint::from((
            constraint
                .from
                .iter()
                .map(|ty| erase_ty(ty, targets))
                .collect(),
            constraint.to.clone(),
        )));
    }
    result
}

/// Recursively erases the type arguments of any occurrence of a declaration name in `targets`
/// within the given type, returning a new type. If the type is a declaration whose name is in `targets`, its type arguments are dropped to an empty vector; otherwise, the type is returned unchanged, except that any nested type arguments are recursively processed.
pub fn erase_ty(ty: &Ty, targets: &HashSet<Identifier>) -> Ty {
    match ty {
        Ty::I64 => Ty::I64,
        Ty::Var(id) => Ty::Var(id.clone()),
        Ty::Decl { name, type_args } => {
            if targets.contains(name) {
                Ty::Decl {
                    name: name.clone(),
                    type_args: TypeArgs::default(),
                }
            } else {
                Ty::Decl {
                    name: name.clone(),
                    type_args: TypeArgs {
                        args: type_args
                            .args
                            .iter()
                            .map(|a| erase_ty(a, targets))
                            .collect(),
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod erasure_tests {
    use std::collections::HashSet;

    use crate::{
        mono::{
            constraints::{FlowConstraint, FlowConstraintSet},
            erasure::{erase_constraints, erase_ty},
        },
        syntax::Ty,
    };
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn erase_ty_replaces_type_args_of_target_head() {
        let targets = HashSet::from([id!("Box")]);
        let input = ty!(id!("Box"), [tvar!(id!("A", 1))]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, ty!(id!("Box")));
    }

    #[test]
    fn erase_ty_leaves_non_target_heads_untouched() {
        let targets = HashSet::from([id!("Box")]);
        let input = ty!(id!("List"), [tvar!(id!("A", 1))]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, input);
    }

    #[test]
    fn erase_ty_recurses_into_non_target_wrapping_a_target() {
        // List[Box[i64]] -- List is not erased, Box is. Only Box's own
        // arguments should be dropped; List's argument list itself, and its
        // nesting of Box, must be preserved.
        let targets = HashSet::from([id!("Box")]);
        let input = ty!(id!("List"), [ty!(id!("Box"), [tvar!(id!("A", 1))])]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, ty!(id!("List"), [ty!(id!("Box"))]));
    }

    #[test]
    fn erase_ty_erases_target_nested_inside_target() {
        // Box[Box[i64]], erasing Box: the outer Box loses its args entirely,
        // so the inner Box[i64] disappears along with it.
        let targets = HashSet::from([id!("Box")]);
        let input = ty!(id!("Box"), [ty!(id!("Box"), [tvar!(id!("A", 1))])]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, ty!(id!("Box")));
    }

    #[test]
    fn erase_ty_leaves_primitives_and_vars_untouched() {
        let targets = HashSet::from([id!("Box")]);
        assert_eq!(erase_ty(&Ty::I64, &targets), Ty::I64);
        let var = tvar!(id!("A", 1));
        assert_eq!(erase_ty(&var, &targets), var);
    }

    #[test]
    fn erase_constraints_only_touches_from_side() {
        // Box[A] ⊑ A -- erasing Box must erase the `from` side but never
        // touch the `to` side, which is always a bare variable vector.
        let targets = HashSet::from([id!("Box")]);
        let mut input = FlowConstraintSet::new();
        input.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])],
            vec![id!("A", 1)],
        )));

        let result = erase_constraints(&input, &targets);

        let expected = FlowConstraint::from((vec![ty!(id!("Box"))], vec![id!("A", 1)]));
        assert!(result.constraints.contains(&expected));
        assert_eq!(result.constraints.len(), 1);
    }

    #[test]
    fn erase_constraints_leaves_unrelated_constraints_unchanged() {
        let targets = HashSet::from([id!("Box")]);
        let mut input = FlowConstraintSet::new();
        input.insert(FlowConstraint::from((
            vec![ty!(id!("int"))],
            vec![id!("A", 1)],
        )));
        input.insert(FlowConstraint::from((
            vec![ty!(id!("List"), [tvar!(id!("A", 1))])],
            vec![id!("A", 1)],
        )));

        let result = erase_constraints(&input, &targets);

        assert_eq!(result, input);
    }
}
