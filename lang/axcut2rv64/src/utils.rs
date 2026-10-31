//! This module implements the utility functions used during code generation.

use super::Backend;
use super::config::{REGISTER_NUM, RESERVED, Register};

use axcut2backend::utils::Utils;

impl Utils<Register> for Backend {
    fn temporary_from_position(position: usize) -> Register {
        // temporaries are assigned left to right, starting with the first free register not
        // reserved for other purposes
        let register_number = position + RESERVED;
        assert!(register_number < REGISTER_NUM, "Out of registers");
        Register(register_number)
    }
}
