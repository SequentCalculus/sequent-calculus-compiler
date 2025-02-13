use crate::code::Code;

use super::config::{field_offset, Register, FIELDS_PER_BLOCK, FREE, HEAP};

use axcut2backend::{coder::AssemblyProg, config::TemporaryNumber::Fst};

fn preamble() -> Vec<Code> {
    use Code::*;
    vec![
        TEXT,
        GLOBAL("asm_main".to_string()),
        LAB("asm_main".to_string()),
    ]
}

fn move_arguments(number_of_arguments: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::COMMENT("move parameters into place".to_string()));
    match number_of_arguments {
        0 => {}
        1 => {
            instructions.push(Code::MOVR(Register::X(4), Register::X(1)));
        }
        2 => {
            instructions.push(Code::MOVR(Register::X(6), Register::X(2)));
            move_arguments(1, instructions);
        }
        3 => {
            instructions.push(Code::MOVR(Register::X(8), Register::X(3)));
            move_arguments(2, instructions);
        }
        4 => {
            instructions.push(Code::MOVR(Register::X(10), Register::X(4)));
            move_arguments(3, instructions);
        }
        5 => {
            instructions.push(Code::MOVR(Register::X(12), Register::X(5)));
            move_arguments(4, instructions);
        }
        6 => {
            instructions.push(Code::MOVR(Register::X(14), Register::X(6)));
            move_arguments(5, instructions);
        }
        7 => {
            instructions.push(Code::MOVR(Register::X(16), Register::X(7)));
            move_arguments(6, instructions);
        }
        _ => panic!("too many arguments for main"),
    }
}

fn setup(number_of_arguments: usize, instructions: &mut Vec<Code>) {
    use Code::*;
    instructions.push(COMMENT("setup".to_string()));
    instructions.push(COMMENT("save registers".to_string()));
    instructions.push(STP_PRE_INDEX(
        Register::X(18),
        Register::X(19),
        Register::SP,
        (-16).into(),
    ));
    instructions.push(STP_PRE_INDEX(
        Register::X(20),
        Register::X(21),
        Register::SP,
        (-16).into(),
    ));
    instructions.push(STP_PRE_INDEX(
        Register::X(22),
        Register::X(23),
        Register::SP,
        (-16).into(),
    ));
    instructions.push(STP_PRE_INDEX(
        Register::X(24),
        Register::X(25),
        Register::SP,
        (-16).into(),
    ));
    instructions.push(STP_PRE_INDEX(
        Register::X(26),
        Register::X(27),
        Register::SP,
        (-16).into(),
    ));
    instructions.push(STP_PRE_INDEX(
        Register::X(28),
        Register::X(29),
        Register::SP,
        (-16).into(),
    ));
    move_arguments(number_of_arguments, instructions);
    instructions.push(COMMENT("initialize free pointer".to_string()));
    instructions.push(MOVR(FREE, HEAP));
    instructions.push(ADDI(FREE, FREE, field_offset(Fst, FIELDS_PER_BLOCK)));
}

fn cleanup() -> Vec<Code> {
    use Code::*;
    vec![
        LAB("cleanup".to_string()),
        COMMENT("restore registers".to_string()),
        LDP_POST_INDEX(Register::X(28), Register::X(29), Register::SP, 16.into()),
        LDP_POST_INDEX(Register::X(26), Register::X(27), Register::SP, 16.into()),
        LDP_POST_INDEX(Register::X(24), Register::X(25), Register::SP, 16.into()),
        LDP_POST_INDEX(Register::X(22), Register::X(23), Register::SP, 16.into()),
        LDP_POST_INDEX(Register::X(20), Register::X(21), Register::SP, 16.into()),
        LDP_POST_INDEX(Register::X(18), Register::X(19), Register::SP, 16.into()),
        RET,
    ]
}

#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_aarch64_routine(prog: AssemblyProg<Code>) -> AssemblyProg<Code> {
    let AssemblyProg {
        mut instructions,
        number_of_arguments,
    } = prog;

    let mut all_instructions: Vec<Code> = Vec::new();
    all_instructions.append(&mut preamble());
    setup(number_of_arguments, &mut all_instructions);
    all_instructions.push(Code::COMMENT("actual code".to_string()));
    all_instructions.append(&mut instructions);
    all_instructions.append(&mut cleanup());
    AssemblyProg {
        instructions: all_instructions,
        number_of_arguments,
    }
}
