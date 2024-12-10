use super::config::{Register, REGISTER_NUM, RESERVED};
use super::Backend;

use axcut::syntax::{TypingContext, Var};
use axcut2backend::{config::TemporaryNumber, utils::Utils};

impl Utils<Register> for Backend {
    fn variable_temporary(
        number: TemporaryNumber,
        context: &TypingContext,
        variable: &Var,
    ) -> Register {
        fn get_position(context: &TypingContext, variable: &Var) -> usize {
            context
                .bindings
                .iter()
                .position(|binding| binding.var == *variable)
                .unwrap_or_else(|| panic!("Variable {variable} not found in context {context:?}"))
        }

        let variable_position = get_position(context, variable);
        let register_number = 2 * variable_position + number as usize + RESERVED;
        assert!(register_number < REGISTER_NUM, "Out of registers");
        Register::X(register_number)
    }

    fn fresh_temporary(number: TemporaryNumber, context: &TypingContext) -> Register {
        let variable_position = context.bindings.len();
        let register_number = 2 * variable_position + number as usize + RESERVED;
        assert!(register_number < REGISTER_NUM, "Out of registers");
        Register::X(register_number)
    }
}
