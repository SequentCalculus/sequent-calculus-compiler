use std::{
    fmt, fs,
    fs::{read_dir, read_to_string, File},
    io::prelude::Read,
    path::PathBuf,
};

#[derive(Clone)]
pub struct Example {
    pub source_file: PathBuf,
    pub example_name: String,
    pub file_name: String,
    pub expected_result: Vec<u8>,
}

impl Example {
    pub fn get_compiled_path(&self, out_base: PathBuf) -> PathBuf {
        let mut path = out_base;
        path.push(self.file_name.clone());
        path.set_extension("");

        path
    }

    pub fn compare_output(&self, result: Vec<u8>) -> ExampleResult {
        let fail_msg = if result == self.expected_result {
            None
        } else {
            Some(format!(
                "Example {} did not give expected result: expected {:?}, got {:?}. ",
                self.example_name, self.expected_result, result
            ))
        };
        ExampleResult::new(self.example_name.clone(), ExampleType::Compile, fail_msg)
    }

    pub fn to_fail<T: std::error::Error>(&self, err: T) -> ExampleResult {
        ExampleResult::new(
            self.example_name.clone(),
            ExampleType::Compile,
            Some(err.to_string()),
        )
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
pub fn load_examples() -> Result<Vec<Example>, String> {
    let mut paths = vec![];
    let examples_path = PathBuf::from(driver::paths::EXAMPLES_PATH);
    let expected_path = PathBuf::from(driver::paths::EXPECTED_PATH);
    let dir_entries =
        fs::read_dir(examples_path).map_err(|err| format!("Could not load examples: {err}"))?;
    for entry in dir_entries {
        let file_entry = entry.map_err(|err| format!("Could not get file path for: {err}"))?;
        let file_path = file_entry.path();
        let extension = file_path
            .extension()
            .ok_or(format!("Could not get file extension for {file_path:?}"))?;

        if extension != "sc" {
            continue;
        }

        let file_stem = file_path
            .file_stem()
            .ok_or(format!("Could not get file stem for {file_path:?}"))?;
        let example_name = file_stem
            .to_str()
            .ok_or(format!("Could not get name for {file_path:?}"))?;

        let file_name = file_path
            .file_name()
            .ok_or(format!("Could not get file name for {example_name:?}"))?;
        let file_name_str = file_name
            .to_str()
            .ok_or(format!("Could not get file name {file_name:?} as string"))?;

        let mut expected_path = expected_path.clone();
        expected_path.push(file_name);
        expected_path.set_extension("expected");

        let mut expected_file = File::open(&expected_path)
            .map_err(|err| format!("Could not open expected file {expected_path:?}: {err}"))?;
        let mut expected_result = Vec::new();
        expected_file
            .read_to_end(&mut expected_result)
            .map_err(|err| format!("Could not read expected file {expected_path:?}: {err}"))?;

        paths.push(Example {
            source_file: file_path.clone(),
            file_name: file_name_str.to_owned(),
            example_name: example_name.to_owned(),
            expected_result,
        });
    }
    Ok(paths)
}

pub fn load_success() -> Vec<(String, String)> {
    let dir = PathBuf::from("testsuite/success");
    let mut examples = vec![];
    for example in read_dir(dir).expect("Could not load test suite") {
        let path = example.expect("Could not load example").path();
        let example_name = path
            .file_name()
            .expect("Could not load file name")
            .to_str()
            .expect("Could not get file name string")
            .to_owned();
        let contents = read_to_string(path).expect("Could not read example");
        examples.push((example_name, contents));
    }
    examples
}

pub fn load_fail() -> Vec<(String, String)> {
    let dir = PathBuf::from("testsuite/fail_check");
    let mut examples = vec![];
    for example in read_dir(dir).expect("Could not load test suite") {
        let path = example.expect("Could not load example").path();
        let example_name = path
            .file_name()
            .expect("Could not load file name")
            .to_str()
            .expect("Could not get filename string")
            .to_owned();
        let contents = read_to_string(path).expect("Could not read example");
        examples.push((example_name, contents));
    }
    examples
}
