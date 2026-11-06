//! Detects growing cycles in the constraint graph and reports the path and applied constructors of each one found.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};

use printer::Print;

use crate::{
    mono::{
        constraint_graph::{ConstraintGraph, Edge, Node},
        position::{Position, collect_vars},
    },
    syntax::{Identifier, Ty},
};

/// One hop in a growing-cycle path: the node reached at this point, and every type constructor
/// the edge taken to reach it applied, e.g. `[Box[A]]` for a constraint `Box[A] ⊑ A`. A single
/// edge can apply more than one constructor at once, one per position, e.g. `[Box[A], Bag[B]]`
/// for `[Box[A], Bag[B]] ⊑ [A, B]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleStep {
    pub node: Node,
    pub applied: Vec<Ty>,
}

/// A growing cycle found in the constraint graph: a path of nodes, starting and ending at the
/// same node, along which at least one edge applies a type constructor to a variable whose values
/// flow back into that very variable. Following this path repeatedly would require generating an
/// unboundedly growing family of specializations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrowingCycle {
    pub steps: Vec<CycleStep>,
    /// The declaration type parameters to erase to break this cycle, computed during detection.
    trigger: HashSet<(Identifier, usize)>,
}

impl GrowingCycle {
    /// Returns the nodes in the cycle, in order, without the applied type constructors.
    pub fn nodes(&self) -> Vec<Node> {
        self.steps.iter().map(|step| step.node.clone()).collect()
    }

    /// Returns every declaration type parameter that must be erased to break this specific
    /// cycle, as `(declaration name, zero-based parameter index)` pairs.
    pub fn trigger_params(&self) -> HashSet<(Identifier, usize)> {
        self.trigger.clone()
    }

    /// Returns the head name of every type constructor application found anywhere along the
    /// discovered path, not just the triggering one
    pub fn path_constructors(&self) -> HashSet<Identifier> {
        self.steps
            .iter()
            .flat_map(|step| step.applied.iter())
            .filter_map(|ty| match ty {
                Ty::Decl { name, .. } => Some(name.clone()),
                _ => None,
            })
            .collect()
    }
}

impl fmt::Display for GrowingCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.steps.iter();

        let first = match iter.next() {
            Some(step) => step,
            None => return write!(f, "<empty cycle>"),
        };

        write!(f, "{}", first.node.print_to_string(None))?;

        for step in iter {
            if step.applied.is_empty() {
                write!(f, " -> ")?;
            } else {
                let templates = step
                    .applied
                    .iter()
                    .map(|ty| ty.print_to_string(None))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, " -{}-> ", templates)?;
            }
            write!(f, "{}", step.node.print_to_string(None))?;
        }

        Ok(())
    }
}

