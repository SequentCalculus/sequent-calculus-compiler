//! This module implements the utility functions used during code generation.

use super::Backend;
use super::config::{
    REGISTER_NUM, RESERVED, RESERVED_SPILLS, Register, SPILL_NUM, Spill, Temporary,
};

use axcut2backend::utils::Utils;

impl Utils<Temporary> for Backend {
    fn temporary_from_position(position: usize) -> Temporary {
        // temporaries are assigned left to right, starting with the first free register not
        // reserved for other purposes
        let register_number = position + RESERVED;
        // we spill if all registers are occupied
        if register_number < REGISTER_NUM {
            Temporary::Register(Register::X(register_number))
        } else {
            let spill_number = register_number - REGISTER_NUM + RESERVED_SPILLS;
            assert!(spill_number < SPILL_NUM, "Out of temporaries");
            Temporary::Spill(Spill(spill_number))
        }
    }
}
