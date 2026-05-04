//! This module implements the utility functions used during code generation.

use super::Backend;
use super::config::{
    REGISTER_NUM, RESERVED, RESERVED_SPILLS, Register, SPILL_NUM, Spill, Temporary,
};

use axcut::syntax::{ID, TypingContext};
use axcut2backend::{config::TemporaryNumber, utils::Utils};

fn temporary_from_position(position: usize) -> Temporary {
    // temporaries are assigned left to right, starting with the first free register not
    // reserved for other purposes
    let register_number = position + RESERVED;
    // we spill if all registers are occupied
    if register_number < REGISTER_NUM {
        Temporary::Register(Register(register_number))
    } else {
        let spill_number = register_number - REGISTER_NUM + RESERVED_SPILLS;
        assert!(spill_number < SPILL_NUM, "Out of temporaries");
        Temporary::Spill(Spill(spill_number))
    }
}

impl Utils<Temporary> for Backend {
    fn variable_temporary(
        number: TemporaryNumber,
        context: &TypingContext,
        variable_id: ID,
    ) -> Temporary {
        fn get_position(context: &TypingContext, variable_id: ID) -> usize {
            context
                .bindings
                .iter()
                .position(|binding| binding.var.id == variable_id)
                .unwrap_or_else(|| {
                    panic!("Variable {variable_id} not found in context {context:?}",)
                })
        }

        let position = 2 * get_position(context, variable_id) + number as usize;
        temporary_from_position(position)
    }

    fn fresh_temporary(number: TemporaryNumber, context: &TypingContext) -> Temporary {
        let position = 2 * context.bindings.len() + number as usize;
        temporary_from_position(position)
    }
}
