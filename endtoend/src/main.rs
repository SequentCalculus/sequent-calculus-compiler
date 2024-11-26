use driver::Driver;
use std::{fs, path::PathBuf, process::Command, str};

const EXAMPLES_PATH: &str = "examples";
const EXPECTED_PATH: &str = "examples_expected";

struct ExamplePaths {
    pub source_file: PathBuf,
    pub expected_file: PathBuf,
    pub out_file: PathBuf,
}

fn get_file_paths() -> Vec<ExamplePaths> {
    let mut paths = vec![];
    let examples_path = PathBuf::from(EXAMPLES_PATH);
    let expected_path = PathBuf::from(EXPECTED_PATH);
    let out_path = PathBuf::from(driver::paths::TARGET_PATH)
        .join(driver::paths::BIN_PATH)
        .join(driver::paths::X86_64_PATH);

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

        let mut out = out_path.clone();
        out.push(file_name);
        out.set_extension("");
        paths.push(ExamplePaths {
            source_file: file_path,
            expected_file: expected,
            out_file: out,
        });
    }
    paths
}

fn main() {
    let paths = get_file_paths();
    let mut driver = Driver::new();

    for example in paths.iter() {
        driver
            .compile_x86_64(&example.source_file, false)
            .expect("Could not compile example {file_path:?}");
        let run_out = Command::new(&example.out_file)
            .output()
            .expect("Coult not run compiled program");
        let result = str::from_utf8(&run_out.stdout)
            .expect("Could not parse output")
            .trim();
        let expected =
            fs::read_to_string(&example.expected_file).expect("Could not read expected output");
        assert_eq!(result, expected.trim())
    }
}
