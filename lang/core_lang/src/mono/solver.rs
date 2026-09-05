use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Deref,
};

use printer::{Alloc, Builder, DocAllocator, Print, PrintCfg};

use crate::{
    mono::{
        constraint_graph::{ConstraintGraph, Edge, Node, VarLocations},
        constraints::FlowConstraintSet,
        erasure::{ErasedDecls, erase_constraints},
        errors::MonoError,
        growing_cycle::find_all_growing_cycles,
        position::Position,
    },
    syntax::{Identifier, Ty, types::TypeArgs},
};

/// Maps each [`Node`] to the set of concrete ground vectors it may be instantiated with.
///
/// Each element of the set is a full vector, e.g. `[i64, Bool]` for a `Pair`
/// node `[A, B]`, preserving the correlation between positions. This is the
/// output of the solving phase and the direct input to specialization.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Solution {
    pub map: HashMap<Node, HashSet<Vec<Ty>>>,
}

impl Deref for Solution {
    type Target = HashMap<Node, HashSet<Vec<Ty>>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl From<HashMap<Node, HashSet<Vec<Ty>>>> for Solution {
    fn from(map: HashMap<Node, HashSet<Vec<Ty>>>) -> Self {
        Solution { map }
    }
}

impl Print for Solution {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        if self.map.is_empty() {
            return alloc.text("{}");
        }

        let mut sorted_entries: Vec<(&Node, &HashSet<Vec<Ty>>)> = self.map.iter().collect();
        sorted_entries.sort_by(|a, b| a.0.cmp(b.0));

        let body = alloc.intersperse(
            sorted_entries.into_iter().map(|(node, ty_sets)| {
                let mut sorted_solutions: Vec<&Vec<Ty>> = ty_sets.iter().collect();
                sorted_solutions.sort();

                let solutions_disp = alloc.intersperse(
                    sorted_solutions.into_iter().map(|ty_vec| {
                        let tys = alloc.intersperse(
                            ty_vec.iter().map(|ty| ty.print(cfg, alloc)),
                            alloc.text(", "),
                        );
                        alloc.text("[").append(tys).append(alloc.text("]"))
                    }),
                    alloc.text(", "),
                );

                let mut inline_cfg = cfg.clone();
                inline_cfg.allow_linebreaks = false;

                let key_disp = alloc
                    .text("[")
                    .append(node.print(&inline_cfg, alloc))
                    .append(alloc.text("]"));

                key_disp.append(alloc.text(" -> ")).append(solutions_disp)
            }),
            alloc.line(),
        );

        alloc
            .text("{")
            .append(alloc.line().append(body).nest(cfg.indent))
            .append(alloc.line())
            .append(alloc.text("}"))
            .group()
    }
}

/// Runs the worklist fixpoint solver over the constraint graph.
/// Returns a [`Solution`] mapping each node to the set of concrete ground vectors it may be instantiated with.
/// In case of a growing cycle, returns a [`MonoError::PolymorphicRecursion`] with the detected cycles.
pub fn solve(graph: &ConstraintGraph) -> Result<Solution, MonoError> {
    if let Some(growing_cycles) = find_all_growing_cycles(graph).into_iter().next() {
        return Err(MonoError::PolymorphicRecursion {
            cycles: vec![growing_cycles.clone()],
        });
    };

    Ok(fixpoint_solve(graph))
}

/// Performs total monomorphization: detects every growing cycle in the constraint set in one
/// pass and erases the type parameters of every declaration responsible for one, then computes
/// the fixpoint solution over the resulting (necessarily acyclic-in-growth) constraint graph.
///
/// Unlike [`solve`], this function always succeeds, including for programs with polymorphic
/// recursion of any kind. A single erasure pass suffices: `erase_ty` flattens *every* occurrence
/// of a targeted declaration's head to its bare, argument-less name in one step, so a single edge
/// that grows at several positions at once (e.g. `[Box[A], Bag[B]] ⊑ [A, B]`) is fully de-grown as
/// long as every one of its own growing heads is included in `targets`, not just the first.
/// [`GrowingCycle::trigger_targets`] collects all of them for exactly this reason. Beyond the
/// triggering edge itself, erasure also never adds structure elsewhere in the graph (it only ever
/// removes type arguments), so it can only remove growing edges, never introduce new ones.
pub fn solve_with_erasure(
    constraints: FlowConstraintSet,
) -> (Solution, ErasedDecls, FlowConstraintSet) {
    let graph = ConstraintGraph::from(constraints.clone());
    let cycles = find_all_growing_cycles(&graph);

    if cycles.is_empty() {
        return (
            fixpoint_solve(&graph),
            ErasedDecls::default(),
            FlowConstraintSet::default(),
        );
    }

    let targets: HashSet<_> = cycles.iter().flat_map(|c| c.trigger_targets()).collect();
    let erased = ErasedDecls(targets.clone());

    let erased_constraints = erase_constraints(&constraints, &targets);
    let erased_graph = ConstraintGraph::from(erased_constraints.clone());

    debug_assert!(
        find_all_growing_cycles(&erased_graph).is_empty(),
        "single-pass erasure did not eliminate all growing cycles -- this indicates a bug in \
         growing-cycle detection or erasure, since erasure should only ever remove structure"
    );

    (fixpoint_solve(&erased_graph), erased, erased_constraints)
}