/// Searches the constraint graph for all growing cycles.
///
/// An edge closes a growing cycle through one of its source nodes if one of its constructor
/// positions wraps a variable of that node whose values flow back into it from the position's
/// target, including the trivial case of a direct self-loop. The way back is searched on the
/// same graph, only following the one variable of each node that actually carries the values
/// (see [`ConstraintGraph::flows_into`]). A variable sharing the source node without being fed
/// back, like `B` in `[Pair[A, B], B] ⊑ [A, B]`, therefore does not cause the cycle.
///
/// Returns every growing cycle found, each as the full path with the applied type constructors recorded at the hop where they were applied.
/// Returns an empty vector if the graph is safe to solve as-is.
pub fn find_all_growing_cycles(graph: &ConstraintGraph) -> Vec<GrowingCycle> {
    let mut cycles = Vec::new();
    let mut seen_cycle_keys: HashSet<Vec<(Node, Vec<Ty>)>> = HashSet::new();

    for edges in graph.edges.values() {
        for edge in edges {
            for source in edge.source_nodes(&graph.locations) {
                let mut trigger = HashSet::new();
                // the first way back found, from a constructor's target to the variable it wraps
                let mut back_path: Option<Vec<Identifier>> = None;

                for (position, target) in edge.positions.iter().zip(&edge.into) {
                    let Position::Variable {
                        template: Ty::Decl { name, type_args },
                        vars,
                    } = position
                    else {
                        continue;
                    };

                    for var in vars.iter().filter(|var| source.contains(var)) {
                        // does the variable this constructor feeds flow back into `var`?
                        let Some(path) = bfs_path(graph, target, var) else {
                            continue;
                        };
                        back_path.get_or_insert(path);

                        // `var` causes the cycle, so every argument mentioning it is erased
                        for (index, arg) in type_args.args.iter().enumerate() {
                            if collect_vars(arg).contains(var) {
                                trigger.insert((name.clone(), index));
                            }
                        }
                    }
                }

                let Some(back_path) = back_path else {
                    continue;
                };

                let mut steps = vec![
                    CycleStep {
                        node: source.clone(),
                        applied: vec![],
                    },
                    CycleStep {
                        node: edge.into.clone(),
                        applied: edge_applied_templates(edge),
                    },
                ];
                let back_nodes: Vec<Node> = back_path
                    .iter()
                    .map(|var| graph.locations.node_of(var))
                    .collect();
                for window in back_nodes.windows(2) {
                    let (from, to) = (&window[0], &window[1]);
                    let applied = graph
                        .outgoing(from)
                        .iter()
                        .find(|e| &e.into == to)
                        .map(edge_applied_templates)
                        .unwrap_or_default();

                    steps.push(CycleStep {
                        node: to.clone(),
                        applied,
                    });
                }
                let cycle = GrowingCycle { steps, trigger };

                // deduplicate cycles by their canonical (node, applied constructor) sequence
                let key = canonical_cycle_key(&cycle.steps);
                if seen_cycle_keys.insert(key) {
                    cycles.push(cycle);
                }
            }
        }
    }
    cycles
}

/// Performs a breadth-first search from `start` to `target`, following single variables through
/// the graph (see [`ConstraintGraph::flows_into`]), returning the path if one exists. The path is
/// just `[start]` if both are the same.
fn bfs_path(
    graph: &ConstraintGraph,
    start: &Identifier,
    target: &Identifier,
) -> Option<Vec<Identifier>> {
    if start == target {
        return Some(vec![start.clone()]);
    }

    let mut visited: HashSet<Identifier> = HashSet::new();
    let mut parents: HashMap<Identifier, Identifier> = HashMap::new();
    let mut queue: VecDeque<Identifier> = VecDeque::new();

    visited.insert(start.clone());
    queue.push_back(start.clone());

    while let Some(current) = queue.pop_front() {
        for neighbor in graph.flows_into(&current) {
            if &neighbor == target {
                parents.insert(neighbor.clone(), current.clone());
                return Some(reconstruct_path(&parents, start, target));
            }
            if visited.insert(neighbor.clone()) {
                parents.insert(neighbor.clone(), current.clone());
                queue.push_back(neighbor);
            }
        }
    }
    None
}

/// Reconstructs the path from `start` to `target` by following parent
/// pointers backwards, then reversing the result.
fn reconstruct_path(
    parents: &HashMap<Identifier, Identifier>,
    start: &Identifier,
    target: &Identifier,
) -> Vec<Identifier> {
    let mut path = vec![target.clone()];
    let mut current = target.clone();
    while &current != start {
        current = parents[&current].clone();
        path.push(current.clone());
    }
    path.reverse();
    path
}

/// Returns every template among an edge's positions that actually applies a type constructor,
/// i.e. is not simply a bare variable passed through unchanged. A single edge can grow at more
/// than one position at once (e.g. `[Box[A], Bag[B]] ⊑ [A, B]`), so callers erasing a cycle's
/// trigger must erase *all* of these, not just one.
fn edge_applied_templates(edge: &Edge) -> Vec<Ty> {
    edge.positions
        .iter()
        .filter_map(|pos| match pos {
            Position::Variable { template, .. } if !matches!(template, Ty::Var(_)) => {
                Some(template.clone())
            }
            _ => None,
        })
        .collect()
}

