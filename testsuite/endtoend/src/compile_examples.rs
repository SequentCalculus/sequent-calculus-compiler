use driver::Driver;
use std::process::Command;

use super::examples::{Example, ExampleResult};

#[cfg(target_arch = "aarch64")]
fn run_test_aarch64(driver: &mut Driver, example: &Example) -> ExampleResult {
    let out_path = example.get_compiled_path(driver::paths::Paths::aarch64_binary_dir());
    match driver.compile_aarch64(&example.source_file, None) {
        Ok(_) => (),
        Err(err) => return example.to_fail(err),
    }

    let mut command = Command::new(&out_path);
    for arg in example.config.test.clone() {
        command.arg(arg);
    }
    let result = match command.output() {
        Ok(res) => res.stdout,
        Err(err) => return example.to_fail(err),
    };
    example.compare_output(result)
}

#[cfg(target_arch = "x86_64")]
fn run_test_x86_64(driver: &mut Driver, example: &Example) -> ExampleResult {
    let out_path = example.get_compiled_path(driver::paths::Paths::x86_64_binary_dir());
    match driver.compile_x86_64(&example.source_file, None) {
        Ok(_) => (),
        Err(err) => return example.to_fail(err),
    };

    let mut command = Command::new(&out_path);
    for arg in example.config.test.clone() {
        command.arg(arg);
    }
    let result = match command.output() {
        Ok(res) => res.stdout,
        Err(err) => return example.to_fail(err),
    };

    example.compare_output(result)
}

pub fn run_tests(examples: &Vec<Example>) -> Vec<ExampleResult> {
    let mut results = vec![];
    let mut driver = Driver::new();

    for example in examples {
        #[cfg(target_arch = "x86_64")]
        results.push(run_test_x86_64(&mut driver, example));

        #[cfg(target_arch = "aarch64")]
        results.push(run_test_aarch64(&mut driver, example));
    }
    results
}
