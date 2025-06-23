use super::Backend;
use super::config::{
    REGISTER_NUM, RESERVED, RESERVED_SPILLS, Register, SPILL_NUM, Spill, Temporary,
};

use axcut::syntax::{TypingContext, Var};
use axcut2backend::{config::TemporaryNumber, utils::Utils};

fn temporary_from_position(position: usize) -> Temporary {
    let register_number = position + RESERVED;
    if register_number < REGISTER_NUM {
        Temporary::Register(Register::X(register_number))
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
        variable: &Var,
    ) -> Temporary {
        fn get_position(context: &TypingContext, variable: &Var) -> usize {
            context
                .bindings
                .iter()
                .position(|binding| binding.var == *variable)
                .unwrap_or_else(|| panic!("Variable {variable} not found in context {context:?}"))
        }

        let position = 2 * get_position(context, variable) + number as usize;
        temporary_from_position(position)
    }

    fn fresh_temporary(number: TemporaryNumber, context: &TypingContext) -> Temporary {
        let position = 2 * context.bindings.len() + number as usize;
        temporary_from_position(position)
    }
}
