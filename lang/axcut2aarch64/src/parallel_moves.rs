use super::Backend;
use super::code::Code;
use super::config::{Register, TEMP, TEMP2, Temporary, stack_offset};

use axcut2backend::parallel_moves::{ParallelMoves, Root, Tree};

fn store_temporary(temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(register) => {
            instructions.push(Code::MOVR(TEMP, register));
        }
        Temporary::Spill(position) => {
            instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position)));
        }
    }
}

fn restore_temporary(temporary: Temporary, instructions: &mut Vec<Code>) {
    match temporary {
        Temporary::Register(register) => {
            instructions.push(Code::MOVR(register, TEMP));
        }
        Temporary::Spill(position) => {
            instructions.push(Code::STR(TEMP, Register::SP, stack_offset(position)));
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
            instructions.push(Code::MOVR(target_register, source_register));
        }
        (Temporary::Register(source_register), Temporary::Spill(target_position)) => instructions
            .push(Code::STR(
                source_register,
                Register::SP,
                stack_offset(target_position),
            )),
        (Temporary::Spill(source_position), Temporary::Register(target_register)) => instructions
            .push(Code::LDR(
                target_register,
                Register::SP,
                stack_offset(source_position),
            )),
        (Temporary::Spill(source_position), Temporary::Spill(target_position)) => {
            instructions.push(Code::LDR(
                TEMP2,
                Register::SP,
                stack_offset(source_position),
            ));
            instructions.push(Code::STR(
                TEMP2,
                Register::SP,
                stack_offset(target_position),
            ));
        }
    }
}

fn tree_moves(temporary: Temporary, tree: &Tree<Temporary>, instructions: &mut Vec<Code>) {
    match tree {
        Tree::BackEdge => store_temporary(temporary, instructions),
        Tree::Node(target_temporary, trees) => {
            for tree in trees {
                tree_moves(*target_temporary, tree, instructions);
            }
            move_to_temporary(*target_temporary, temporary, instructions);
        }
    }
}

impl ParallelMoves<Code, Temporary> for Backend {
    fn root_moves(
        root: axcut2backend::parallel_moves::Root<Temporary>,
        instructions: &mut Vec<Code>,
    ) {
        match root {
            Root::StartNode(temporary, trees) => {
                for tree in &trees {
                    tree_moves(temporary, tree, instructions);
                }
                if trees.iter().any(Tree::refers_back) {
                    restore_temporary(temporary, instructions);
                };
            }
        }
    }
}
