use crate::code::Code;

use super::config::{
    arg, field_offset, Register, FIELDS_PER_BLOCK, FREE, HEAP, SPILL_SPACE, STACK, TEMP,
};

use axcut2backend::{coder::AssemblyProg, config::TemporaryNumber::Fst};

pub fn preamble() -> Vec<Code> {
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
        COMMENT("setup".to_string()),
        COMMENT("save registers".to_string()),
        PUSH(Register::rbx()),
        PUSH(Register::rbp()),
        PUSH(Register(12)),
        PUSH(Register(13)),
        PUSH(Register(14)),
        PUSH(Register(15)),
    ]
}

fn move_params(n: usize, instructions: &mut Vec<Code>) {
    match n {
        0 => {}
        1 => instructions.push(Code::MOV(Register(5), arg(1))),
        2 => {
            instructions.push(Code::MOV(Register(7), arg(2)));
            move_params(1, instructions)
        }
        3 => {
            instructions.push(Code::MOV(Register(9), TEMP));
            move_params(2, instructions);
        }
        4 => {
            instructions.push(Code::MOV(Register(11), arg(4)));
            move_params(3, instructions);
        }
        5 => {
            instructions.push(Code::MOV(Register(13), arg(5)));
            move_params(4, instructions);
        }
        _ => panic!("too many arguments for main"),
    }
}

pub fn cleanup() -> Vec<Code> {
    use Code::*;
    vec![
        COMMENT("cleanup".to_string()),
        LAB("cleanup".to_string()),
        COMMENT("free space for register spills".to_string()),
        ADDI(STACK, SPILL_SPACE),
        COMMENT("restore registers".to_string()),
        POP(Register(15)),
        POP(Register(14)),
        POP(Register(13)),
        POP(Register(12)),
        POP(Register::rbp()),
        POP(Register::rbx()),
        RET,
    ]
}

#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_x86_64_routine(prog: AssemblyProg<Code>) -> String {
    let AssemblyProg {
        mut instructions,
        number_of_arguments,
    } = prog;

    let mut all_instructions: Vec<Code> = Vec::new();
    all_instructions.push(Code::COMMENT("asmsyntax=nasm".to_string()));
    all_instructions.append(&mut preamble());
    all_instructions.push(Code::COMMENT(
        "reserve space for register spills".to_string(),
    ));
    all_instructions.push(Code::SUBI(STACK, SPILL_SPACE));
    all_instructions.push(Code::COMMENT("initialize heap pointer".to_string()));
    all_instructions.push(Code::MOV(HEAP, arg(0)));
    all_instructions.push(Code::COMMENT("initialize free pointer".to_string()));
    all_instructions.push(Code::MOV(FREE, HEAP));
    all_instructions.push(Code::ADDI(FREE, field_offset(Fst, FIELDS_PER_BLOCK)));
    all_instructions.push(Code::COMMENT("move parameters into place".to_string()));
    move_params(number_of_arguments, &mut all_instructions);

    all_instructions.push(Code::COMMENT("actual code".to_string()));
    all_instructions.append(&mut instructions);
    all_instructions.append(&mut cleanup());

    all_instructions
        .into_iter()
        .map(|code| format!("{code}"))
        .collect::<Vec<String>>()
        .join("\n")
}
