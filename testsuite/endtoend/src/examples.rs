use super::errors::Error;
use super::fun_tests::{TestResult, TestType};

use std::{fs::read_to_string, path::PathBuf};

#[derive(Clone, serde::Deserialize)]
pub struct ExampleConfig {
    pub test: Vec<String>,
    pub expected: String,
    pub heap_size: Option<usize>,
}

#[derive(Clone)]
pub struct Example {
    pub source_file: PathBuf,
    pub name: String,
    pub file_name: String,
    pub config: ExampleConfig,
}

impl Example {
    pub fn from_dir(dir: PathBuf) -> Result<Example, Error> {
        if dir.is_file() {
            return Err(Error::DirIsFile { path: dir });
        }
        let name = dir
            .file_stem()
            .ok_or(Error::path_access(&dir, "File Stem"))?
            .to_str()
            .ok_or(Error::path_access(&dir, "File Stem as String"))?
            .to_owned();

        let mut source_file = dir.join(&name);
        source_file.set_extension("sc");

        let file_name = source_file
            .file_name()
            .ok_or(Error::path_access(&source_file, "File Name"))?
            .to_str()
            .ok_or(Error::path_access(&source_file, "File Name as String"))?
            .to_owned();

        let mut args_path = dir.join(&name);
        args_path.set_extension("args");
        let args_contents = read_to_string(args_path.clone())
            .map_err(|err| Error::file_access(&args_path, "Read File", err))?;
        let mut config = basic_toml::from_str::<ExampleConfig>(&args_contents)
            .map_err(|err| Error::parse_toml(&args_path, err))?;
        config.expected.push('\n');

        Ok(Example {
            source_file,
            name,
            file_name,
            config,
        })
    }

    pub fn get_compiled_path(&self, out_base: PathBuf) -> PathBuf {
        let mut path = out_base;
        path.push(self.file_name.clone());
        path.set_extension("");

        path
    }

    pub fn compare_output(&self, result: Vec<u8>) -> TestResult {
        let expected_u8 = self.config.expected.clone().into_bytes();
        let fail_msg = if result == expected_u8 {
            None
        } else {
            let found_str = String::from_utf8(result.clone()).unwrap_or(format!("{:?}", result));
            Some(format!(
                "Example {} did not give expected result: expected {:?}, got {:?}. ",
                self.name, self.config.expected, found_str
            ))
        };
        TestResult::new(self.name.clone(), TestType::Compile, fail_msg)
    }

    pub fn to_fail<T: std::error::Error>(&self, err: T) -> TestResult {
        TestResult::new(self.name.clone(), TestType::Compile, Some(err.to_string()))
    }
}
