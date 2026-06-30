use assert_cmd::Command;
use std::process::Command as StdCommand;

/// The name of the CLI binary, from Cargo
const BINARY: &str = env!("CARGO_BIN_EXE_scc");

/// Check that "scc --version" works correctly
#[test]
fn version_command() {
    // build a plain std::process::Command from the dynamic path
    let cmd = StdCommand::new(BINARY);
    // wrap in assert_cmd to use assertions
    let assert = Command::from(cmd).arg("--version").assert();
    assert.success().stdout("scc 0.1.0\n");
}

/// Check that "scc compile" works correctly
#[test]
fn compile_command() {
    let cmd = StdCommand::new(BINARY);
    let assert = Command::from(cmd)
        .args(vec!["compile", "../examples/Tuples/Tuples.sc"])
        .assert();
    assert.success();
}

/// Check that "scc check" works correctly
#[test]
fn check_command() {
    let cmd = StdCommand::new(BINARY);
    let assert = Command::from(cmd)
        .args(vec!["check", "../examples/Tuples/Tuples.sc"])
        .assert();
    assert.success();
}
