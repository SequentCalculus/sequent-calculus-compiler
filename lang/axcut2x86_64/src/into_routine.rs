use super::config::{
    arg, field_offset, Register, FIELDS_PER_BLOCK, FREE, HEAP, SPILL_SPACE, STACK, TEMP,
};

use axcut2backend::config::TemporaryNumber::Fst;

fn header(name: &str) -> String {
    let mut header = Vec::new();
    header.push("; asmsyntax=nasm\n;".to_string());
    header.push("; To create an executable:".to_string());
    header.push(format!("; $ nasm -f elf64 {name}.x86_64.asm"));
    header.push(format!(
        "; $ gcc -o {name} path/to/X86_64-infrastructure/driver$MODE.c {name}.x86_64.o"
    ));
    header.push("; where $MODE = Args | Debug".to_string());
    header.join("\n")
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
pub fn into_x86_64_routine(name: &str, program: &str, arg_num: usize) -> String {
    let mut code = Vec::new();
    code.push(header(name));
    code.push(setup(arg_num));
    code.push("; actual code".to_string() + program);
    code.push(cleanup());
    code.join("\n\n")
}
