use crate::code::Code;

use super::config::{field_offset, Register, FIELDS_PER_BLOCK, FREE, HEAP};

use axcut2backend::{coder::AssemblyProg, config::TemporaryNumber::Fst};

fn setup_code() -> Vec<Code> {
    vec![
        Code::TEXT,
        Code::GLOBAL("asm_main0".to_string()),
        Code::GLOBAL("_asm_main0".to_string()),
        Code::GLOBAL("asm_main1".to_string()),
        Code::GLOBAL("_asm_main1".to_string()),
        Code::GLOBAL("asm_main2".to_string()),
        Code::GLOBAL("_asm_main2".to_string()),
        Code::GLOBAL("asm_main3".to_string()),
        Code::GLOBAL("_asm_main3".to_string()),
        Code::GLOBAL("asm_main4".to_string()),
        Code::GLOBAL("_asm_main4".to_string()),
        Code::GLOBAL("asm_main5".to_string()),
        Code::GLOBAL("_asm_main5".to_string()),
        Code::GLOBAL("asm_main6".to_string()),
        Code::GLOBAL("_asm_main6".to_string()),
        Code::GLOBAL("asm_main7".to_string()),
        Code::GLOBAL("_asm_main7".to_string()),
        Code::LAB("asm_main0".to_string()),
        Code::LAB("_asm_main0".to_string()),
        Code::LAB("asm_main1".to_string()),
        Code::LAB("_asm_main1".to_string()),
        Code::LAB("asm_main2".to_string()),
        Code::LAB("_asm_main2".to_string()),
        Code::LAB("asm_main3".to_string()),
        Code::LAB("_asm_main3".to_string()),
        Code::LAB("asm_main4".to_string()),
        Code::LAB("_asm_main4".to_string()),
        Code::LAB("asm_main5".to_string()),
        Code::LAB("_asm_main5".to_string()),
        Code::LAB("asm_main6".to_string()),
        Code::LAB("_asm_main6".to_string()),
        Code::LAB("asm_main7".to_string()),
        Code::LAB("_asm_main7".to_string()),
        Code::COMMENT("Setup".to_string()),
        Code::COMMENT("Save registers".to_string()),
        Code::STR_X(Register::X(16), Register::SP, -16),
        Code::STR_X(Register::X(17), Register::SP, -16),
        Code::STR_X(Register::X(18), Register::SP, -16),
        Code::STR_X(Register::X(19), Register::SP, -16),
        Code::STR_X(Register::X(20), Register::SP, -16),
        Code::STR_X(Register::X(21), Register::SP, -16),
        Code::STR_X(Register::X(22), Register::SP, -16),
        Code::STR_X(Register::X(23), Register::SP, -16),
        Code::STR_X(Register::X(24), Register::SP, -16),
        Code::STR_X(Register::X(25), Register::SP, -16),
        Code::STR_X(Register::X(26), Register::SP, -16),
        Code::STR_X(Register::X(27), Register::SP, -16),
        Code::STR_X(Register::X(28), Register::SP, -16),
        Code::STR_X(Register::X(29), Register::SP, -16),
        Code::STR_X(Register::X(30), Register::SP, -16),
    ]
}

fn initialize_free_pointer() -> Vec<Code> {
    vec![
        Code::COMMENT("Initialize free pointer".to_string()),
        Code::MOVR(FREE, HEAP),
        Code::ADDI(FREE, FREE, field_offset(Fst, FIELDS_PER_BLOCK)),
    ]
}
fn move_params(n: usize) -> Vec<Code> {
    match n {
        0 => vec![Code::COMMENT("Move parameters into place".to_string())],
        1 => {
            let mut instructions = vec![Code::MOVR(Register::X(4), Register::X(1))];
            instructions.append(&mut move_params(0));
            instructions
        }
        2 => {
            let mut instructions = vec![Code::MOVR(Register::X(6), Register::X(2))];
            instructions.append(&mut move_params(1));
            instructions
        }
        3 => {
            let mut instructions = vec![Code::MOVR(Register::X(8), Register::X(3))];
            instructions.append(&mut move_params(2));
            instructions
        }
        4 => {
            let mut instructions = vec![Code::MOVR(Register::X(10), Register::X(4))];
            instructions.append(&mut move_params(3));
            instructions
        }
        5 => {
            let mut instructions = vec![Code::MOVR(Register::X(12), Register::X(5))];
            instructions.append(&mut move_params(4));
            instructions
        }
        6 => {
            let mut instructions = vec![Code::MOVR(Register::X(14), Register::X(6))];
            instructions.append(&mut move_params(5));
            instructions
        }
        7 => {
            let mut instructions = vec![Code::MOVR(Register::X(16), Register::X(7))];
            instructions.append(&mut move_params(6));
            instructions
        }
        _ => panic!("too many arguments for main"),
    }
}

fn cleanup() -> Vec<Code> {
    vec![
        Code::COMMENT("Cleanup".to_string()),
        Code::LAB("cleanup".to_string()),
        Code::COMMENT("Restore registers".to_string()),
        Code::LDR_X(Register::X(30), Register::SP, 16),
        Code::LDR_X(Register::X(29), Register::SP, 16),
        Code::LDR_X(Register::X(28), Register::SP, 16),
        Code::LDR_X(Register::X(27), Register::SP, 16),
        Code::LDR_X(Register::X(26), Register::SP, 16),
        Code::LDR_X(Register::X(25), Register::SP, 16),
        Code::LDR_X(Register::X(24), Register::SP, 16),
        Code::LDR_X(Register::X(23), Register::SP, 16),
        Code::LDR_X(Register::X(22), Register::SP, 16),
        Code::LDR_X(Register::X(21), Register::SP, 16),
        Code::LDR_X(Register::X(20), Register::SP, 16),
        Code::LDR_X(Register::X(19), Register::SP, 16),
        Code::LDR_X(Register::X(18), Register::SP, 16),
        Code::LDR_X(Register::X(17), Register::SP, 16),
        Code::LDR_X(Register::X(16), Register::SP, 16),
        Code::RET,
    ]
}

#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_aarch64_routine(prog: AssemblyProg<Code>) -> String {
    let AssemblyProg {
        instructions,
        number_of_arguments,
    } = prog;

    let mut all_instructions: Vec<Code> = Vec::new();
    all_instructions.append(&mut setup_code());
    all_instructions.append(&mut move_params(number_of_arguments));
    all_instructions.append(&mut initialize_free_pointer());
    all_instructions.push(Code::COMMENT("Actual code".to_string()));
    all_instructions.append(&mut instructions.clone());
    all_instructions.append(&mut cleanup());

    all_instructions
        .into_iter()
        .map(|code| format!("{code}"))
        .collect::<Vec<String>>()
        .join("\n")
}