/// Computes a canonical key for a growing cycle (independent of the starting node), pairing each
/// node with the constructor applied on the edge used to *leave* it. Including the applied
/// constructors (not just the node sequence) matters because two distinct growing edges can close
/// a cycle over the very same node sequence, e.g. a def's own shared type parameter `C` reached
/// by two different recursive calls that each wrap it in a different constructor (`Box[C] ⊑ C`
/// and `Bag[C] ⊑ C`, both self-loops on node `C`). Deduplicating by node sequence alone would
/// collapse these into a single cycle and silently drop one constructor from
/// [`GrowingCycle::path_constructors`].
///
/// For example, `[A, B, C, A]`/`[B, C, A, B]` with matching applied constructors on every hop
/// yield the same minimal rotated sequence.
fn canonical_cycle_key(steps: &[CycleStep]) -> Vec<(Node, Vec<Ty>)> {
    if steps.len() <= 1 {
        return steps
            .iter()
            .map(|s| (s.node.clone(), s.applied.clone()))
            .collect();
    }

    // The path always starts and ends at the same node (see `find_all_growing_cycles`); drop the
    // duplicated closing node here.
    let elems = if steps.first().map(|s| &s.node) == steps.last().map(|s| &s.node) {
        &steps[..steps.len() - 1]
    } else {
        steps
    };

    if elems.is_empty() {
        return Vec::new();
    }

    let pairs: Vec<(Node, Vec<Ty>)> = elems
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let applied = steps
                .get(i + 1)
                .map(|next| next.applied.clone())
                .unwrap_or_default();
            (step.node.clone(), applied)
        })
        .collect();

    let n = pairs.len();
    let mut min_rotation = pairs.clone();

    for i in 1..n {
        let mut rotated = Vec::with_capacity(n);
        rotated.extend_from_slice(&pairs[i..]);
        rotated.extend_from_slice(&pairs[..i]);
        if rotated < min_rotation {
            min_rotation = rotated;
        }
    }

    min_rotation
}

#[cfg(test)]
mod growing_cycle_tests {

