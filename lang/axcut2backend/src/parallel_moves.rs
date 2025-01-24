use crate::code::Instructions;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::hash::Hash;

pub enum Tree<Temporary> {
    BackEdge,
    Node(Temporary, Vec<Tree<Temporary>>),
}

impl<Temporary: Eq + Hash + Copy> Tree<Temporary> {
    fn nodes(&self) -> HashSet<Temporary> {
        let mut visited = HashSet::new();
        match self {
            Tree::BackEdge => {}
            Tree::Node(temporary, trees) => {
                visited.insert(*temporary);
                for tree in trees {
                    visited.extend(tree.nodes());
                }
            }
        }
        visited
    }

    pub fn refers_back(&self) -> bool {
        match self {
            Tree::BackEdge => true,
            Tree::Node(_, trees) => trees.iter().any(Tree::refers_back),
        }
    }
}

pub enum Root<Temporary> {
    StartNode(Temporary, Vec<Tree<Temporary>>),
}

impl<Temporary: Eq + Hash + Copy> Root<Temporary> {
    fn visited_by(&self) -> HashSet<Temporary> {
        let mut visited = HashSet::new();
        match self {
            Root::StartNode(temporary, trees) => {
                if trees.iter().any(Tree::refers_back) {
                    visited.insert(*temporary);
                };
                for tree in trees {
                    visited.extend(tree.nodes());
                }
            }
        }
        visited
    }
}

fn delete_targets<Temporary: Ord + Hash>(
    to_delete: &HashSet<Temporary>,
    parallel_moves: &mut BTreeMap<Temporary, BTreeSet<Temporary>>,
) {
    for targets in parallel_moves.values_mut() {
        targets.retain(|temporary| !(to_delete.contains(temporary)));
    }
}

fn spanning_tree<Temporary: Ord + Copy + Clone>(
    parallel_moves: &BTreeMap<Temporary, BTreeSet<Temporary>>,
    root: Temporary,
    node: Temporary,
) -> Tree<Temporary> {
    if root == node {
        Tree::BackEdge
    } else if parallel_moves.contains_key(&node) {
        let targets = parallel_moves[&node].clone();
        Tree::Node(
            node,
            targets
                .into_iter()
                .map(|target| spanning_tree(parallel_moves, root, target))
                .collect(),
        )
    } else {
        Tree::Node(node, Vec::new())
    }
}

fn spanning_forest<Temporary: Ord + Hash + Copy>(
    mut parallel_moves: BTreeMap<Temporary, BTreeSet<Temporary>>,
) -> Vec<Root<Temporary>> {
    let mut root_list = Vec::with_capacity(parallel_moves.len());
    let mappings = parallel_moves.clone();
    for temporary in mappings.keys() {
        let mut targets = parallel_moves[temporary].clone();
        let _ = targets.remove(temporary);
        let root = Root::StartNode(
            *temporary,
            targets
                .into_iter()
                .map(|target| spanning_tree(&parallel_moves, *temporary, target))
                .collect(),
        );
        delete_targets(&root.visited_by(), &mut parallel_moves);
        root_list.push(root);
    }
    root_list
}

pub trait ParallelMoves<Code, Temporary> {
    fn root_moves(root: Root<Temporary>, instructions: &mut Vec<Code>);
}

/// This assumes that the `BTreeSet`s in `assignments` are pairwise disjoint.
pub fn parallel_moves<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    assignments: BTreeMap<Temporary, BTreeSet<Temporary>>,
    instructions: &mut Vec<Code>,
) where
    Backend: ParallelMoves<Code, Temporary> + Instructions<Code, Temporary, Immediate>,
{
    let spanning_forest = spanning_forest(assignments);

    if !spanning_forest
        .iter()
        .all(|Root::StartNode(_, targets)| targets.is_empty())
    {
        instructions.push(Backend::comment("#move variables".to_string()));
    }

    for root in spanning_forest {
        Backend::root_moves(root, instructions);
    }
}
