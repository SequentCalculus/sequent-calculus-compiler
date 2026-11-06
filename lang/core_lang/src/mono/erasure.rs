//! This module defines metadata tracking which type parameters of which type declarations have
//! been erased in order to break a growing cycle during constraint solving, as part of total
//! monomorphization of polymorphic recursion.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::{
    mono::constraints::{FlowConstraint, FlowConstraintSet},
    syntax::{Identifier, Ty, types::TypeArgs},
};

/// The type parameters of type declarations (data or codata) that have been erased to break a
/// growing cycle in the constraint graph, as a map from declaration name to the zero-based
/// indices of its erased parameters.
///
/// Erasure works per parameter, not per declaration: only the parameters whose argument actually
/// carries a growing cycle are erased.
/// At every constraint position the arguments at erased indices are dropped, the remaining
/// ("kept") ones stay in place, e.g. `Pair[Int, Bool]` with parameter 0 erased becomes
/// `Pair[Bool]`, and `Box[Int]` with its only parameter erased becomes `Box`. The declaration is
/// then duplicated only per instantiation of its kept parameters, while the (now finite) flow
/// into its erased parameters is redirected onto each of its own constructors/destructors,
/// exactly as if those parameters had been declared on the xtor itself. This is the naming
/// table's and specialization's single source of truth for distinguishing an "ordinary" (never
/// erased) declaration from a "widened" one, so the two phases cannot diverge on this point.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ErasedDecls(BTreeMap<Identifier, BTreeSet<usize>>);

impl ErasedDecls {
    /// True iff at least one of `name`'s own declared type parameters was erased to break a
    /// growing cycle.
    pub fn is_erased(&self, name: &Identifier) -> bool {
        self.0.contains_key(name)
    }

    /// Returns the zero-based indices of `name`'s erased type parameters, or `None` if none of
    /// them was erased.
    pub fn erased_params(&self, name: &Identifier) -> Option<&BTreeSet<usize>> {
        self.0.get(name)
    }

    /// Projects `items`, one per declared type parameter of `name`, onto the parameters that
    /// were *not* erased, keeping their order. Returns `items` unchanged if `name` is not erased.
    pub fn kept_args<T: Clone>(&self, name: &Identifier, items: &[T]) -> Vec<T> {
        self.project(name, items, false)
    }

    /// Projects `items`, one per declared type parameter of `name`, onto the erased parameters,
    /// keeping their order. Returns an empty vector if `name` is not erased.
    pub fn erased_args<T: Clone>(&self, name: &Identifier, items: &[T]) -> Vec<T> {
        self.project(name, items, true)
    }

    /// Keeps exactly the elements of `items` whose index is (`erased == true`) or is not
    /// (`erased == false`) among `name`'s erased parameter indices.
    fn project<T: Clone>(&self, name: &Identifier, items: &[T], erased: bool) -> Vec<T> {
        let erased_indices = self.0.get(name);
        items
            .iter()
            .enumerate()
            .filter(|(i, _)| erased_indices.is_some_and(|set| set.contains(i)) == erased)
            .map(|(_, item)| item.clone())
            .collect()
    }

    /// True iff no declaration was erased.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over every erased declaration's name together with the indices of its erased
    /// parameters.
    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &BTreeSet<usize>)> {
        self.0.iter()
    }
}

impl From<HashSet<(Identifier, usize)>> for ErasedDecls {
    fn from(erased: HashSet<(Identifier, usize)>) -> Self {
        let mut map: BTreeMap<Identifier, BTreeSet<usize>> = BTreeMap::new();
        for (name, index) in erased {
            map.entry(name).or_default().insert(index);
        }
        ErasedDecls(map)
    }
}

