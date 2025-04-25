use super::Backend;
use super::code::Code;
use super::config::{Register, TEMP};

use axcut2backend::parallel_moves::{ParallelMoves, Root, Tree};

fn tree_moves(register: Register, tree: &Tree<Register>, instructions: &mut Vec<Code>) {
    match tree {
        Tree::BackEdge => instructions.push(Code::MV(TEMP, register)),
        Tree::Node(target_register, trees) => {
            for tree in trees {
                tree_moves(*target_register, tree, instructions);
            }
            instructions.push(Code::MV(*target_register, register));
        }
    }
}

impl ParallelMoves<Code, Register> for Backend {
    fn root_moves(
        root: axcut2backend::parallel_moves::Root<Register>,
        instructions: &mut Vec<Code>,
    ) {
        match root {
            Root::StartNode(register, trees) => {
                for tree in &trees {
                    tree_moves(register, tree, instructions);
                }
                if trees.iter().any(Tree::refers_back) {
                    instructions.push(Code::MV(register, TEMP));
                };
            }
        }
    }
}
