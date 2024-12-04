use std::{
    ffi::OsString,
    fs,
    fs::{read_dir, read_to_string, File},
    io::prelude::Read,
    path::PathBuf,
};

pub struct Example {
    pub source_file: PathBuf,
    pub example_name: String,
    pub expected_result: Vec<u8>,
}

pub fn load_examples() -> Vec<Example> {
    let mut paths = vec![];
    let examples_path = PathBuf::from(driver::paths::EXAMPLES_PATH);
    let expected_path = PathBuf::from(driver::paths::EXPECTED_PATH);
    for entry in fs::read_dir(examples_path).expect("Could not load examples directory") {
        let file_path = entry.expect("Could not get filepath for example").path();
        let extension = file_path.extension().expect("Could not get file extension");

        if extension != "sc" {
            continue;
        }

        let example_name = file_path
            .file_stem()
            .expect("Could not get example name")
            .to_str()
            .expect("Could not read file name")
            .to_owned();

        let mut expected_path = expected_path.clone();
        expected_path.push(
            file_path
                .file_name()
                .expect("Could not get example file name"),
        );
        expected_path.set_extension("expected");
        let mut expected_file =
            File::open(&expected_path).expect("Could not open file for expected output");
        let mut expected_result = Vec::new();
        expected_file
            .read_to_end(&mut expected_result)
            .expect("Could not read expected output");

        paths.push(Example {
            source_file: file_path,
            example_name,
            expected_result,
        });
    }
    paths
}

pub fn load_success() -> Vec<(Box<OsString>, String)> {
    let dir = PathBuf::from("testsuite/success");
    let mut examples = vec![];
    for example in read_dir(dir).expect("Could not load test suite") {
        let path = example.expect("Could not load example").path();
        let example_name = path
            .file_name()
            .expect("Could not load file name")
            .to_owned();
        let contents = read_to_string(path).expect("Could not read example");
        examples.push((Box::new(example_name), contents));
    }
    examples
}

pub fn load_fail() -> Vec<(Box<OsString>, String)> {
    let dir = PathBuf::from("testsuite/fail_check");
    let mut examples = vec![];
    for example in read_dir(dir).expect("Could not load test suite") {
        let path = example.expect("Could not load example").path();
        let example_name = path
            .file_name()
            .expect("Could not load file name")
            .to_owned();
        let contents = read_to_string(path).expect("Could not read example");
        examples.push((Box::new(example_name), contents));
    }
    examples
}
