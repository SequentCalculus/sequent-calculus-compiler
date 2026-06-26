use std::collections::{HashMap, HashSet, VecDeque};

use crate::mono::constraint_graph::{ConstraintGraph, Node};

/// Searches the constraint graph for a growing cycle.
///
/// An edge is "growing" if it applies a type constructor at some position.
/// A growing cycle exists if, for some growing edge, its target node can
/// reach back to one of its own source nodes via any path in the graph.
///
/// Returns the path forming the cycle if one is found, or `None` if the
/// graph is safe to solve.
pub fn find_growing_cycle(graph: &ConstraintGraph) -> Option<Vec<Node>> {
    for edges in graph.edges.values() {
        for edge in edges {
            if !edge.has_constructor_position() {
                continue;
            }
            for source in edge.source_nodes(&graph.locations) {
                if let Some(path) = bfs_path(graph, &edge.into, &source) {
                    return Some(path);
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
        assert_eq!(path, vec![node_a]);
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

        assert_eq!(path, vec![node_b, node_a]);
    }
}
