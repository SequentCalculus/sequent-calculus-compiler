use parallel_moves::{spanning_forest, Root, Tree};

use super::code::Code;
use super::config::{Register, REGISTER_NUM, TEMP};

use std::collections::{BTreeMap, BTreeSet};

fn tree_moves(register: Register, tree: &Tree<Register>, instructions: &mut Vec<Code>) {
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

fn root_moves(root: Root<Register>, instructions: &mut Vec<Code>) {
    match root {
        Root::StartNode(register, trees) => {
            for tree in &trees {
                tree_moves(register, tree, instructions);
            }
            if trees.iter().any(Tree::refers_back) {
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
    for root in spanning_forest(REGISTER_NUM, assignments) {
        root_moves(root, instructions);
    }
}
