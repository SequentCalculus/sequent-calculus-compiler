use super::errors::Error;
use std::{fmt, fs::read_to_string, path::PathBuf};

#[derive(Clone, serde::Deserialize)]
pub struct ExampleConfig {
    pub test: Vec<String>,
    pub expected: String,
    pub heap_size: Option<usize>,
}

#[derive(Clone)]
pub struct Example {
    pub source_file: PathBuf,
    pub example_name: String,
    pub file_name: String,
    pub config: ExampleConfig,
}

impl Example {
    pub fn from_dir(dir: PathBuf) -> Result<Example, Error> {
        if dir.is_file() {
            return Err(Error::FileIsDir { path: dir });
        }
        let example_name = dir
            .file_stem()
            .ok_or(Error::path_access(&dir, "File Stem"))?
            .to_str()
            .ok_or(Error::path_access(&dir, "File Stem as String"))?
            .to_owned();

        let mut source_file = dir.join(&example_name);
        source_file.set_extension("sc");

        let file_name = source_file
            .file_name()
            .ok_or(Error::path_access(&source_file, "File Name"))?
            .to_str()
            .ok_or(Error::path_access(&source_file, "File Name as String"))?
            .to_owned();

        let mut args_path = dir.join(&example_name);
        args_path.set_extension("args");
        let args_contents = read_to_string(args_path.clone())
            .map_err(|err| Error::file_access(&args_path, "Read File", err))?;
        let mut config = basic_toml::from_str::<ExampleConfig>(&args_contents)
            .map_err(|err| Error::parse_toml(&args_path, err))?;
        config.expected.push('\n');

        Ok(Example {
            source_file,
            file_name,
            example_name,
            config,
        })
    }

    pub fn get_compiled_path(&self, out_base: PathBuf) -> PathBuf {
        let mut path = out_base;
        path.push(self.file_name.clone());
        path.set_extension("");

        path
    }

    pub fn compare_output(&self, result: Vec<u8>) -> ExampleResult {
        let expected_u8 = self.config.expected.clone().into_bytes();
        let fail_msg = if result == expected_u8 {
            None
        } else {
            let found_str = String::from_utf8(result.clone()).unwrap_or(format!("{:?}", result));
            Some(format!(
                "Example {} did not give expected result: expected {:?}, got {:?}. ",
                self.example_name, self.config.expected, found_str
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

    pub fn report(results: Vec<ExampleResult>) -> Result<(), Error> {
        println!("Ran {} tests", results.len());
        let mut num_success = 0;
        let mut num_fail = 0;
        for result in results {
            println!("\t{}", result);
            if result.fail_msg.is_none() {
                num_success += 1
            } else {
                num_fail += 1
            }
        }
        println!(
            "\ntest result: {} passed; {} failed\n",
            num_success, num_fail
        );
        if num_fail == 0 {
            Ok(())
        } else {
            Err(Error::TestFailure { num_fail })
        }
    }
}

impl fmt::Display for ExampleResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fail_str = "\x1B[38;2;255;0;0mfail\x1B[0m";
        let ok_str = "\x1b[38;2;0;255;0mok\x1B[0m";
        let succ = match &self.fail_msg {
            None => ok_str.to_owned(),
            Some(err) => format!("{fail_str}:\n\t\tError: {err}\n"),
        };
        write!(f, "Test: {} {} ... {}", self.ty, self.name, succ)
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
