use std::{
    ffi::OsString,
    fmt, fs,
    fs::{read_dir, read_to_string, File},
    io::prelude::Read,
    path::PathBuf,
};

#[derive(Clone)]
pub struct Example {
    pub source_file: PathBuf,
    pub example_name: String,
    pub expected_result: Vec<u8>,
}

impl Example {
    pub fn get_compiled_path(&self, out_base: PathBuf) -> PathBuf {
        let mut path = out_base;
        let file_name = self
            .source_file
            .file_name()
            .expect("Could not get example file name");
        path.push(file_name);
        path.set_extension("");

        path
    }
}

pub enum ExampleType {
    Parse,
    Reparse,
    Typecheck,
    Compile,
}

pub struct ExampleResult {
    pub name: String,
    pub ty: ExampleType,
    pub fail_msg: Option<String>,
}

impl ExampleResult {
    pub fn new(example_name: String, ty: ExampleType, result: Option<String>) -> ExampleResult {
        ExampleResult {
            name: example_name,
            ty,
            fail_msg: result,
        }
    }

    pub fn assert_success(results: Vec<ExampleResult>) {
        let mut err_msg = "".to_owned();
        let mut assertion = true;
        for result in results {
            let fail = result.fail_msg.is_some();
            assertion = assertion && !fail;
            if fail {
                err_msg += &format!(
                    "Failed to {} example {:?}: {}\n",
                    result.ty,
                    result.name,
                    result.fail_msg.unwrap()
                );
            }
        }
        assert!(assertion, "{}", err_msg)
    }
}

impl fmt::Display for ExampleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExampleType::Parse => f.write_str("parse"),
            ExampleType::Reparse => f.write_str("reparse"),
            ExampleType::Typecheck => f.write_str("typecheck"),
            ExampleType::Compile => f.write_str("compile"),
        }
    }
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
