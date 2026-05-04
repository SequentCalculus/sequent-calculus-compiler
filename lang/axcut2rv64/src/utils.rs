//! This module implements the utility functions used during code generation.

use super::Backend;
use super::config::{REGISTER_NUM, RESERVED, Register};

use axcut::syntax::{ID, TypingContext};
use axcut2backend::{config::TemporaryNumber, utils::Utils};

impl Utils<Register> for Backend {
    fn variable_temporary(
        number: TemporaryNumber,
        context: &TypingContext,
        variable_id: ID,
    ) -> Register {
        fn get_position(context: &TypingContext, variable_id: ID) -> usize {
            context
                .bindings
                .iter()
                .position(|binding| binding.var.id == variable_id)
                .unwrap_or_else(|| {
                    panic!("Variable {variable_id} not found in context {context:?}",)
                })
        }

        let variable_position = get_position(context, variable_id);
        // temporaries are assigned left to right, starting with the first free register not
        // reserved for other purposes
        let register_number = 2 * variable_position + number as usize + RESERVED;
        assert!(register_number < REGISTER_NUM, "Out of registers");
        Register(register_number)
    }

    fn fresh_temporary(number: TemporaryNumber, context: &TypingContext) -> Register {
        let variable_position = context.bindings.len();
        // temporaries are assigned left to right, starting with the first free register not
        // reserved for other purposes
        let register_number = 2 * variable_position + number as usize + RESERVED;
        assert!(register_number < REGISTER_NUM, "Out of registers");
        Register(register_number)
    }
}
