use axcut2backend::{code::pretty, coder::AssemblyProg};

use crate::code::Code;

// The setup for the assembly routine probably depends on the hardware used.
#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_rv64_routine(prog: AssemblyProg<Code>) -> String {
    let program = pretty(prog.instructions);
    let mut code = Vec::new();
    code.push("// actual code".to_string() + &program);
    code.push("cleanup:".to_string());
    code.join("\n\n")
}