/// Runs the worklist fixpoint solver over the constraint graph, assuming it is already free of
/// growing cycles (either because it never had any, or because [`solve_with_erasure`] erased
/// them).
fn fixpoint_solve(graph: &ConstraintGraph) -> Solution {
    let mut solution: HashMap<Node, HashSet<Vec<Ty>>> = graph
        .nodes
        .iter()
        .map(|n| (n.clone(), HashSet::new()))
        .collect();

    for (node, seed_tuples) in &graph.seeds {
        solution
            .entry(node.clone())
            .or_default()
            .extend(seed_tuples.iter().cloned());
    }

    let mut in_worklist: HashSet<Node> = graph.seeds.keys().cloned().collect();
    let mut worklist: VecDeque<Node> = in_worklist.iter().cloned().collect();

    while let Some(changed) = worklist.pop_front() {
        in_worklist.remove(&changed);

        for edge in graph.outgoing(&changed) {
            let new_tuples = propagate(edge, &graph.locations, &solution);
            if new_tuples.is_empty() {
                continue;
            }

            let target = solution.entry(edge.into.clone()).or_default();
            let before = target.len();
            target.extend(new_tuples);

            if target.len() > before && in_worklist.insert(edge.into.clone()) {
                worklist.push_back(edge.into.clone());
            }
        }
    }

    solution.into()
}

/// Computes the new vector that flow through a single edge given the
/// current solution.
///
/// Builds the Cartesian product of the vector sets of all distinct source
/// nodes the edge depends on, then resolves each position of the edge
/// against the chosen combination. When the edge depends on exactly one
/// source node - including when several positions reference different
/// indices of that *same* node, as in `Pair[A, B]` - the "product" is just
/// an iteration over that node's own vectors: no spurious combinations are
/// introduced, because the correlation between positions is already baked
/// into each vector stored for that node.
pub fn propagate(
    edge: &Edge,
    locations: &VarLocations,
    solution: &HashMap<Node, HashSet<Vec<Ty>>>,
) -> HashSet<Vec<Ty>> {
    let source_nodes = edge.source_nodes(locations);
    if source_nodes.is_empty() {
        // An edge with no variable-dependent positions would have been
        // classified as a seed during graph construction, so this should
        // not occur in practice.
        return HashSet::new();
    }

    // Build the Cartesian product of the current vector sets of all distinct
    // source nodes. `combinations[k]` is a vector with one chosen vector per
    // source node, in the same order as `source_nodes`.
    let mut combinations: Vec<Vec<Vec<Ty>>> = vec![vec![]];
    for node in &source_nodes {
        let vectors = solution.get(node).cloned().unwrap_or_default();
        let mut next = Vec::with_capacity(combinations.len() * vectors.len().max(1));
        for partial in &combinations {
            for vector in &vectors {
                let mut extended = partial.clone();
                extended.push(vector.clone());
                next.push(extended);
            }
        }
        combinations = next;
    }

    let mut results = HashSet::new();
    for combo in &combinations {
        let resolved: Vec<Ty> = edge
            .positions
            .iter()
            .map(|pos| resolve_position(pos, &source_nodes, combo, locations))
            .collect();
        results.insert(resolved);
    }
    results
}

/// Resolves a single position to a concrete type, given one chosen
/// combination of source vectors.
///
/// `combo[k]` is the vector chosen for `source_nodes[k]`; this function finds
/// which node a variable position belongs to, looks up the corresponding
/// vector in `combo`, and reads off the component at the variable's index.
fn resolve_position(
    pos: &Position,
    source_nodes: &[Node],
    combo: &[Vec<Ty>],
    locations: &VarLocations,
) -> Ty {
    match pos {
        Position::Ground(ty) => ty.clone(),
        Position::Variable { template, vars } => {
            let resolved: HashMap<&Identifier, &Ty> = vars
                .iter()
                .map(|v| (v, lookup_value(v, source_nodes, combo, locations)))
                .collect();
            substitute(template, &resolved)
        }
    }
}

