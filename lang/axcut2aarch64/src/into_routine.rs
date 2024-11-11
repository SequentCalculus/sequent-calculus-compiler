use super::config::{field_offset, Register, FIELDS_PER_BLOCK, FREE, HEAP};

use axcut2backend::config::TemporaryNumber::Fst;

fn header(name: &str) -> String {
    let mut header = Vec::new();
    header.push("// To create an executable:".to_string());
    header.push(format!("// $ as -o {name}.aarch64.o {name}.aarch64.asm"));
    header.push(format!(
        "// $ gcc -o {name} path/to/AARCH64-infrastructure/driver$MODE.c {name}.aarch64.o"
    ));
    header.push("// where $MODE = Args | Debug".to_string());
    header.join("\n")
}

fn setup(arg_num: usize) -> String {
    const PREAMBLE: &str = ".text
  .global asm_main0, _asm_main0
  .global asm_main1, _asm_main1
  .global asm_main2, _asm_main2
  .global asm_main3, _asm_main3
  .global asm_main4, _asm_main4
  .global asm_main5, _asm_main5
  .global asm_main6, _asm_main6
  .global asm_main7, _asm_main7
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
asm_main6:
_asm_main6:
asm_main7:
_asm_main7:
// setup
// save registers
str X16, [sp, -16]!
str X17, [sp, -16]!
str X18, [sp, -16]!
str X19, [sp, -16]!
str X20, [sp, -16]!
str X21, [sp, -16]!
str X22, [sp, -16]!
str X23, [sp, -16]!
str X24, [sp, -16]!
str X25, [sp, -16]!
str X26, [sp, -16]!
str X27, [sp, -16]!
str X28, [sp, -16]!
str X29, [sp, -16]!
str X30, [sp, -16]!\n";

    fn move_params(n: usize) -> String {
        match n {
            0 => String::new(),
            1 => format!("MOV {}, {}", Register(4), Register(1)),
            2 => format!("MOV {}, {}\n", Register(6), Register(2)) + &move_params(1),
            3 => format!("MOV {}, {}\n", Register(8), Register(3)) + &move_params(2),
            4 => format!("MOV {}, {}\n", Register(10), Register(4)) + &move_params(3),
            5 => format!("MOV {}, {}\n", Register(12), Register(5)) + &move_params(4),
            6 => format!("MOV {}, {}\n", Register(14), Register(6)) + &move_params(5),
            7 => format!("MOV {}, {}\n", Register(16), Register(7)) + &move_params(6),
            _ => panic!("too many arguments for main"),
        }
    }

    let mut setup = Vec::new();
    setup.push(PREAMBLE.to_string());
    setup.push("// move parameters into place".to_string());
    setup.push(move_params(arg_num));
    setup.push("// initialize free pointer".to_string());
    setup.push(format!("MOV {FREE}, {HEAP}"));
    setup.push(format!(
        "ADD {}, {}, {}",
        FREE,
        FREE,
        field_offset(Fst, FIELDS_PER_BLOCK)
    ));
    setup.join("\n")
}

const CLEANUP: &str = "// cleanup
cleanup:
// restore registers
ldr X30, [sp], 16
ldr X29, [sp], 16
ldr X28, [sp], 16
ldr X27, [sp], 16
ldr X26, [sp], 16
ldr X25, [sp], 16
ldr X24, [sp], 16
ldr X23, [sp], 16
ldr X22, [sp], 16
ldr X21, [sp], 16
ldr X20, [sp], 16
ldr X19, [sp], 16
ldr X18, [sp], 16
ldr X17, [sp], 16
ldr X16, [sp], 16
ret";

#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_aarch64_routine(name: &str, program: &str, arg_num: usize) -> String {
    let mut code = Vec::new();
    code.push(header(name));
    code.push(setup(arg_num));
    code.push("// actual code".to_string() + program);
    code.push(CLEANUP.to_string());
    code.join("\n\n")
}
