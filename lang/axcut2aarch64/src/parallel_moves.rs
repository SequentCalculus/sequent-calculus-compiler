use super::Backend;
use super::code::Code;
use super::config::{Register, TEMP, TEMP2, Temporary, stack_offset};

use axcut2backend::parallel_moves::{ParallelMoves, Root, SpillMove};

impl ParallelMoves<Code, Temporary> for Backend {
    fn contains_spill_edge(_root: &Root<Temporary>) -> bool {
        // as we have two scratch registers, it is immaterial whether there is a spill edge
        false
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
            (Temporary::Register(source_register), Temporary::Spill(target_position)) => {
                instructions.push(Code::STR(
                    source_register,
                    Register::SP,
                    stack_offset(target_position),
                ))
            }
            (Temporary::Spill(source_position), Temporary::Register(target_register)) => {
                instructions.push(Code::LDR(
                    target_register,
                    Register::SP,
                    stack_offset(source_position),
                ))
            }
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
