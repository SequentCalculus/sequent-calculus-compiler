use super::{end_to_end_tests::EndToEndTest, errors::Error};

use driver::paths::{BENCHMARKS_PATH, EXAMPLES_PATH};

use std::{
    fs,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub struct AllTests {
    pub end_to_end_tests: Vec<EndToEndTest>,
    pub success_tests: Vec<(String, String)>,
    pub fail_tests: Vec<(String, String)>,
}

pub fn load_all() -> Result<AllTests, Error> {
    let mut end_to_end_tests = load_end_to_end_tests(EXAMPLES_PATH)?;
    end_to_end_tests.extend(load_end_to_end_tests(BENCHMARKS_PATH)?);
    end_to_end_tests.extend(load_end_to_end_tests("testsuite/end_to_end")?);
    let success_tests = load_micro_tests("testsuite/success_check")?;
    let fail_tests = load_micro_tests("testsuite/fail_check")?;
    Ok(AllTests {
        end_to_end_tests,
        success_tests,
        fail_tests,
    })
}

pub fn load_end_to_end_tests(path: &str) -> Result<Vec<EndToEndTest>, Error> {
    let mut tests = vec![];
    let tests_path = PathBuf::from(path);
    let dir_entries = fs::read_dir(&tests_path).map_err(|err| Error::read_dir(&tests_path, err))?;
    for entry in dir_entries {
        let entry = entry.map_err(|err| Error::read_dir(&tests_path, err))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let test = EndToEndTest::from_dir(path)?;
        tests.push(test);
    }
    Ok(tests)
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
