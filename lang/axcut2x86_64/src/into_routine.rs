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

fn setup(arg_num: usize) -> String {
    const PREAMBLE: &str = "segment .text
  global asm_main0, _asm_main0
  global asm_main1, _asm_main1
  global asm_main2, _asm_main2
  global asm_main3, _asm_main3
  global asm_main4, _asm_main4
  global asm_main5, _asm_main5
asm_main0:
_asm_main0:
asm_main1:
_asm_main1:
asm_main2:
_asm_main2:
asm_main3:
_asm_main3:
asm_main4:
_asm_main4:
asm_main5:
_asm_main5:
; setup
; save registers
push rbx
push rbp
push r12
push r13
push r14
push r15\n";

    fn move_params(n: usize) -> String {
        match n {
            0 => String::new(),
            1 => format!("mov {}, {}", Register(5), arg(1)),
            2 => format!("mov {}, {}\n", Register(7), arg(2)) + &move_params(1),
            3 => format!("mov {}, {}\n", Register(9), TEMP) + &move_params(2),
            4 => format!("mov {}, {}\n", Register(11), arg(4)) + &move_params(3),
            5 => format!("mov {}, {}\n", Register(13), arg(5)) + &move_params(4),
            _ => panic!("too many arguments for main"),
        }
    }

    let mut setup = Vec::new();
    setup.push(PREAMBLE.to_string());
    setup.push("; reserve space for register spills".to_string());
    setup.push(format!("sub {STACK}, {SPILL_SPACE}"));
    setup.push("; initialize heap pointer".to_string());
    setup.push(format!("mov {HEAP}, {}", arg(0)));
    setup.push("; initialize free pointer".to_string());
    setup.push(format!("mov {FREE}, {HEAP}"));
    setup.push(format!(
        "add {}, {}",
        FREE,
        field_offset(Fst, FIELDS_PER_BLOCK)
    ));
    setup.push("; move parameters into place".to_string());
    setup.push(move_params(arg_num));
    setup.join("\n")
}

fn cleanup() -> String {
    const CLEANUP_LABEL: &str = "; cleanup
cleanup:";
    const RESTORE_REGISTERS: &str = "; restore registers
pop r15
pop r14
pop r13
pop r12
pop rbp
pop rbx
ret";

    let mut cleanup = Vec::new();
    cleanup.push(CLEANUP_LABEL.to_string());
    cleanup.push("; free space for register spills".to_string());
    cleanup.push(format!("add {STACK}, {SPILL_SPACE}"));
    cleanup.push(RESTORE_REGISTERS.to_string());
    cleanup.join("\n")
}

#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_x86_64_routine(prog: AssemblyProg<Code>) -> String {
    let program = prog
        .instructions
        .into_iter()
        .map(|code| format!("{code}"))
        .collect::<Vec<String>>()
        .join("\n");
    let mut code = Vec::new();
    code.push("; asmsyntax=nasm".to_string());
    code.push(setup(prog.number_of_arguments));
    code.push("; actual code".to_string() + &program);
    code.push(cleanup());
    code.join("\n\n")
}
