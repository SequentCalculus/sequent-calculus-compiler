//! This module implements the details for how the parallel moves between temporaries are performed.

use super::Backend;
use super::code::Code;
use super::config::{SPILL_TEMP, STACK, TEMP, Temporary, stack_offset};

use axcut2backend::parallel_moves::{ParallelMoves, Root, SpillMove, Tree};

/// This type encodes whether the root node in a rooted spanning tree represents a spill spot.
type IsSpill = bool;

/// This function returns whether one of the edges in a part of a rooted spanning tree represents a
/// move between two spill positions in memory. The parent of the given part of the tree is
/// supposed to be a spill spot.
/// - `root_spill` encodes whether the root of the rooted spanning tree represents a spill spot.
/// - `tree` is the part of the rooted spanning tree.
fn spill_edge_spill(root_spill: IsSpill, tree: &Tree<Temporary>) -> SpillMove {
    match tree {
        // if we have arrived at the back edge to the root from a spill node, then we have a spill
        // move if the root is a spill too
        Tree::BackEdge => root_spill,
        Tree::Node(Temporary::Register(_), trees) => trees
            .iter()
            .any(|tree| spill_edge_register(root_spill, tree)),
        // if we have arrived at at a spill node from a spill node, then we have a spill move
        Tree::Node(Temporary::Spill(_), _) => true,
    }
}

/// This function returns whether one of the edges in a part of a rooted spanning tree represents a
/// move between two spill positions in memory. The parent of the given part of the tree is
/// supposed to be a register.
/// - `root_spill` encodes whether the root of the rooted spanning tree represents a spill spot.
/// - `tree` is the part of the rooted spanning tree.
fn spill_edge_register(root_spill: IsSpill, tree: &Tree<Temporary>) -> SpillMove {
    match tree {
        // if we have arrived at the back edge to the root from a register node, then we have no
        // spill move
        Tree::BackEdge => false,
        Tree::Node(Temporary::Register(_), trees) => trees
            .iter()
            .any(|tree| spill_edge_register(root_spill, tree)),
        Tree::Node(Temporary::Spill(_), trees) => {
            trees.iter().any(|tree| spill_edge_spill(root_spill, tree))
        }
    }
}

impl ParallelMoves<Code, Temporary> for Backend {
    fn contains_spill_edge(root: &Root<Temporary>) -> SpillMove {
        match root {
            Root::StartNode(Temporary::Register(_), trees) => {
                trees.iter().any(|tree| spill_edge_register(false, tree))
            }
            Root::StartNode(Temporary::Spill(_), trees) => {
                trees.iter().any(|tree| spill_edge_spill(true, tree))
            }
        }
    }

    fn store_temporary(
        temporary: Temporary,
        contains_spill_move: SpillMove,
        instructions: &mut Vec<Code>,
    ) {
        match temporary {
            Temporary::Register(register) => {
                // if there is a move between spill positions, we evacuate the temporary to the
                // stack
                if contains_spill_move {
                    instructions.push(Code::MOVS(register, STACK, stack_offset(SPILL_TEMP)));
                } else {
                    instructions.push(Code::MOV(TEMP, register));
                }
            }
            Temporary::Spill(position) => {
                instructions.push(Code::MOVL(TEMP, STACK, stack_offset(position)));
                // if there is a move between spill positions, we evacuate the temporary to the
                // stack
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
                // if there was a move between spill positions, we had evacuated the temporary to
                // the stack
                if contains_spill_move {
                    instructions.push(Code::MOVL(register, STACK, stack_offset(SPILL_TEMP)));
                } else {
                    instructions.push(Code::MOV(register, TEMP));
                }
            }
            Temporary::Spill(position) => {
                // if there was a move between spill positions, we had evacuated the temporary to
                // the stack
                if contains_spill_move {
                    instructions.push(Code::MOVL(TEMP, STACK, stack_offset(SPILL_TEMP)));
                }
                instructions.push(Code::MOVS(TEMP, STACK, stack_offset(position)));
            }
        }
    }
}
