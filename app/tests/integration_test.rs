use assert_cmd::Command;

/// The name of the CLI binary
const BINARY: &str = env!("CARGO_BIN_EXE_scc");

/// Check that "scc --version" works correctly
#[test]
fn version_command() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd.arg("--version").assert();
    assert.success().stdout("scc 0.1.0\n");
}

/// Check that "scc compile" works correctly
#[test]
fn compile_command() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd
        .args(vec!["compile", "../examples/Tuples/Tuples.sc"])
        .assert();
    assert.success();
}

/// Check that "scc check" works correctly
#[test]
fn check_command() {
    let mut cmd = Command::cargo_bin(BINARY).unwrap();
    let assert = cmd
        .args(vec!["check", "../examples/Tuples/Tuples.sc"])
        .assert();
    assert.success();
}
