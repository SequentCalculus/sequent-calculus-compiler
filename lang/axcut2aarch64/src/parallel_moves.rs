//! This module implements the details for how the parallel moves between temporaries are performed.

use super::Backend;
use super::code::Code;
use super::config::{Register, TEMP, Temporary, stack_offset};

use axcut2backend::parallel_moves::{ParallelMoves, Root, SpillMove};

impl ParallelMoves<Code, Temporary> for Backend {
    /// This implementation always trivially returns `false` since we have two scratch registers,
    /// making it immaterial whether there is a spill edge.
    fn contains_spill_edge(_root: &Root<Temporary>) -> bool {
        false
    }

    fn store_temporary(temporary: Temporary, _: SpillMove, instructions: &mut Vec<Code>) {
        // we use the first scratch register for the backup, knowing that the moves between spill
        // positions use the first one
        match temporary {
            Temporary::Register(register) => {
                instructions.push(Code::MOVR(TEMP, register));
            }
            Temporary::Spill(position) => {
                instructions.push(Code::LDR(TEMP, Register::SP, stack_offset(position)));
            }
        }
    }

    fn restore_temporary(temporary: Temporary, _: SpillMove, instructions: &mut Vec<Code>) {
        match temporary {
            Temporary::Register(register) => {
                instructions.push(Code::MOVR(register, TEMP));
            }
            Temporary::Spill(position) => {
                instructions.push(Code::STR(TEMP, Register::SP, stack_offset(position)));
            }
        }
    }
}
