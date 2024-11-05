use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::hash::Hash;

pub enum Tree<T> {
    BackEdge,
    Node(T, Vec<Tree<T>>),
}

pub enum Root<T> {
    StartNode(T, Vec<Tree<T>>),
}

fn tree_nodes<T>(tree: &Tree<T>) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    let mut visited = HashSet::new();
    match tree {
        Tree::BackEdge => {}
        Tree::Node(temporary, trees) => {
            visited.insert(*temporary);
            for tree in trees {
                visited.extend(tree_nodes(tree));
            }
        }
    }
    visited
}

pub fn refers_back<T>(tree: &Tree<T>) -> bool {
    match tree {
        Tree::BackEdge => true,
        Tree::Node(_, trees) => trees.iter().any(refers_back),
    }
}

fn visited_by<T: Eq + Hash + Copy>(root: &Root<T>) -> HashSet<T> {
    let mut visited = HashSet::new();
    match root {
        Root::StartNode(temporary, trees) => {
            if trees.iter().any(refers_back) {
                visited.insert(*temporary);
            };
            for tree in trees {
                visited.extend(tree_nodes(tree));
            }
        }
    }
    visited
}

fn delete_targets<T: Ord + Hash>(
    to_delete: &HashSet<T>,
    parallel_moves: &mut BTreeMap<T, BTreeSet<T>>,
) {
    for targets in parallel_moves.values_mut() {
        targets.retain(|register| !(to_delete.contains(register)));
    }
}

fn spanning_tree<T: Ord + Clone + Copy>(
    parallel_moves: &BTreeMap<T, BTreeSet<T>>,
    root: T,
    node: T,
) -> Tree<T> {
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

pub fn spanning_forest<T: Copy + Ord + Hash>(
    register_num: usize,
    mut parallel_moves: BTreeMap<T, BTreeSet<T>>,
) -> Vec<Root<T>> {
    let mut root_list = Vec::with_capacity(register_num);
    let mappings = parallel_moves.clone();
    for register in mappings.keys() {
        let mut targets = parallel_moves[register].clone();
        let _ = targets.remove(register);
        let root = Root::StartNode(
            *register,
            targets
                .into_iter()
                .map(|target| spanning_tree(&parallel_moves, *register, target))
                .collect(),
        );
        delete_targets(&visited_by(&root), &mut parallel_moves);
        root_list.push(root);
    }
    root_list
}
