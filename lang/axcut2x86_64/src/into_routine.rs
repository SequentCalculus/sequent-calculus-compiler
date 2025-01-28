use super::config::{
    arg, field_offset, Register, FIELDS_PER_BLOCK, FREE, HEAP, SPILL_SPACE, STACK,
};
use crate::{
    code::{Code, Codes},
    config::Immediate,
};
use axcut2backend::{coder::AssemblyProg, config::TemporaryNumber::Fst};
use printer::tokens::PRINTLN_I64;

#[must_use]
pub fn preamble() -> Vec<Code> {
    use Code::*;
    vec![
        NOEXECSTACK,
        TEXT,
        EXTERN(PRINTLN_I64.to_string()),
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
    ]
}

fn move_arguments(number_of_arguments: usize, instructions: &mut Vec<Code>) {
    instructions.push(Code::COMMENT("move parameters into place".to_string()));
    match number_of_arguments {
        0 => {}
        1 => instructions.push(Code::MOV(Register(5), arg(1))),
        2 => {
            instructions.push(Code::MOV(Register(7), arg(2)));
            move_arguments(1, instructions);
        }
        3 => {
            instructions.push(Code::MOV(Register(9), arg(3)));
            move_arguments(2, instructions);
        }
        4 => {
            instructions.push(Code::MOV(Register(11), arg(4)));
            move_arguments(3, instructions);
        }
        5 => {
            instructions.push(Code::MOV(Register(13), arg(5)));
            move_arguments(4, instructions);
        }
        _ => panic!("too many arguments for main"),
    }
}

pub fn setup(number_of_arguments: usize, instructions: &mut Vec<Code>) {
    use Code::*;
    instructions.push(COMMENT("setup".to_string()));
    instructions.push(COMMENT("save registers".to_string()));
    instructions.push(PUSH(Register::rbx()));
    instructions.push(PUSH(Register::rbp()));
    instructions.push(PUSH(Register(12)));
    instructions.push(PUSH(Register(13)));
    instructions.push(PUSH(Register(14)));
    instructions.push(PUSH(Register(15)));
    instructions.push(COMMENT("reserve space for register spills".to_string()));
    instructions.push(SUBI(STACK, Immediate { val: SPILL_SPACE }));
    instructions.push(COMMENT("initialize heap pointer".to_string()));
    instructions.push(MOV(HEAP, arg(0)));
    instructions.push(COMMENT("initialize free pointer".to_string()));
    instructions.push(MOV(FREE, HEAP));
    instructions.push(ADDI(FREE, field_offset(Fst, FIELDS_PER_BLOCK)));
    move_arguments(number_of_arguments, instructions);
}

#[must_use]
pub fn cleanup() -> Vec<Code> {
    use Code::*;
    vec![
        COMMENT("cleanup".to_string()),
        LAB("cleanup".to_string()),
        COMMENT("free space for register spills".to_string()),
        ADDI(STACK, Immediate { val: SPILL_SPACE }),
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
pub fn into_x86_64_routine(prog: AssemblyProg<Code>) -> Codes {
    let AssemblyProg {
        mut instructions,
        number_of_arguments,
    } = prog;

    let mut all_instructions: Vec<Code> = Vec::new();
    all_instructions.push(Code::COMMENT("asmsyntax=nasm".to_string()));
    all_instructions.append(&mut preamble());
    setup(number_of_arguments, &mut all_instructions);
    all_instructions.push(Code::COMMENT("actual code".to_string()));
    all_instructions.append(&mut instructions);
    all_instructions.append(&mut cleanup());
    Codes {
        instructions: all_instructions,
    }
}
