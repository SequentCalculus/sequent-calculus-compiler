use super::examples::Example;
use std::{
    fs,
    fs::{read_dir, read_to_string, File},
    io::prelude::Read,
    path::PathBuf,
};

pub struct AllExamples {
    pub examples: Vec<Example>,
    pub success_examples: Vec<(String, String)>,
    pub fail_examples: Vec<(String, String)>,
}

pub fn load_all() -> Result<AllExamples, String> {
    let examples = load_examples()?;
    let success_examples = load_success()?;
    let fail_examples = load_fail()?;
    Ok(AllExamples {
        examples,
        success_examples,
        fail_examples,
    })
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

pub fn load_success() -> Result<Vec<(String, String)>, String> {
    let dir = PathBuf::from("testsuite/success");
    let mut examples = vec![];
    let dir_entries =
        read_dir(dir).map_err(|err| format!("Could not load typecheck examples: {err}"))?;
    for example in dir_entries {
        let dir_entry = example.map_err(|err| format!("Could not get dir entry: {err}"))?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .ok_or(format!("Could not get file name for path {path:?}"))?;
        let example_name = file_name
            .to_str()
            .ok_or(format!("Could not convert {file_name:?} to string"))?;
        let contents = read_to_string(path.clone())
            .map_err(|err| format!("Could not load example contents: {err}"))?;
        examples.push((example_name.to_owned(), contents));
    }
    Ok(examples)
}

pub fn load_fail() -> Result<Vec<(String, String)>, String> {
    let dir = PathBuf::from("testsuite/fail_check");
    let mut examples = vec![];
    let dir_entries =
        read_dir(dir).map_err(|err| format!("Could not load fail examples: {err}"))?;
    for example in dir_entries {
        let dir_entry = example.map_err(|err| format!("Could not get dir entry: {err}"))?;
        let path = dir_entry.path();
        let file_name = path
            .file_name()
            .ok_or(format!("Could not get file name for path {path:?}"))?;
        let example_name = file_name.to_str().ok_or(format!(
            "Could not convert file name {file_name:?} to string"
        ))?;
        let contents = read_to_string(path.clone())
            .map_err(|err| format!("Could not read example contents: {err}"))?;
        examples.push((example_name.to_owned(), contents));
    }
    Ok(examples)
}
