use driver::Driver;
use std::process::Command;

use super::examples::{Example, ExampleResult};

#[cfg(target_arch = "aarch64")]
fn aarch64_tests(driver: &mut Driver, paths: &Vec<Example>) -> Vec<ExampleResult> {
    let mut results = vec![];

    for example in paths.iter() {
        let out_path = example.get_compiled_path(driver::paths::Paths::aarch64_binary_dir());
        driver
            .compile_aarch64(&example.source_file, false)
            .expect("could not compile example");

        let result = Command::new(&out_path)
            .arg("10")
            .output()
            .expect("Could not run compiled binary")
            .stdout;
        results.push(example.compare_output)
    }
    results
}

#[cfg(target_arch = "x86_64")]
fn x86_84_tests(driver: &mut Driver, paths: &Vec<Example>) -> Vec<ExampleResult> {
    let mut results = vec![];

    for example in paths.iter() {
        let out_path = example.get_compiled_path(driver::paths::Paths::x86_64_binary_dir());
        driver
            .compile_x86_64(&example.source_file, false)
            .expect("Could not compile example");

        let result = Command::new(&out_path)
            .arg("10")
            .output()
            .expect("Could not run compiled binary")
            .stdout;
        results.push(example.compare_output(result));
    }
    results
}

pub fn run_tests(examples: &Vec<Example>) -> Vec<ExampleResult> {
    let mut results = vec![];
    let mut driver = Driver::new();
    #[cfg(target_arch = "x86_64")]
    results.extend(x86_84_tests(&mut driver, examples));

    #[cfg(target_arch = "aarch64")]
    results.extend(aarch64_tests(&mut driver, examples));

    results
}
