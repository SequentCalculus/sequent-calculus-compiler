use super::config::{
    Register, Spill, Temporary, REGISTER_NUM, RESERVED, RESERVED_SPILLS, SPILL_NUM,
};
use super::Backend;

use axcut::syntax::{TypingContext, Var};
use axcut2backend::{config::TemporaryNumber, utils::Utils};

impl Utils<Temporary> for Backend {
    fn variable_temporary(
        &self,
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
        let register_number = position + RESERVED;
        if register_number < REGISTER_NUM {
            Temporary::Register(Register(register_number))
        } else {
            let spill_number = register_number - REGISTER_NUM + RESERVED_SPILLS;
            assert!(spill_number < SPILL_NUM, "Out of temporaries");
            Temporary::Spill(Spill(spill_number))
        }
    }

    fn fresh_temporary(&self, number: TemporaryNumber, context: &TypingContext) -> Temporary {
        let position = 2 * context.bindings.len() + number as usize;
        let register_number = position + RESERVED;
        if register_number < REGISTER_NUM {
            Temporary::Register(Register(register_number))
        } else {
            let spill_number = register_number - REGISTER_NUM + RESERVED_SPILLS;
            assert!(spill_number < SPILL_NUM, "Out of temporaries");
            Temporary::Spill(Spill(spill_number))
        }
    }
}
