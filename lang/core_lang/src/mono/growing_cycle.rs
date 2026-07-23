use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Display, Formatter},
};

use printer::Print;

use crate::{
    mono::{
        constraint_graph::{ConstraintGraph, Edge, Node},
        position::Position,
    },
    syntax::Ty,
};

/// One hop in a growing-cycle path: the node reached at this point, and, if the edge taken to
/// reach it applied a type constructor (rather than simply passing a type through unchanged),
/// the concrete template `Ty` that was applied, e.g. `Box[A]` for a constraint `Box[A] ⊑ A`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleStep {
    pub node: Node,
    pub applied: Option<Ty>,
}

impl Display for CycleStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node.print_to_string(None))
    }
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

/// Searches the constraint graph for a growing cycle.
///
/// An edge is "growing" if it applies a type constructor at some position.
/// A growing cycle exists if, for some growing edge, its target node can
/// reach back to one of its own source nodes via any path in the graph,
/// including the trivial case of a direct self-loop, where the edge's source and target are the very same node.
///
/// Returns the full path with the applied type constructor recorded at the hop where it was
/// applied, or `None` if the graph is safe to solve as-is.
pub fn find_growing_cycle(graph: &ConstraintGraph) -> Option<GrowingCycle> {
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
                    return Some(GrowingCycle { steps });
                }
            }
        }
    }
    None
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

#[cfg(test)]
mod tests {

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
        let result = find_growing_cycle(&graph);

        assert!(
            result.is_none(),
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
        let result = find_growing_cycle(&graph);

        assert!(result.is_some(), "Direct growing cycle was not detected.");

        let path = result.unwrap();
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
        let result = find_growing_cycle(&graph);

        assert!(result.is_some(), "Indirect growing cycle was not detected.");

        let path = result.unwrap();
        let node_a = vec![id!("A", 1)];
        let node_b = vec![id!("B", 2)];

        assert_eq!(path.nodes(), vec![node_a.clone(), node_b, node_a.clone()]);
    }
}
