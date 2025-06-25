//! This module implements the details for how the parallel moves between temporaries are performed.

use super::Backend;
use super::code::Code;
use super::config::{Register, TEMP};

use axcut2backend::parallel_moves::{ParallelMoves, Root, SpillMove};

impl ParallelMoves<Code, Register> for Backend {
    fn contains_spill_edge(_root: &Root<Register>) -> bool {
        // as there are no spills, there can be no spill edge
        false
    }

    fn store_temporary(temporary: Register, _: SpillMove, instructions: &mut Vec<Code>) {
        instructions.push(Code::MV(TEMP, temporary));
    }

    fn restore_temporary(temporary: Register, _: SpillMove, instructions: &mut Vec<Code>) {
        instructions.push(Code::MV(temporary, TEMP));
    }
}
