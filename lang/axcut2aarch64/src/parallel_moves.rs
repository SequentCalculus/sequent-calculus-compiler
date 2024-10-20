use super::code::Code;
use super::config::{Register, REGISTER_NUM, TEMP};

use std::collections::{BTreeMap, BTreeSet, HashSet};

enum Tree {
    BackEdge,
    Node(Register, Vec<Tree>),
}

enum Root {
    StartNode(Register, Vec<Tree>),
}

fn tree_nodes(tree: &Tree) -> HashSet<Register> {
    let mut visited = HashSet::new();
    match tree {
        Tree::BackEdge => {}
        Tree::Node(register, trees) => {
            visited.insert(*register);
            for tree in trees {
                visited.extend(tree_nodes(tree));
            }
        }
    }
    visited
}

fn refers_back(tree: &Tree) -> bool {
    match tree {
        Tree::BackEdge => true,
        Tree::Node(_, trees) => trees.iter().any(refers_back),
    }
}

fn visited_by(root: &Root) -> HashSet<Register> {
    let mut visited = HashSet::new();
    match root {
        Root::StartNode(register, trees) => {
            if trees.iter().any(refers_back) {
                visited.insert(*register);
            };
            for tree in trees {
                visited.extend(tree_nodes(tree));
            }
        }
    }
    visited
}

fn delete_targets(
    to_delete: &HashSet<Register>,
    parallel_moves: &mut BTreeMap<Register, BTreeSet<Register>>,
) {
    for targets in parallel_moves.values_mut() {
        targets.retain(|register| !(to_delete.contains(register)));
    }
}

fn spanning_tree(
    parallel_moves: &BTreeMap<Register, BTreeSet<Register>>,
    root: Register,
    node: Register,
) -> Tree {
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

fn spanning_forest(mut parallel_moves: BTreeMap<Register, BTreeSet<Register>>) -> Vec<Root> {
    let mut root_list = Vec::with_capacity(REGISTER_NUM);
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

fn tree_moves(register: Register, tree: &Tree, instructions: &mut Vec<Code>) {
    match tree {
        Tree::BackEdge => instructions.push(Code::MOVR(TEMP, register)),
        Tree::Node(target_register, trees) => {
            for tree in trees {
                tree_moves(*target_register, tree, instructions);
            }
            instructions.push(Code::MOVR(*target_register, register));
        }
    }
}

fn root_moves(root: Root, instructions: &mut Vec<Code>) {
    match root {
        Root::StartNode(register, trees) => {
            for tree in &trees {
                tree_moves(register, tree, instructions);
            }
            if trees.iter().any(refers_back) {
                instructions.push(Code::MOVR(register, TEMP));
            };
        }
    }
}

/// This assumes that the `BTreeSet`s in `assignments` are pairwise disjoint.
pub fn parallel_moves(
    assignments: BTreeMap<Register, BTreeSet<Register>>,
    instructions: &mut Vec<Code>,
) {
    for root in spanning_forest(assignments) {
        root_moves(root, instructions);
    }
}
