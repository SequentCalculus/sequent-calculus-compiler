//! This module implements the plumbing for generating a complete assembly routine.

use axcut2backend::coder::AssemblyProg;

use crate::code::Code;

/// This function is supposed to turn an [`axcut2backend::coder::AssemblyProg`] generated from an
/// AxCut program by [`axcut2backend::coder::compile`] into a complete assembly routine which can
/// then be printed to a file, assembled and linked with a driver and some runtime functions.
/// Currently, this function is a stub. It needs to be adapted for every specific hardware platform.
#[allow(clippy::vec_init_then_push)]
pub fn into_rv64_routine(prog: AssemblyProg<Code>) -> String {
    let program = prog
        .instructions
        .into_iter()
        .map(|code| format!("{code}"))
        .collect::<Vec<String>>()
        .join("\n");
    let mut code = Vec::new();
    code.push("// actual code".to_string() + &program);
    // we need a cleanup label at the end, because the code for an `exit` statement jumps there
    code.push("cleanup:".to_string());
    code.join("\n\n")
}