/// Substitutes every occurrence of a variable in `ty` with its resolved
/// value from `resolved`. Variables not present in `resolved` are left
/// unchanged (should not occur in practice once all positions are classified
/// and their variables located).
fn substitute(ty: &Ty, resolved: &HashMap<&Identifier, &Ty>) -> Ty {
    match ty {
        Ty::I64 => Ty::I64,
        Ty::Var(id) => resolved
            .get(id)
            .map(|t| (*t).clone())
            .unwrap_or_else(|| ty.clone()),
        Ty::Decl { name, type_args } => Ty::Decl {
            name: name.clone(),
            type_args: TypeArgs {
                args: type_args
                    .args
                    .iter()
                    .map(|a| substitute(a, resolved))
                    .collect(),
            },
        },
    }
}

/// Looks up the resolved value of a variable within the chosen combination
/// of source vectors.
fn lookup_value<'a>(
    var: &Identifier,
    source_nodes: &[Node],
    combo: &'a [Vec<Ty>],
    locations: &VarLocations,
) -> &'a Ty {
    let owning_node = locations.node_of(var);
    let node_position = source_nodes
        .iter()
        .position(|n| n == &owning_node)
        .expect("owning node must be among the edge's source nodes");
    let index = locations.index_of(var);
    &combo[node_position][index]
}

#[cfg(test)]
mod solve_tests {
    use super::*;
    use crate::mono::{
        constraint_graph::ConstraintGraph,
        constraints::{FlowConstraint, FlowConstraintSet},
        errors::MonoError,
    };
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn test_solve_basic_propagation() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("A", 1)],
        });

        set.insert(FlowConstraint {
            from: vec![tvar!(id!("A", 1))],
            to: vec![id!("B", 2)],
        });

        let graph = ConstraintGraph::from(set);
        let result = solve(&graph);

        assert!(result.is_ok(), "Solver failed with error: {:?}", result);
        let solution = result.unwrap();

        let node_a = vec![id!("A", 1)];
        let node_b = vec![id!("B", 2)];

        let expected_ty_vector = vec![ty!("int")];

        assert!(solution[&node_a].contains(&expected_ty_vector));
        assert!(solution[&node_b].contains(&expected_ty_vector));
    }

    #[test]
    fn test_solve_cartesian_product_and_constructor() {
        let mut set = FlowConstraintSet::new();

        // Provide multiple types for A (Node 1)
        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("A", 1)],
        });
        set.insert(FlowConstraint {
            from: vec![ty!(id!("float"))],
            to: vec![id!("A", 1)],
        });

        // Provide multiple types for B (Node 2)
        set.insert(FlowConstraint {
            from: vec![ty!(id!("bool"))],
            to: vec![id!("B", 2)],
        });
        set.insert(FlowConstraint {
            from: vec![ty!(id!("string"))],
            to: vec![id!("B", 2)],
        });

        // Combine them into Pair[A, B] -> C
        set.insert(FlowConstraint {
            from: vec![ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))])],
            to: vec![id!("C", 3)],
        });

        let graph = ConstraintGraph::from(set);
        let result = solve(&graph);

        assert!(result.is_ok());
        let solution = result.unwrap();

        let node_c = vec![id!("C", 3)];

        // We expect exactly 4 combinations (2 * 2) due to the Cartesian product
        let expected_1 = vec![ty!(id!("Pair"), [ty!("int"), ty!(id!("bool"))])];
        let expected_2 = vec![ty!(id!("Pair"), [ty!("int"), ty!(id!("string"))])];
        let expected_3 = vec![ty!(id!("Pair"), [ty!(id!("float")), ty!(id!("bool"))])];
        let expected_4 = vec![ty!(id!("Pair"), [ty!(id!("float")), ty!(id!("string"))])];

        let solver_solutions_for_c = &solution[&node_c];

        assert_eq!(solver_solutions_for_c.len(), 4);
        assert!(solver_solutions_for_c.contains(&expected_1));
        assert!(solver_solutions_for_c.contains(&expected_2));
        assert!(solver_solutions_for_c.contains(&expected_3));
        assert!(solver_solutions_for_c.contains(&expected_4));
    }

    #[test]
    fn test_solve_detects_growing_cycle() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: vec![ty!(id!("List"), [tvar!(id!("A", 1))])],
            to: vec![id!("A", 1)],
        });

        let graph = ConstraintGraph::from(set);
        let result = solve(&graph);

        assert!(result.is_err(),);

        let err = result.unwrap_err();
        assert!(
            matches!(err, MonoError::PolymorphicRecursion { .. }),
            "Expected MonoError::PolymorphicRecursion, got: {:?}",
            err
        );

        if let MonoError::PolymorphicRecursion { cycles } = err {
            let node_a = vec![id!("A", 1)];
            assert_eq!(
                cycles.iter().flat_map(|c| c.nodes()).collect::<Vec<_>>(),
                vec![node_a.clone(), node_a.clone()],
                "Expected cycle to contain only node A, got: {:?}",
                cycles
            );
        }
    }
}

