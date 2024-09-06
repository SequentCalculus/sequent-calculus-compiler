use assert_cmd::Command;

/// The name of the CLI binary
const BINARY: &str = env!("CARGO_BIN_EXE_grokking");

/// Check that "grokking --version" works correctly
#[test]
fn version_command() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd.arg("--version").assert();
    assert.success().stdout("grokking 0.1.0\n");
}

/// Check that "grokking compile" works correctly
#[test]
fn compile_command() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd.args(vec!["compile", "../examples/Tuples.sc"]).assert();
    assert.success();
}
