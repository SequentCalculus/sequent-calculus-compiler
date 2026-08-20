use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};

use printer::Print;

use crate::{
    mono::{
        constraint_graph::{ConstraintGraph, Edge, Node},
        position::Position,
    },
    syntax::{Identifier, Ty},
};

/// One hop in a growing-cycle path: the node reached at this point, and, if the edge taken to
/// reach it applied a type constructor (rather than simply passing a type through unchanged),
/// the concrete template `Ty` that was applied, e.g. `Box[A]` for a constraint `Box[A] ⊑ A`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleStep {
    pub node: Node,
    pub applied: Option<Ty>,
}

/// A growing cycle found in the constraint graph: a path of nodes, starting and ending at the
/// same node, along which at least one edge applies a type constructor. Following this path
/// repeatedly would require generating an unboundedly growing family of specializations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrowingCycle {
    pub steps: Vec<CycleStep>,
}

impl GrowingCycle {
    /// Returns the nodes in the cycle, in order, without the applied type constructors.
    pub fn nodes(&self) -> Vec<Node> {
        self.steps.iter().map(|step| step.node.clone()).collect()
    }

    /// Returns the set of declaration names that must be erased to break this cycle: the head
    /// name of every type constructor application found along the path.
    pub fn erasure_targets(&self) -> HashSet<Identifier> {
        self.steps
            .iter()
            .filter_map(|step| step.applied.as_ref())
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
            if let Some(ref ty) = step.applied {
                write!(f, " -{}-> ", ty.print_to_string(None))?;
            } else {
                write!(f, " -> ")?;
            }
            write!(f, "{}", step.node.print_to_string(None))?;
        }

        Ok(())
    }
}

/// Searches the constraint graph for all growing cycles.
///
/// An edge is "growing" if it applies a type constructor at some position.
/// A growing cycle exists if, for some growing edge, its target node can
/// reach back to one of its own source nodes via any path in the graph,
/// including the trivial case of a direct self-loop, where the edge's source and target are the very same node.
///
/// Returns the full path with the applied type constructor recorded at the hop where it was
/// applied, or `None` if the graph is safe to solve as-is.
pub fn find_all_growing_cycles(graph: &ConstraintGraph) -> Vec<GrowingCycle> {
    let mut cycles = Vec::new();
    let mut seen_cycle_keys: HashSet<Vec<(Node, Option<Ty>)>> = HashSet::new();

    for edges in graph.edges.values() {
        for edge in edges {
            if !edge.has_constructor_position() {
                continue;
            }
            for source in edge.source_nodes(&graph.locations) {
                if let Some(rest_of_path) = bfs_path(graph, &edge.into, &source) {
                    let mut steps = vec![CycleStep {
                        node: source.clone(),
                        applied: None,
                    }];

                    steps.push(CycleStep {
                        node: edge.into.clone(),
                        applied: edge_applied_template(edge),
                    });

                    for window in rest_of_path.windows(2) {
                        let (from, to) = (&window[0], &window[1]);
                        let applied = graph
                            .outgoing(from)
                            .iter()
                            .find(|e| &e.into == to)
                            .and_then(edge_applied_template);

                        steps.push(CycleStep {
                            node: to.clone(),
                            applied,
                        });
                    }
                    let cycle = GrowingCycle { steps };

                    // deduplicate cycles by their canonical (node, applied constructor) sequence
                    let key = canonical_cycle_key(&cycle.steps);
                    if seen_cycle_keys.insert(key) {
                        cycles.push(cycle);
                    }
                }
            }
        }
    }
    cycles
}

/// Performs a breadth-first search from `start` to `target` over the graph's
/// edges, returning the path if one exists.
fn bfs_path(graph: &ConstraintGraph, start: &Node, target: &Node) -> Option<Vec<Node>> {
    if start == target {
        return Some(vec![start.clone()]);
    }

    let mut visited: HashSet<Node> = HashSet::new();
    let mut parents: HashMap<Node, Node> = HashMap::new();
    let mut queue: VecDeque<Node> = VecDeque::new();

    visited.insert(start.clone());
    queue.push_back(start.clone());

    while let Some(current) = queue.pop_front() {
        for edge in graph.outgoing(&current) {
            let neighbor = &edge.into;
            if neighbor == target {
                parents.insert(neighbor.clone(), current.clone());
                return Some(reconstruct_path(&parents, start, target));
            }
            if !visited.contains(neighbor) {
                visited.insert(neighbor.clone());
                parents.insert(neighbor.clone(), current.clone());
                queue.push_back(neighbor.clone());
            }
        }
    }
    None
}

/// Reconstructs the path from `start` to `target` by following parent
/// pointers backwards, then reversing the result.
fn reconstruct_path(parents: &HashMap<Node, Node>, start: &Node, target: &Node) -> Vec<Node> {
    let mut path = vec![target.clone()];
    let mut current = target.clone();
    while &current != start {
        current = parents[&current].clone();
        path.push(current.clone());
    }
    path.reverse();
    path
}

/// Returns the first template among an edge's positions that actually applies a type
/// constructor, i.e. is not simply a bare variable passed through unchanged.
fn edge_applied_template(edge: &Edge) -> Option<Ty> {
    edge.positions.iter().find_map(|pos| match pos {
        Position::Variable { template, .. } if !matches!(template, Ty::Var(_)) => {
            Some(template.clone())
        }
        _ => None,
    })
}

/// Computes a canonical key for a growing cycle (independent of the starting node), pairing each
/// node with the constructor applied on the edge used to *leave* it. Including the applied
/// constructors (not just the node sequence) matters because two distinct growing edges can close
/// a cycle over the very same node sequence, e.g. a def's own shared type parameter `C` reached
/// by two different recursive calls that each wrap it in a different constructor (`Box[C] ⊑ C`
/// and `Bag[C] ⊑ C`, both self-loops on node `C`). Deduplicating by node sequence alone would
/// collapse these into a single cycle and silently drop one constructor from
/// [`GrowingCycle::erasure_targets`].
///
/// For example, `[A, B, C, A]`/`[B, C, A, B]` with matching applied constructors on every hop
/// yield the same minimal rotated sequence.
fn canonical_cycle_key(steps: &[CycleStep]) -> Vec<(Node, Option<Ty>)> {
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

    let pairs: Vec<(Node, Option<Ty>)> = elems
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let applied = steps.get(i + 1).and_then(|next| next.applied.clone());
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
        assert_eq!(cycles[0].erasure_targets(), HashSet::from([id!("Box")]));
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
        assert_eq!(cycles[0].erasure_targets(), HashSet::from([id!("Box")]));
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
        let all_targets: HashSet<_> = cycles.iter().flat_map(|c| c.erasure_targets()).collect();
        assert_eq!(all_targets, HashSet::from([id!("Box"), id!("Bag")]));
    }

    #[test]
    fn two_growing_self_loops_on_the_very_same_node_are_not_deduplicated() {
        // Same identifier `C` used as the target of two different growing self-loops -- e.g. a
        // single def's own shared type parameter, reached via two different recursive calls that
        // each wrap it in a different constructor. Deduplicating by node sequence alone would
        // collapse these into one cycle and silently drop one constructor from
        // `erasure_targets()`.
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
        let all_targets: HashSet<_> = cycles.iter().flat_map(|c| c.erasure_targets()).collect();
        assert_eq!(all_targets, HashSet::from([id!("Box"), id!("Bag")]));
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
        assert_eq!(steps[0].applied, None);
        assert_eq!(
            steps[1].applied,
            Some(ty!(id!("Box"), [tvar!(id!("A", 1))]))
        );
    }
}
