// The setup for the assembly routine probably depends on the hardware used.
#[allow(clippy::vec_init_then_push)]
#[must_use]
pub fn into_rv64_routine(name: &str, program: &str, _arg_num: usize) -> String {
    let mut code = Vec::new();
    code.push(name.to_string());
    code.push("// actual code".to_string() + program);
    code.push("cleanup:".to_string());
    code.join("\n\n")
}
