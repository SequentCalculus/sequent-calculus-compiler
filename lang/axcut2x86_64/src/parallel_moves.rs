use super::code::Code;
use super::config::{stack_offset, Temporary, SPILL_TEMP, STACK, TEMP};
use super::Backend;

use axcut2backend::parallel_moves::{ParallelMoves, Root, Tree};

type IsSpill = bool;

fn spill_edge_spill(root_spill: IsSpill, tree: &Tree<Temporary>) -> bool {
    match tree {
        Tree::BackEdge => root_spill,
        Tree::Node(Temporary::Register(_), trees) => trees
            .iter()
            .any(|tree| spill_edge_register(root_spill, tree)),
        Tree::Node(Temporary::Spill(_), _) => true,
    }
}

fn spill_edge_register(root_spill: IsSpill, tree: &Tree<Temporary>) -> bool {
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

fn contains_spill_edge(root: &Root<Temporary>) -> bool {
    match root {
        Root::StartNode(Temporary::Register(_), trees) => {
            trees.iter().any(|tree| spill_edge_register(false, tree))
        }
        Root::StartNode(Temporary::Spill(_), trees) => {
            trees.iter().any(|tree| spill_edge_spill(true, tree))
        }
    }
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
    tree: &Tree<Temporary>,
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

impl ParallelMoves<Code, Temporary> for Backend {
    fn root_moves(
        &self,
        root: axcut2backend::parallel_moves::Root<Temporary>,
        instructions: &mut Vec<Code>,
    ) {
        let contains_spill_move = contains_spill_edge(&root);
        match root {
            Root::StartNode(temporary, trees) => {
                for tree in &trees {
                    tree_moves(temporary, tree, contains_spill_move, instructions);
                }
                if trees.iter().any(Tree::refers_back) {
                    restore_temporary(temporary, contains_spill_move, instructions);
                };
            }
        }
    }
}
