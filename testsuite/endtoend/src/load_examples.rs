use super::{errors::Error, examples::Example};

use driver::paths::{BENCHMARKS_PATH, EXAMPLES_PATH};

use std::{
    fs,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub struct AllTests {
    pub examples: Vec<Example>,
    pub success_tests: Vec<(String, String)>,
    pub fail_tests: Vec<(String, String)>,
}

pub fn load_all() -> Result<AllTests, Error> {
    let mut examples = load_examples(EXAMPLES_PATH)?;
    examples.extend(load_examples(BENCHMARKS_PATH)?);
    let success_tests = load_micro_tests("testsuite/success_check")?;
    let fail_tests = load_micro_tests("testsuite/fail_check")?;
    Ok(AllTests {
        examples,
        success_tests,
        fail_tests,
    })
}

pub fn load_examples(path: &str) -> Result<Vec<Example>, Error> {
    let mut examples = vec![];
    let examples_path = PathBuf::from(path);
    let dir_entries =
        fs::read_dir(&examples_path).map_err(|err| Error::read_dir(&examples_path, err))?;
    for entry in dir_entries {
        let entry = entry.map_err(|err| Error::read_dir(&examples_path, err))?;
        let path = entry.path();
        let example = Example::from_dir(path)?;
        examples.push(example);
    }
    Ok(examples)
}

pub fn load_micro_tests(path: &str) -> Result<Vec<(String, String)>, Error> {
    let mut tests = vec![];
    let dir = PathBuf::from(path);
    let dir_entries = read_dir(&dir).map_err(|err| Error::read_dir(&dir, err))?;
    for entry in dir_entries {
        let dir_entry = entry.map_err(|err| Error::read_dir(&dir, err))?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .ok_or(Error::path_access(&path, "File Name"))?;
        let test_name = file_name
            .to_str()
            .ok_or(Error::path_access(&path, "File Name as String"))?;
        let contents =
            read_to_string(path.clone()).map_err(|err| Error::file_access(&path, "read", err))?;
        tests.push((test_name.to_string(), contents));
    }
    Ok(tests)
}