/// Erases the erased type parameters' arguments of every occurrence of an erased declaration
/// within a constraint set's `from` types, wherever they appear, returning a new, equivalent
/// constraint set. `to` sides are always bare type variables and are left untouched, since there
/// is nothing to erase there.
pub fn erase_constraints(
    constraints: &FlowConstraintSet,
    targets: &ErasedDecls,
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

/// Recursively erases the arguments of erased type parameters within the given type, returning
/// a new type. For a declaration with erased parameters in `targets`, the arguments at the
/// erased indices are dropped entirely, and the kept ones are recursively processed; for any
/// other declaration, all arguments are recursively processed. Primitives and variables are
/// returned unchanged.
pub fn erase_ty(ty: &Ty, targets: &ErasedDecls) -> Ty {
    match ty {
        Ty::I64 => Ty::I64,
        Ty::Var(id) => Ty::Var(id.clone()),
        Ty::Decl { name, type_args } => Ty::Decl {
            name: name.clone(),
            type_args: TypeArgs {
                args: targets
                    .kept_args(name, &type_args.args)
                    .iter()
                    .map(|a| erase_ty(a, targets))
                    .collect(),
            },
        },
    }
}

#[cfg(test)]
mod erasure_tests {
    use std::collections::HashSet;

    use crate::{
        mono::{
            constraints::{FlowConstraint, FlowConstraintSet},
            erasure::{ErasedDecls, erase_constraints, erase_ty},
        },
        syntax::Ty,
    };
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn erase_ty_replaces_type_args_of_target_head() {
        let targets = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let input = ty!(id!("Box"), [tvar!(id!("A", 1))]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, ty!(id!("Box")));
    }

    #[test]
    fn erase_ty_leaves_non_target_heads_untouched() {
        let targets = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let input = ty!(id!("List"), [tvar!(id!("A", 1))]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, input);
    }

    #[test]
    fn erase_ty_recurses_into_non_target_wrapping_a_target() {
        // List[Box[i64]] -- List is not erased, Box is. Only Box's own
        // arguments should be dropped; List's argument list itself, and its
        // nesting of Box, must be preserved.
        let targets = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let input = ty!(id!("List"), [ty!(id!("Box"), [tvar!(id!("A", 1))])]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, ty!(id!("List"), [ty!(id!("Box"))]));
    }

    #[test]
    fn erase_ty_erases_target_nested_inside_target() {
        // Box[Box[i64]], erasing Box: the outer Box loses its args entirely,
        // so the inner Box[i64] disappears along with it.
        let targets = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let input = ty!(id!("Box"), [ty!(id!("Box"), [tvar!(id!("A", 1))])]);
        let result = erase_ty(&input, &targets);
        assert_eq!(result, ty!(id!("Box")));
    }

    #[test]
    fn erase_ty_leaves_primitives_and_vars_untouched() {
        let targets = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        assert_eq!(erase_ty(&Ty::I64, &targets), Ty::I64);
        let var = tvar!(id!("A", 1));
        assert_eq!(erase_ty(&var, &targets), var);
    }

    #[test]
    fn erase_constraints_only_touches_from_side() {
        // Box[A] ⊑ A -- erasing Box must erase the `from` side but never
        // touch the `to` side, which is always a bare variable vector.
        let targets = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
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
        let targets = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
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

    #[test]
    fn erase_ty_drops_only_the_erased_parameter() {
        // Pair[A, i64] with only parameter 0 erased keeps its second argument.
        let targets = ErasedDecls::from(HashSet::from([(id!("Pair"), 0)]));
        let input = ty!(id!("Pair"), [tvar!(id!("A", 1)), ty!("int")]);
        assert_eq!(erase_ty(&input, &targets), ty!(id!("Pair"), [ty!("int")]));
    }

    #[test]
    fn erase_ty_recurses_into_kept_arguments_of_a_partially_erased_head() {
        // Pair[Pair[A, B], C] with parameter 1 erased: the outer Pair drops C, the inner one
        // (a kept argument) drops B, so Pair[Pair[A]] remains.
        let targets = ErasedDecls::from(HashSet::from([(id!("Pair"), 1)]));
        let input = ty!(
            id!("Pair"),
            [
                ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))]),
                tvar!(id!("C", 3))
            ]
        );
        assert_eq!(
            erase_ty(&input, &targets),
            ty!(id!("Pair"), [ty!(id!("Pair"), [tvar!(id!("A", 1))])])
        );
    }
}
