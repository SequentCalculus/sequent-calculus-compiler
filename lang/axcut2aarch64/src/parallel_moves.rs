use super::Backend;
use super::code::Code;
use super::config::{Register, TEMP, Temporary, stack_offset};

use axcut2backend::parallel_moves::{ParallelMoves, Root, SpillMove};

impl ParallelMoves<Code, Temporary> for Backend {
    fn contains_spill_edge(_root: &Root<Temporary>) -> bool {
        // as we have two scratch registers, it is immaterial whether there is a spill edge
        false
    }

    fn store_temporary(temporary: Temporary, _: SpillMove, instructions: &mut Vec<Code>) {
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
