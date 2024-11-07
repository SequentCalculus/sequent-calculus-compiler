use super::code::Code;
use super::config::{stack_offset, Temporary, REGISTER_NUM, SPILL_TEMP, STACK, TEMP};

use std::collections::{BTreeMap, BTreeSet, HashSet};

enum Tree {
    BackEdge,
    Node(Temporary, Vec<Tree>),
}

enum Root {
    StartNode(Temporary, Vec<Tree>),
}

fn tree_nodes(tree: &Tree) -> HashSet<Temporary> {
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

fn refers_back(tree: &Tree) -> bool {
    match tree {
        Tree::BackEdge => true,
        Tree::Node(_, trees) => trees.iter().any(refers_back),
    }
}

type IsSpill = bool;

fn spill_edge_spill(root_spill: IsSpill, tree: &Tree) -> bool {
    match tree {
        Tree::BackEdge => root_spill,
        Tree::Node(Temporary::Register(_), trees) => trees
            .iter()
            .any(|tree| spill_edge_register(root_spill, tree)),
        Tree::Node(Temporary::Spill(_), _) => true,
    }
}

fn spill_edge_register(root_spill: IsSpill, tree: &Tree) -> bool {
    match tree {
        Tree::BackEdge => false,
        Tree::Node(Temporary::Register(_), trees) => trees
            .iter()
            .any(|tree| spill_edge_register(root_spill, tree)),
        Tree::Node(Temporary::Spill(_), trees) => {
            trees.iter().any(|tree| spill_edge_spill(root_spill, tree))
        }
    }
}

fn contains_spill_edge(root: &Root) -> bool {
    match root {
        Root::StartNode(Temporary::Register(_), trees) => {
            trees.iter().any(|tree| spill_edge_register(false, tree))
        }
        Root::StartNode(Temporary::Spill(_), trees) => {
            trees.iter().any(|tree| spill_edge_spill(true, tree))
        }
    }
}

fn visited_by(root: &Root) -> HashSet<Temporary> {
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

fn delete_targets(
    to_delete: &HashSet<Temporary>,
    parallel_moves: &mut BTreeMap<Temporary, BTreeSet<Temporary>>,
) {
    for targets in parallel_moves.values_mut() {
        targets.retain(|temporary| !(to_delete.contains(temporary)));
    }
}

fn spanning_tree(
    parallel_moves: &BTreeMap<Temporary, BTreeSet<Temporary>>,
    root: Temporary,
    node: Temporary,
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

fn spanning_forest(mut parallel_moves: BTreeMap<Temporary, BTreeSet<Temporary>>) -> Vec<Root> {
    let mut root_list = Vec::with_capacity(REGISTER_NUM);
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
        delete_targets(&visited_by(&root), &mut parallel_moves);
        root_list.push(root);
    }
    root_list
}

type SpillMove = bool;

fn store_temporary(
    temporary: Temporary,
    contains_spill_move: SpillMove,
    instructions: &mut Vec<Code>,
) {
    match temporary {
        Temporary::Register(register) => {
            if contains_spill_move {
                instructions.push(Code::MOVS(register, STACK, stack_offset(SPILL_TEMP)));
            } else {
                instructions.push(Code::MOV(TEMP, register));
            }
        }
        Temporary::Spill(position) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
            if contains_spill_move {
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(SPILL_TEMP)));
            }
        }
    }
}

fn restore_temporary(
    temporary: Temporary,
    contains_spill_move: SpillMove,
    instructions: &mut Vec<Code>,
) {
    match temporary {
        Temporary::Register(register) => {
            if contains_spill_move {
                instructions.push(Code::MOVL(register, STACK, stack_offset(SPILL_TEMP)));
            } else {
                instructions.push(Code::MOV(register, TEMP));
            }
        }
        Temporary::Spill(position) => {
            if contains_spill_move {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(SPILL_TEMP)));
            }
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
        }
    }
}

fn move_to_temporary(
    target_temporary: Temporary,
    source_temporary: Temporary,
    instructions: &mut Vec<Code>,
) {
    match (source_temporary, target_temporary) {
        (Temporary::Register(source_register), Temporary::Register(target_register)) => {
            instructions.push(Code::MOV(target_register, source_register));
        }
        (Temporary::Register(source_register), Temporary::Spill(target_position)) => instructions
            .push(Code::MOVS(
                source_register,
                STACK,
                stack_offset(target_position),
            )),
        (Temporary::Spill(source_position), Temporary::Register(target_register)) => instructions
            .push(Code::MOVL(
                target_register,
                STACK,
                stack_offset(source_position),
            )),
        (Temporary::Spill(source_position), Temporary::Spill(target_position)) => {
            instructions.push(Code::MOVL(TEMP, STACK, stack_offset(source_position)));
            instructions.push(Code::MOVS(TEMP, STACK, stack_offset(target_position)));
        }
    }
}

fn tree_moves(
    temporary: Temporary,
    tree: &Tree,
    contains_spill_move: SpillMove,
    instructions: &mut Vec<Code>,
) {
    match tree {
        Tree::BackEdge => store_temporary(temporary, contains_spill_move, instructions),
        Tree::Node(target_temporary, trees) => {
            for tree in trees {
                tree_moves(*target_temporary, tree, contains_spill_move, instructions);
            }
            move_to_temporary(*target_temporary, temporary, instructions);
        }
    }
}

fn root_moves(root: Root, instructions: &mut Vec<Code>) {
    let contains_spill_move = contains_spill_edge(&root);
    match root {
        Root::StartNode(temporary, trees) => {
            for tree in &trees {
                tree_moves(temporary, tree, contains_spill_move, instructions);
            }
            if trees.iter().any(refers_back) {
                restore_temporary(temporary, contains_spill_move, instructions);
            };
        }
    }
}

/// This assumes that the `BTreeSet`s in `assignments` are pairwise disjoint.
pub fn parallel_moves(
    assignments: BTreeMap<Temporary, BTreeSet<Temporary>>,
    instructions: &mut Vec<Code>,
) {
    for root in spanning_forest(assignments) {
        root_moves(root, instructions);
    }
}
