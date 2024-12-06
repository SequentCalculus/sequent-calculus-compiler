use crate::code::Code;

use super::config::{field_offset, Register, FIELDS_PER_BLOCK, FREE, HEAP};

use axcut2backend::{coder::AssemblyProg, config::TemporaryNumber::Fst};

fn preamble() -> Vec<Code> {
    use Code::*;
    vec![
        TEXT,
        GLOBAL("asm_main0".to_string()),
        GLOBAL("_asm_main0".to_string()),
        GLOBAL("asm_main1".to_string()),
        GLOBAL("_asm_main1".to_string()),
        GLOBAL("asm_main2".to_string()),
        GLOBAL("_asm_main2".to_string()),
        GLOBAL("asm_main3".to_string()),
        GLOBAL("_asm_main3".to_string()),
        GLOBAL("asm_main4".to_string()),
        GLOBAL("_asm_main4".to_string()),
        GLOBAL("asm_main5".to_string()),
        GLOBAL("_asm_main5".to_string()),
        GLOBAL("asm_main6".to_string()),
        GLOBAL("_asm_main6".to_string()),
        GLOBAL("asm_main7".to_string()),
        GLOBAL("_asm_main7".to_string()),
        LAB("asm_main0".to_string()),
        LAB("_asm_main0".to_string()),
        LAB("asm_main1".to_string()),
        LAB("_asm_main1".to_string()),
        LAB("asm_main2".to_string()),
        LAB("_asm_main2".to_string()),
        LAB("asm_main3".to_string()),
        LAB("_asm_main3".to_string()),
        LAB("asm_main4".to_string()),
        LAB("_asm_main4".to_string()),
        LAB("asm_main5".to_string()),
        LAB("_asm_main5".to_string()),
        LAB("asm_main6".to_string()),
        LAB("_asm_main6".to_string()),
        LAB("asm_main7".to_string()),
        LAB("_asm_main7".to_string()),
        COMMENT("setup".to_string()),
        COMMENT("save registers".to_string()),
        STR_PRE_INDEX(Register::X(16), Register::SP, -16),
        STR_PRE_INDEX(Register::X(17), Register::SP, -16),
        STR_PRE_INDEX(Register::X(18), Register::SP, -16),
        STR_PRE_INDEX(Register::X(19), Register::SP, -16),
        STR_PRE_INDEX(Register::X(20), Register::SP, -16),
        STR_PRE_INDEX(Register::X(21), Register::SP, -16),
        STR_PRE_INDEX(Register::X(22), Register::SP, -16),
        STR_PRE_INDEX(Register::X(23), Register::SP, -16),
        STR_PRE_INDEX(Register::X(24), Register::SP, -16),
        STR_PRE_INDEX(Register::X(25), Register::SP, -16),
        STR_PRE_INDEX(Register::X(26), Register::SP, -16),
        STR_PRE_INDEX(Register::X(27), Register::SP, -16),
        STR_PRE_INDEX(Register::X(28), Register::SP, -16),
        STR_PRE_INDEX(Register::X(29), Register::SP, -16),
        STR_PRE_INDEX(Register::X(30), Register::SP, -16),
    ]
}

fn move_params(number_of_params: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::COMMENT("move parameters into place".to_string()));
    match number_of_params {
        0 => {}
        1 => {
            instructions.push(Code::MOVR(Register::X(4), Register::X(1)));
        }
        2 => {
            instructions.push(Code::MOVR(Register::X(6), Register::X(2)));
            move_params(1, instructions);
        }
        3 => {
            instructions.push(Code::MOVR(Register::X(8), Register::X(3)));
            move_params(2, instructions);
        }
        4 => {
            instructions.push(Code::MOVR(Register::X(10), Register::X(4)));
            move_params(3, instructions);
        }
        5 => {
            instructions.push(Code::MOVR(Register::X(12), Register::X(5)));
            move_params(4, instructions);
        }
        6 => {
            instructions.push(Code::MOVR(Register::X(14), Register::X(6)));
            move_params(5, instructions);
        }
        7 => {
            instructions.push(Code::MOVR(Register::X(16), Register::X(7)));
            move_params(6, instructions);
        }
        _ => panic!("too many arguments for main"),
    }
}

fn setup() -> Vec<Code> {
    use Code::*;
    vec![
        COMMENT("initialize free pointer".to_string()),
        MOVR(FREE, HEAP),
        ADDI(FREE, FREE, field_offset(Fst, FIELDS_PER_BLOCK)),
    ]
}

fn cleanup() -> Vec<Code> {
    use Code::*;
    vec![
        COMMENT("cleanup".to_string()),
        LAB("cleanup".to_string()),
        COMMENT("restore registers".to_string()),
        LDR_POST_INDEX(Register::X(30), Register::SP, 16),
        LDR_POST_INDEX(Register::X(29), Register::SP, 16),
        LDR_POST_INDEX(Register::X(28), Register::SP, 16),
        LDR_POST_INDEX(Register::X(27), Register::SP, 16),
        LDR_POST_INDEX(Register::X(26), Register::SP, 16),
        LDR_POST_INDEX(Register::X(25), Register::SP, 16),
        LDR_POST_INDEX(Register::X(24), Register::SP, 16),
        LDR_POST_INDEX(Register::X(23), Register::SP, 16),
        LDR_POST_INDEX(Register::X(22), Register::SP, 16),
        LDR_POST_INDEX(Register::X(21), Register::SP, 16),
        LDR_POST_INDEX(Register::X(20), Register::SP, 16),
        LDR_POST_INDEX(Register::X(19), Register::SP, 16),
        LDR_POST_INDEX(Register::X(18), Register::SP, 16),
        LDR_POST_INDEX(Register::X(17), Register::SP, 16),
        LDR_POST_INDEX(Register::X(16), Register::SP, 16),
        RET,
    ]
}

#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_aarch64_routine(prog: AssemblyProg<Code>) -> String {
    let AssemblyProg {
        mut instructions,
        number_of_arguments,
    } = prog;

    let mut all_instructions: Vec<Code> = Vec::new();
    all_instructions.append(&mut preamble());
    move_params(number_of_arguments, &mut all_instructions);
    all_instructions.append(&mut setup());
    all_instructions.push(Code::COMMENT("actual code".to_string()));
    all_instructions.append(&mut instructions);
    all_instructions.append(&mut cleanup());

    all_instructions
        .into_iter()
        .map(|code| format!("{code}"))
        .collect::<Vec<String>>()
        .join("\n")
}
