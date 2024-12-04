use driver::Driver;
use std::process::Command;

use super::examples::{Example, ExampleResult};

#[cfg(target_arch = "aarch64")]
fn run_test_aarch64(driver: &mut Driver, example: &Example) -> ExampleResult {
    let out_path = example.get_compiled_path(driver::paths::Paths::aarch64_binary_dir());
    let comp_res = driver.compile_aarch64(&example.source_file, false);
    if comp_res.is_err() {
        return example.to_fail(comp_res.unwrap_err());
    }

    let result = Command::new(&out_path).arg("10").output();
    if result.is_err() {
        return example.to_fail(result.unwrap_err());
    }
    example.compare_output(result.unwrap().stdout)
}

#[cfg(target_arch = "x86_64")]
fn run_test_x86_64(driver: &mut Driver, example: &Example) -> ExampleResult {
    let out_path = example.get_compiled_path(driver::paths::Paths::x86_64_binary_dir());
    let comp_res = driver.compile_x86_64(&example.source_file, false);
    if comp_res.is_err() {
        return example.to_fail(comp_res.unwrap_err());
    }

    let result = Command::new(&out_path).arg("10").output();
    if result.is_err() {
        return example.to_fail(result.unwrap_err());
    }

    example.compare_output(result.unwrap().stdout)
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
