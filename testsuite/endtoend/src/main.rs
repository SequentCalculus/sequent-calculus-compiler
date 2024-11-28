use driver::Driver;
use std::{fs, fs::File, io::prelude::Read, path::PathBuf, process::Command};

struct ExamplePaths {
    pub source_file: PathBuf,
    pub expected_file: PathBuf,
}

fn get_file_paths() -> Vec<ExamplePaths> {
    let mut paths = vec![];
    let examples_path = PathBuf::from(driver::paths::EXAMPLES_PATH);
    let expected_path = PathBuf::from(driver::paths::EXPECTED_PATH);
    let path_contents = fs::read_dir(examples_path).expect("Could not find examples");
    for path in path_contents {
        let file_path = path.expect("Could not read filename").path();
        if file_path.extension().expect("Could not get file extension") != "sc" {
            continue;
        }

        let file_name = file_path.file_name().expect("Could not get file name");
        let mut expected = expected_path.clone();
        expected.push(file_name);
        expected.set_extension("expected");

        paths.push(ExamplePaths {
            source_file: file_path,
            expected_file: expected,
        });
    }
    paths
}

#[cfg(target_arch = "aarch64")]
fn aarch64_tests(paths: &Vec<ExamplePaths>) {
    let mut driver = Driver::new();

    for example in paths.iter() {
        // TODO: Fix the issue and add the example again.
        if example
            .source_file
            .to_str()
            .unwrap()
            .contains("FactorialAccumulator")
        {
            continue;
        }
        let path: &PathBuf = &example.source_file;
        driver
            .compile_aarch64(path, false)
            .expect("could not compile example");
        let mut out_path = driver::paths::Paths::aarch64_binary_dir();

        let file_name = path.file_name().expect("Could not get file name");
        out_path.push(file_name);
        out_path.set_extension("");

        let result = Command::new(&out_path)
            .arg("0")
            .output()
            .expect("Could not run compiled binary")
            .stdout;
        let mut expected_file =
            File::open(&example.expected_file).expect("Could not open file for expected output");
        let mut expected = Vec::new();
        expected_file
            .read_to_end(&mut expected)
            .expect("Could not read expected output");
        assert_eq!(result, expected)
    }
}

#[cfg(target_arch = "x86_64")]
fn x86_84_tests(paths: &Vec<ExamplePaths>) {
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
            .arg("0")
            .output()
            .expect("Could not run compiled binary")
            .stdout;
        let mut expected_file =
            File::open(&example.expected_file).expect("Could not open file for expected output");
        let mut expected = Vec::new();
        expected_file
            .read_to_end(&mut expected)
            .expect("Could not read expected output");
        assert_eq!(result, expected)
    }
}

fn main() {
    let working_dir = std::env::current_dir()
        .expect("Could not get working dir")
        .join("../../");
    std::env::set_current_dir(working_dir).expect("Could not set working dir");

    let paths = get_file_paths();

    #[cfg(target_arch = "x86_64")]
    x86_84_tests(&paths);

    #[cfg(target_arch = "aarch64")]
    aarch64_tests(&paths);
}
