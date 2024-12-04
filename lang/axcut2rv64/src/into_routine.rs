use axcut2backend::coder::AssemblyProg;

use crate::code::Code;

// The setup for the assembly routine probably depends on the hardware used.
#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_rv64_routine(prog: AssemblyProg<Code>) -> String {
    let program = prog
        .instructions
        .into_iter()
        .map(|code| format!("{code}"))
        .collect::<Vec<String>>()
        .join("\n");
    let mut code = Vec::new();
    code.push("// actual code".to_string() + &program);
    code.push("cleanup:".to_string());
    code.join("\n\n")
}
