use super::{errors::Error, examples::Example};
use driver::paths::BENCHMARKS_PATH;
use std::{
    fs,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub struct AllExamples {
    pub examples: Vec<Example>,
    pub success_examples: Vec<(String, String)>,
    pub fail_examples: Vec<(String, String)>,
}

pub fn load_all() -> Result<AllExamples, Error> {
    let examples = load_examples()?;
    let success_examples = load_success()?;
    let fail_examples = load_fail()?;
    Ok(AllExamples {
        examples,
        success_examples,
        fail_examples,
    })
}

pub fn load_examples() -> Result<Vec<Example>, Error> {
    let mut paths = vec![];
    let examples_path = PathBuf::from(driver::paths::EXAMPLES_PATH);
    let dir_entries =
        fs::read_dir(&examples_path).map_err(|err| Error::read_dir(&examples_path, err))?;
    for entry in dir_entries {
        let entry = entry.map_err(|err| Error::read_dir(&examples_path, err))?;
        let path = entry.path();
        let example = Example::from_dir(path)?;
        paths.push(example);
    }
    Ok(paths)
}

pub fn load_success() -> Result<Vec<(String, String)>, Error> {
    let dir = PathBuf::from("testsuite/success");
    let mut examples = vec![];
    let dir_entries = read_dir(&dir).map_err(|err| Error::read_dir(&dir, err))?;
    for example in dir_entries {
        let dir_entry = example.map_err(|err| Error::read_dir(&dir, err))?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .ok_or(Error::path_access(&path, "File Name"))?;
        let example_name = file_name
            .to_str()
            .ok_or(Error::path_access(&path, "File Name as String"))?;
        let contents =
            read_to_string(path.clone()).map_err(|err| Error::file_access(&path, "read", err))?;
        examples.push((example_name.to_owned(), contents));
    }
    Ok(examples)
}

pub fn load_fail() -> Result<Vec<(String, String)>, Error> {
    let dir = PathBuf::from("testsuite/fail_check");
    let mut examples = vec![];
    let dir_entries = read_dir(&dir).map_err(|err| Error::read_dir(&dir, err))?;
    for example in dir_entries {
        let dir_entry = example.map_err(|err| Error::read_dir(&dir, err))?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .ok_or(Error::path_access(&path, "File Name"))?;
        let example_name = file_name
            .to_str()
            .ok_or(Error::path_access(&path, "File Name as String"))?;
        let contents =
            read_to_string(path.clone()).map_err(|err| Error::file_access(&path, "read", err))?;
        examples.push((example_name.to_owned(), contents));
    }
    Ok(examples)
}

pub fn load_bench() -> Result<Vec<Example>, Error> {
    let bench_dir = PathBuf::from(BENCHMARKS_PATH);
    let mut examples = vec![];
    let dir_entries = read_dir(&bench_dir).map_err(|err| Error::read_dir(&bench_dir, err))?;
    for benchmark in dir_entries {
        let entry = benchmark.map_err(|err| Error::read_dir(&bench_dir, err))?;
        let bench_path = entry.path();
        let example = Example::from_dir(bench_path)?;
        examples.push(example);
    }
    Ok(examples)
}
