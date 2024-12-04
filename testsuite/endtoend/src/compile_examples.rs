use driver::Driver;
use std::{path::PathBuf, process::Command};

use super::examples::Example;

#[cfg(target_arch = "aarch64")]
fn aarch64_tests(paths: &Vec<Example>) {
    let mut driver = Driver::new();

    for example in paths.iter() {
        let path: &PathBuf = &example.source_file;
        driver
            .compile_aarch64(path, false)
            .expect("could not compile example");
        let mut out_path = driver::paths::Paths::aarch64_binary_dir();

        let file_name = path.file_name().expect("Could not get file name");
        out_path.push(file_name);
        out_path.set_extension("");

        let result = Command::new(&out_path)
            .arg("10")
            .output()
            .expect("Could not run compiled binary")
            .stdout;
        assert_eq!(result, example.expected_result)
    }
}

#[cfg(target_arch = "x86_64")]
fn x86_84_tests(paths: &Vec<Example>) {
    let mut driver = Driver::new();

    for example in paths.iter() {
        let path: &PathBuf = &example.source_file;
        let mut out_path = driver::paths::Paths::x86_64_binary_dir();
        driver
            .compile_x86_64(path, false)
            .expect("Could not compile example");

        let file_name = path.file_name().expect("Could not get file name");
        out_path.push(file_name);
        out_path.set_extension("");

        let result = Command::new(&out_path)
            .arg("10")
            .output()
            .expect("Could not run compiled binary")
            .stdout;
        assert_eq!(result, example.expected_result)
    }
}

pub fn run_tests(examples: &Vec<Example>) {
    #[cfg(target_arch = "x86_64")]
    x86_84_tests(examples);

    #[cfg(target_arch = "aarch64")]
    aarch64_tests(examples);
}