#[cfg(test)]
mod solve_with_erasure_tests {
    use super::*;
    use crate::mono::constraints::{FlowConstraint, FlowConstraintSet};
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn solve_with_erasure_matches_plain_solve_when_acyclic() {
        // For a program without polymorphic recursion, solve_with_erasure must produce the
        // exact same solution as the ordinary solver, and erase nothing.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((vec![ty!("int")], vec![id!("A", 1)])));
        set.insert(FlowConstraint::from((
            vec![tvar!(id!("A", 1))],
            vec![id!("B", 2)],
        )));

        let graph = ConstraintGraph::from(set.clone());
        let plain = solve(&graph).unwrap();

        let (with_erasure, erased, _) = solve_with_erasure(set);

        assert_eq!(plain, with_erasure);
        assert!(erased.0.is_empty());
    }

    #[test]
    fn solve_with_erasure_terminates_on_direct_polymorphic_recursion() {
        // Box[A] ⊑ A, i64 ⊑ A -- the classic case that would otherwise diverge.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])],
            vec![id!("A", 1)],
        )));
        set.insert(FlowConstraint::from((vec![ty!("int")], vec![id!("A", 1)])));

        let (solution, erased, _) = solve_with_erasure(set);

        assert_eq!(erased.0, HashSet::from([id!("Box")]));

        let node = vec![id!("A", 1)];
        let sols = solution
            .map
            .get(&node)
            .expect("node A must have a solution");
        assert!(sols.contains(&vec![ty!("int")]));
        assert!(sols.contains(&vec![ty!(id!("Box"))]));
        // No unbounded nesting must survive: exactly these two, not
        // Box[Box[...]] or similar.
        assert_eq!(sols.len(), 2);
    }

    #[test]
    fn solve_with_erasure_only_erases_the_declaration_causing_the_cycle() {
        // Box[C] ⊑ C forms a growing cycle; List[i64] ⊑ C and List[Box[i64]] ⊑ C do not.
        // Only Box should be erased; List's own structure must survive fully.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("C", 6))])],
            vec![id!("C", 6)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!(id!("List"), [ty!("int")])],
            vec![id!("C", 6)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!(id!("List"), [ty!(id!("Box"), [ty!("int")])])],
            vec![id!("C", 6)],
        )));

        let (solution, erased, _) = solve_with_erasure(set);

        assert_eq!(erased.0, HashSet::from([id!("Box")]));

        let node = vec![id!("C", 6)];
        let sols = solution.map.get(&node).unwrap();
        // List itself keeps its full, un-erased type argument.
        assert!(sols.contains(&vec![ty!(id!("List"), [ty!("int")])]));
        // The nested Box[i64] inside List's argument has been erased to Box.
        assert!(sols.contains(&vec![ty!(id!("List"), [ty!(id!("Box"))])]));
    }

    #[test]
    fn solve_with_erasure_handles_deeply_nested_polymorphic_recursion() {
        // Box[Box[A]] ⊑ A -- growth by two levels per iteration instead of one; erasure must
        // still terminate and produce a finite solution.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [ty!(id!("Box"), [tvar!(id!("A", 1))])])],
            vec![id!("A", 1)],
        )));
        set.insert(FlowConstraint::from((vec![ty!("int")], vec![id!("A", 1)])));

        let (solution, erased, _) = solve_with_erasure(set);

        assert_eq!(erased.0, HashSet::from([id!("Box")]));
        let node = vec![id!("A", 1)];
        assert!(solution.map.get(&node).unwrap().len() <= 2);
    }
}
