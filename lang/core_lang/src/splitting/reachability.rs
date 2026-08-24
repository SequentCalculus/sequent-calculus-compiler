//! Static reachability of declarations through their own field types, computed once from the raw
//! (unlabeled) program before labeling starts. Distinguishes fields that structurally loop back to
//! their own enclosing declaration directly (`List.Cons.xs: List[A]`) or via mutual recursion
//! (`A.MkA(b: B)`, `B.MkB(a: A)`) from fields naming a genuinely independent declaration.

use std::collections::{HashMap, HashSet};

use crate::syntax::{
    CodataDeclaration, DataDeclaration, Identifier, Prog, Ty,
    declaration::{Polarity, TypeDeclaration},
};

/// Maps a declaration to every declaration reachable from it via field types, transitively.
pub type ReachableFrom = HashMap<Identifier, HashSet<Identifier>>;

/// Every `Ty::Decl` head appearing in `ty`, recursing into type arguments, e.g. `Pair[Foo, Bar]`
/// yields `{Pair, Foo, Bar}`.
fn decl_heads(ty: &Ty, out: &mut HashSet<Identifier>) {
    match ty {
        Ty::I64 | Ty::Var(_) => {}
        Ty::Decl { name, type_args } => {
            out.insert(name.clone());
            for arg in &type_args.args {
                decl_heads(arg, out);
            }
        }
    }
}

fn direct_edges<P: Polarity>(
    decls: &[TypeDeclaration<P>],
) -> HashMap<Identifier, HashSet<Identifier>> {
    let mut edges = HashMap::new();
    for decl in decls {
        let mut heads = HashSet::new();
        for xtor in &decl.xtors {
            for binding in &xtor.args.bindings {
                decl_heads(&binding.ty, &mut heads);
            }
        }
        edges
            .entry(decl.name.clone())
            .or_insert_with(HashSet::new)
            .extend(heads);
    }
    edges
}

/// Computes, for every declaration, the set of declarations reachable from it via one or more
/// field-type edges (data and codata declarations share one graph, since fields may cross between
/// them).
pub fn compute_reachable_decls(
    data: &[DataDeclaration],
    codata: &[CodataDeclaration],
) -> ReachableFrom {
    let mut direct = direct_edges(data);
    for (name, heads) in direct_edges(codata) {
        direct.entry(name).or_default().extend(heads);
    }

    direct
        .keys()
        .map(|origin| {
            let mut visited = HashSet::new();
            let mut stack: Vec<Identifier> = direct[origin].iter().cloned().collect();
            while let Some(next) = stack.pop() {
                if visited.insert(next.clone())
                    && let Some(neighbors) = direct.get(&next)
                {
                    stack.extend(neighbors.iter().cloned());
                }
            }
            (origin.clone(), visited)
        })
        .collect()
}

/// True iff `field_ty` can structurally reach back to `owner`, directly or transitively. A field
/// with no `Ty::Decl` head at all (a bare type variable like `x: D`, or `i64`) is trivially `true`
/// it already flows through the existing type-parameter substitution, not the field-observation
/// mechanism, so there is nothing to gain from treating it as independent.
fn is_self_referential(field_ty: &Ty, owner: &Identifier, reachable: &ReachableFrom) -> bool {
    let mut heads = HashSet::new();
    decl_heads(field_ty, &mut heads);
    heads.is_empty()
        || heads
            .iter()
            .any(|head| head == owner || reachable.get(head).is_some_and(|r| r.contains(owner)))
}

/// For every xtor in the program, whether each of its field positions (parallel to
/// `args.bindings`) structurally loops back to that xtor's own enclosing declaration. Keyed by the
/// xtor's original (pre-split) name.
pub fn compute_field_reachability(prog: &Prog) -> HashMap<Identifier, Vec<bool>> {
    let reachable = compute_reachable_decls(&prog.data_types, &prog.codata_types);
    let mut result = HashMap::new();

    for decl in &prog.data_types {
        for xtor in &decl.xtors {
            let flags = xtor
                .args
                .bindings
                .iter()
                .map(|b| is_self_referential(&b.ty, &decl.name, &reachable))
                .collect();
            result.insert(xtor.name.clone(), flags);
        }
    }
    for decl in &prog.codata_types {
        for xtor in &decl.xtors {
            let flags = xtor
                .args
                .bindings
                .iter()
                .map(|b| is_self_referential(&b.ty, &decl.name, &reachable))
                .collect();
            result.insert(xtor.name.clone(), flags);
        }
    }

    result
}

#[cfg(test)]
mod reachability_tests {
    use super::*;
    extern crate self as core_lang;
    use core_macros::{bind, ctor_sig, data, id, prd, prog, ty};

    fn foo_decl() -> DataDeclaration {
        data!(id!("Foo"), [ctor_sig!(id!("MkFoo"), [], [])], [])
    }

    fn bar_decl() -> DataDeclaration {
        data!(
            id!("Bar"),
            [ctor_sig!(
                id!("MkBar"),
                [],
                [bind!(id!("f"), prd!(), ty!(id!("Foo")))]
            )],
            []
        )
    }

    fn list_decl() -> DataDeclaration {
        data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!(), ty!("int")),
                        bind!(id!("xs"), prd!(), ty!(id!("List")))
                    ]
                )
            ],
            []
        )
    }

    fn a_decl() -> DataDeclaration {
        data!(
            id!("A"),
            [ctor_sig!(
                id!("MkA"),
                [],
                [bind!(id!("b"), prd!(), ty!(id!("B")))]
            )],
            []
        )
    }

    fn b_decl() -> DataDeclaration {
        data!(
            id!("B"),
            [
                ctor_sig!(id!("MkB"), [], [bind!(id!("a"), prd!(), ty!(id!("A")))]),
                ctor_sig!(id!("Leaf"), [], [])
            ],
            []
        )
    }

    #[test]
    fn independent_nested_declaration_is_not_self_referential() {
        let prog = prog!([], [foo_decl(), bar_decl()], []);
        let reachability = compute_field_reachability(&prog);
        assert_eq!(reachability[&id!("MkBar")], vec![false]);
    }

    #[test]
    fn direct_self_reference_is_detected() {
        let prog = prog!([], [list_decl()], []);
        let reachability = compute_field_reachability(&prog);
        // Cons's fields are [x: int, xs: List]: xs loops back directly, x has no Decl head at
        // all and is trivially treated the same way.
        assert_eq!(reachability[&id!("Cons")], vec![true, true]);
    }

    #[test]
    fn self_reference_nested_in_an_unrelated_wrapper_is_still_detected() {
        // A.MkA(x: List[A]): the outer head `List` is unrelated to `A`, but `A` itself appears
        // as a nested type argument, must still be flagged self-referential.
        let a_wraps_itself = data!(
            id!("A"),
            [ctor_sig!(
                id!("MkA"),
                [],
                [bind!(id!("x"), prd!(), ty!(id!("List"), [ty!(id!("A"))]))]
            )],
            []
        );
        let prog = prog!([], [a_wraps_itself, list_decl()], []);
        let reachability = compute_field_reachability(&prog);
        assert_eq!(reachability[&id!("MkA")], vec![true]);
    }

    #[test]
    fn mutual_recursion_is_detected_on_both_sides() {
        let prog = prog!([], [a_decl(), b_decl()], []);
        let reachability = compute_field_reachability(&prog);
        assert_eq!(reachability[&id!("MkA")], vec![true]);
        assert_eq!(reachability[&id!("MkB")], vec![true]);
        assert_eq!(reachability[&id!("Leaf")], vec![]);
    }
}