    use super::*;
    use crate::mono::{
        constraint_graph::ConstraintGraph,
        constraints::{FlowConstraint, FlowConstraintSet},
        erasure::{ErasedDecls, erase_constraints},
    };
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn test_no_growing_cycle() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: vec![ty!(id!("List"), [tvar!(id!("A", 1))])],
            to: vec![id!("B", 2)],
        });

        let graph = ConstraintGraph::from(set);
        let result = find_all_growing_cycles(&graph);

        assert!(
            result.is_empty(),
            "graph with no growing cycle incorrectly reported a cycle"
        );
    }

    #[test]
    fn test_direct_growing_cycle() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: vec![ty!(id!("List"), [tvar!(id!("A", 1))])],
            to: vec![id!("A", 1)],
        });

        let graph = ConstraintGraph::from(set);
        let result = find_all_growing_cycles(&graph);

        assert!(!result.is_empty(), "Direct growing cycle was not detected.");

        let path = &result[0];
        let node_a = vec![id!("A", 1)];
        assert_eq!(path.nodes(), vec![node_a.clone(), node_a.clone()]);
    }

    #[test]
    fn test_indirect_growing_cycle() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: vec![ty!(id!("Option"), [tvar!(id!("A", 1))])],
            to: vec![id!("B", 2)],
        });

        set.insert(FlowConstraint {
            from: vec![tvar!(id!("B", 2))],
            to: vec![id!("A", 1)],
        });

        let graph = ConstraintGraph::from(set);
        let result = find_all_growing_cycles(&graph);

        assert!(
            !result.is_empty(),
            "Indirect growing cycle was not detected."
        );

        let path = &result[0];
        let node_a = vec![id!("A", 1)];
        let node_b = vec![id!("B", 2)];

        assert_eq!(path.nodes(), vec![node_a.clone(), node_b, node_a.clone()]);
    }

    #[test]
    fn no_cycle_for_non_growing_self_reference() {
        // A flat self-loop (A ⊑ A, no constructor applied) must not be
        // reported, since it does not require unbounded specializations.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![tvar!(id!("A", 1))],
            vec![id!("A", 1)],
        )));

        let graph = ConstraintGraph::from(set);
        assert!(find_all_growing_cycles(&graph).is_empty());
    }

    #[test]
    fn detects_direct_self_loop_growing_cycle() {
        // Box[A] ⊑ A -- the classic polymorphic-recursion self-loop.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])],
            vec![id!("A", 1)],
        )));

        let graph = ConstraintGraph::from(set);
        let cycles = find_all_growing_cycles(&graph);

        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].path_constructors(), HashSet::from([id!("Box")]));
    }

    #[test]
    fn non_growing_type_in_cycle_is_not_flagged() {
        // Box[C] ⊑ C forms a growing cycle, but List[C] ⊑ C, while also a
        // constructor edge, does not itself close a cycle back into its own
        // source -- only Box should end up as an erasure target.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("C", 6))])],
            vec![id!("C", 6)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!(id!("List"), [ty!(id!("int"))])],
            vec![id!("C", 6)],
        )));

        let graph = ConstraintGraph::from(set);
        let cycles = find_all_growing_cycles(&graph);

        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].path_constructors(), HashSet::from([id!("Box")]));
    }

    #[test]
    fn finds_multiple_independent_growing_cycles() {
        // Two entirely separate self-loops, e.g. Box[A] ⊑ A and Bag[D] ⊑ D,
        // must both be reported.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])],
            vec![id!("A", 1)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Bag"), [tvar!(id!("D", 4))])],
            vec![id!("D", 4)],
        )));

        let graph = ConstraintGraph::from(set);
        let cycles = find_all_growing_cycles(&graph);

        assert_eq!(cycles.len(), 2);
        let all_targets: HashSet<_> = cycles.iter().flat_map(|c| c.path_constructors()).collect();
        assert_eq!(all_targets, HashSet::from([id!("Box"), id!("Bag")]));
    }

    #[test]
    fn two_growing_self_loops_on_the_very_same_node_are_not_deduplicated() {
        // Same identifier `C` used as the target of two different growing self-loops -- e.g. a
        // single def's own shared type parameter, reached via two different recursive calls that
        // each wrap it in a different constructor. Deduplicating by node sequence alone would
        // collapse these into one cycle and silently drop one constructor from
        // `path_constructors()`.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("C", 1))])],
            vec![id!("C", 1)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Bag"), [tvar!(id!("C", 1))])],
            vec![id!("C", 1)],
        )));

        let graph = ConstraintGraph::from(set);
        let cycles = find_all_growing_cycles(&graph);

        assert_eq!(cycles.len(), 2);
        let all_targets: HashSet<_> = cycles.iter().flat_map(|c| c.path_constructors()).collect();
        assert_eq!(all_targets, HashSet::from([id!("Box"), id!("Bag")]));
    }

    /// `P[X] ⊑ Y`, `Q[Y] ⊑ X` -- mutual polymorphic recursion between two *separate*
    /// declarations, each applying exactly one constructor at its own single-position edge,
    /// closing a single two-hop cycle. Erasing either one alone already breaks the cycle (see
    /// `trigger_params`'s doc comment), so `path_constructors` and `trigger_params` must
    /// legitimately disagree here: the former reports both `P` and `Q`, the latter only the one
    /// constructor actually applied by the edge this cycle was discovered from.
    fn mutual_recursion_cycle() -> GrowingCycle {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("P"), [tvar!(id!("X", 1))])],
            vec![id!("Y", 2)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Q"), [tvar!(id!("Y", 2))])],
            vec![id!("X", 1)],
        )));

        let graph = ConstraintGraph::from(set);
        let cycles = find_all_growing_cycles(&graph);
        assert_eq!(
            cycles.len(),
            1,
            "expected exactly one two-hop cycle, got {cycles:?}"
        );
        cycles.into_iter().next().unwrap()
    }

    #[test]
    fn trigger_params_is_only_the_constructor_of_the_discovering_edge() {
        let cycle = mutual_recursion_cycle();

        // both constructors really do sit on the path -- otherwise this test would not be
        // exercising the distinction it claims to
        assert_eq!(
            cycle.path_constructors(),
            HashSet::from([id!("P"), id!("Q")])
        );

        // but only one of them is the trigger: whichever edge `find_all_growing_cycles` was
        // examining when it found this cycle, i.e. steps[1]'s constructor. Here each edge only
        // applies one constructor, so the trigger set has exactly one element.
        let triggers = cycle.trigger_params();
        assert!(
            triggers == HashSet::from([(id!("P"), 0)])
                || triggers == HashSet::from([(id!("Q"), 0)]),
            "expected exactly one of P/Q as the trigger, got {triggers:?}"
        );

        // and erasing just that one is actually sufficient to break the cycle -- the whole point
        let broken_graph = ConstraintGraph::from(erase_constraints(
            &constraints_of_mutual(),
            &ErasedDecls::from(triggers),
        ));
        assert!(
            find_all_growing_cycles(&broken_graph).is_empty(),
            "erasing only the trigger must be sufficient to break the cycle"
        );
    }

    /// Rebuilds the same constraints `mutual_recursion_cycle` used, needed separately because
    /// `erase_constraints` takes the constraint set, not the graph built from it.
    fn constraints_of_mutual() -> FlowConstraintSet {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("P"), [tvar!(id!("X", 1))])],
            vec![id!("Y", 2)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Q"), [tvar!(id!("Y", 2))])],
            vec![id!("X", 1)],
        )));
        set
    }

    #[test]
    fn trigger_params_matches_the_edge_recorded_at_step_one() {
        let cycle = mutual_recursion_cycle();
        let expected: HashSet<(Identifier, usize)> = cycle.steps[1]
            .applied
            .iter()
            .filter_map(|ty| match ty {
                Ty::Decl { name, .. } => Some((name.clone(), 0)),
                _ => None,
            })
            .collect();
        assert_eq!(cycle.trigger_params(), expected);
    }

    #[test]
    fn cycle_step_records_applied_constructor_at_correct_hop() {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])],
            vec![id!("A", 1)],
        )));

        let graph = ConstraintGraph::from(set);
        let cycles = find_all_growing_cycles(&graph);
        assert_eq!(cycles.len(), 1);

        let steps = &cycles[0].steps;
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].applied, vec![]);
        assert_eq!(
            steps[1].applied,
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])]
        );
    }

    #[test]
    fn trigger_params_includes_every_constructor_the_triggering_edge_applies() {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![
                ty!(id!("Box"), [tvar!(id!("A", 1))]),
                ty!(id!("Bag"), [tvar!(id!("B", 2))]),
            ],
            vec![id!("A", 1), id!("B", 2)],
        )));

        let graph = ConstraintGraph::from(set.clone());
        let cycles = find_all_growing_cycles(&graph);
        assert_eq!(cycles.len(), 1);
        assert_eq!(
            cycles[0].trigger_params(),
            HashSet::from([(id!("Box"), 0), (id!("Bag"), 0)]),
            "both constructors applied by the self-looping edge must be triggers"
        );

        let broken_graph = ConstraintGraph::from(erase_constraints(
            &set,
            &ErasedDecls::from(cycles[0].trigger_params()),
        ));
        assert!(
            find_all_growing_cycles(&broken_graph).is_empty(),
            "erasing every trigger found on the edge must fully break the cycle in one pass"
        );

        // sanity check against the pre-fix bug: erasing only the first-found constructor is *not*
        // sufficient, confirming this test would have failed before the fix
        let only_first = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let partially_broken_graph = ConstraintGraph::from(erase_constraints(&set, &only_first));
        assert!(
            !find_all_growing_cycles(&partially_broken_graph).is_empty(),
            "erasing only one of the two constructors must leave the cycle intact -- otherwise \
             this test no longer exercises the bug it targets"
        );
    }

    /// Finds the growing cycles of `set` and returns the union of their trigger parameters.
    fn all_trigger_params(set: &FlowConstraintSet) -> HashSet<(Identifier, usize)> {
        find_all_growing_cycles(&ConstraintGraph::from(set.clone()))
            .iter()
            .flat_map(GrowingCycle::trigger_params)
            .collect()
    }

    #[test]
    fn constructor_wrapping_only_an_off_cycle_variable_is_not_growing() {
        // [Box[C], A] ⊑ [A, B]: the edge loops on [A, B] only through its flat position, while
        // the constructor wraps C, which the cycle never reaches. Values of [A, B] cannot grow.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("C", 3))]), tvar!(id!("A", 1))],
            vec![id!("A", 1), id!("B", 2)],
        )));

        let graph = ConstraintGraph::from(set);
        assert!(find_all_growing_cycles(&graph).is_empty());
    }

    #[test]
    fn trigger_params_skips_a_ground_argument() {
        // Pair[A, i64] ⊑ A grows only through Pair's first parameter.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Pair"), [tvar!(id!("A", 1)), ty!("int")])],
            vec![id!("A", 1)],
        )));

        assert_eq!(all_trigger_params(&set), HashSet::from([(id!("Pair"), 0)]));
    }

    #[test]
    fn trigger_params_skips_an_argument_from_an_off_cycle_node() {
        // Pair[A, B] ⊑ A where B is fed independently: only parameter 0 carries the cycle.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))])],
            vec![id!("A", 1)],
        )));
        set.insert(FlowConstraint::from((vec![ty!("int")], vec![id!("B", 2)])));

        assert_eq!(all_trigger_params(&set), HashSet::from([(id!("Pair"), 0)]));
    }

    #[test]
    fn trigger_params_includes_every_argument_mentioning_the_source() {
        // Pair[A, Box[A]] ⊑ A: both arguments grow with A, so both parameters are triggers, and
        // only the outermost head is selected, not the nested Box.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(
                id!("Pair"),
                [tvar!(id!("A", 1)), ty!(id!("Box"), [tvar!(id!("A", 1))])]
            )],
            vec![id!("A", 1)],
        )));

        assert_eq!(
            all_trigger_params(&set),
            HashSet::from([(id!("Pair"), 0), (id!("Pair"), 1)])
        );
    }

    #[test]
    fn erasing_only_the_trigger_parameter_breaks_the_cycle() {
        // Pair[A, B] ⊑ A, A ⊑ B: B lies on the cycle too, so both parameters are triggers --
        // each found from its own source node -- and erasing them together breaks the cycle.
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))])],
            vec![id!("A", 1)],
        )));
        set.insert(FlowConstraint::from((
            vec![tvar!(id!("A", 1))],
            vec![id!("B", 2)],
        )));

        let triggers = all_trigger_params(&set);
        assert_eq!(
            triggers,
            HashSet::from([(id!("Pair"), 0), (id!("Pair"), 1)])
        );

        let broken_graph =
            ConstraintGraph::from(erase_constraints(&set, &ErasedDecls::from(triggers)));
        assert!(find_all_growing_cycles(&broken_graph).is_empty());
    }

    /// Asserts that `set` grows only through `expected`, and that erasing exactly those
    /// parameters breaks every growing cycle.
    fn assert_triggers_break_cycles(set: &FlowConstraintSet, expected: &[(Identifier, usize)]) {
        let triggers = all_trigger_params(set);
        assert_eq!(triggers, expected.iter().cloned().collect());

        let broken_graph =
            ConstraintGraph::from(erase_constraints(set, &ErasedDecls::from(triggers)));
        assert!(find_all_growing_cycles(&broken_graph).is_empty());
    }

    #[test]
    fn a_variable_flowing_onto_itself_in_a_decl_node_is_not_erased() {
        // Foo[A, B] with field Foo[Pair[A, B], B]: A and B share Foo's node, but only A grows,
        // B merely flows onto itself
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![
                ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))]),
                tvar!(id!("B", 2)),
            ],
            vec![id!("A", 1), id!("B", 2)],
        )));

        assert_triggers_break_cycles(&set, &[(id!("Pair"), 0)]);
    }

    #[test]
    fn a_variable_flowing_onto_itself_in_a_def_node_is_not_erased() {
        // grow[C, L] calling grow[Tag[C, L], L]: the same situation for a def's node
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![
                ty!(id!("Tag"), [tvar!(id!("C", 1)), tvar!(id!("L", 2))]),
                tvar!(id!("L", 2)),
            ],
            vec![id!("C", 1), id!("L", 2)],
        )));

        assert_triggers_break_cycles(&set, &[(id!("Tag"), 0)]);
    }

    #[test]
    fn a_variable_receiving_the_growing_values_is_erased_too() {
        // [Pair[A, B], A] ⊑ [A, B]: B receives A's growing values, so Pair's second argument
        // grows along the cycle as well
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![
                ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))]),
                tvar!(id!("A", 1)),
            ],
            vec![id!("A", 1), id!("B", 2)],
        )));

        assert_triggers_break_cycles(&set, &[(id!("Pair"), 0), (id!("Pair"), 1)]);
    }

    #[test]
    fn flowing_back_into_another_component_of_the_source_node_is_not_growing() {
        // Box[A] ⊑ X, [i64, X] ⊑ [A, B]: the node [A, B] is reached again, but only through B,
        // which never flows back into A, so A's values cannot grow
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])],
            vec![id!("X", 3)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!("int"), tvar!(id!("X", 3))],
            vec![id!("A", 1), id!("B", 2)],
        )));

        assert!(find_all_growing_cycles(&ConstraintGraph::from(set)).is_empty());
    }

    #[test]
    fn reported_path_follows_the_variable_cycle() {
        // Box[A] ⊑ X, [i64, X] ⊑ [A, B] (a shortcut back into node [A, B], but only into B),
        // X ⊑ Y, [Y, i64] ⊑ [A, B]: the actual cycle runs A -> X -> Y -> A, so the reported
        // path must take the detour over Y rather than the node-level shortcut
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint::from((
            vec![ty!(id!("Box"), [tvar!(id!("A", 1))])],
            vec![id!("X", 3)],
        )));
        set.insert(FlowConstraint::from((
            vec![ty!("int"), tvar!(id!("X", 3))],
            vec![id!("A", 1), id!("B", 2)],
        )));
        set.insert(FlowConstraint::from((
            vec![tvar!(id!("X", 3))],
            vec![id!("Y", 4)],
        )));
        set.insert(FlowConstraint::from((
            vec![tvar!(id!("Y", 4)), ty!("int")],
            vec![id!("A", 1), id!("B", 2)],
        )));

        let cycles = find_all_growing_cycles(&ConstraintGraph::from(set));
        assert_eq!(cycles.len(), 1);
        let source = vec![id!("A", 1), id!("B", 2)];
        assert_eq!(
            cycles[0].nodes(),
            vec![source.clone(), vec![id!("X", 3)], vec![id!("Y", 4)], source]
        );
        assert_eq!(cycles[0].trigger_params(), HashSet::from([(id!("Box"), 0)]));
    }
}
