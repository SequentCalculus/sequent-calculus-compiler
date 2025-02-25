use super::{errors::Error, examples::Example};
//use benchmarks::config::Config;
use driver::paths::BENCHMARKS_PATH;
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
        let file_entry = entry.map_err(|err| Error::read_dir(&examples_path, err))?;
        let file_path = file_entry.path();
        let extension = file_path
            .extension()
            .ok_or(Error::path_access(&file_path, "Extension"))?;

        if extension != "sc" {
            continue;
        }

        let file_stem = file_path
            .file_stem()
            .ok_or(Error::path_access(&file_path, "File Stem"))?;
        let example_name = file_stem
            .to_str()
            .ok_or(Error::path_access(&file_path, "File Stem as String"))?;

        let file_name = file_path
            .file_name()
            .ok_or(Error::path_access(&file_path, "File Name"))?;
        let file_name_str = file_name
            .to_str()
            .ok_or(Error::path_access(&file_path, "File Name as String"))?;

        let mut args_path = file_path.clone();
        args_path.set_extension("args");
        let args = fs::read_to_string(args_path)
            .expect("Should have been able to read the file")
            .split(',')
            .filter(|arg| *arg != "")
            .map(ToString::to_string)
            .collect();

        let mut expected_path = file_path.clone();
        expected_path.set_extension("expected");

        let mut expected_file = File::open(&expected_path)
            .map_err(|err| Error::file_access(&expected_path, "open", err))?;
        let mut expected_result = Vec::new();
        expected_file
            .read_to_end(&mut expected_result)
            .map_err(|err| Error::file_access(&expected_path, "read", err))?;

        paths.push(Example {
            source_file: file_path.clone(),
            file_name: file_name_str.to_owned(),
            example_name: example_name.to_owned(),
            args,
            expected_result,
        });
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
        if !bench_path.is_dir() {
            continue;
        }
        let bench_name = bench_path
            .file_stem()
            .ok_or(Error::path_access(&bench_path, "File Stem"))?
            .to_str()
            .ok_or(Error::path_access(&bench_path, "File Stem as String"))?;
        let mut bench_source = bench_path.join(bench_name);
        bench_source.set_extension("sc");
        let file_name = bench_source
            .file_name()
            .ok_or(Error::path_access(&bench_path, "File Name"))?
            .to_str()
            .ok_or(Error::path_access(&bench_path, "File Name as String"))?
            .to_owned();
        let mut bench_args = bench_path.join(bench_name);
        bench_args.set_extension("args");
        let config_contents = read_to_string(bench_args.clone())
            .map_err(|err| Error::file_access(&bench_args, "Read", err))?;

        #[derive(serde::Deserialize)]
        struct Config {
            test: Vec<String>,
            expected: String,
        }
        let conf = basic_toml::from_str::<Config>(&config_contents)
            .map_err(|err| Error::parse_toml(&bench_args, err))?;
        let test_args: Vec<String> = conf
            .test
            .into_iter()
            .flat_map(|arg| {
                arg.split(" ")
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .collect();

        let mut expected = conf.expected.into_bytes();
        expected.push('\n' as u8);

        examples.push(Example {
            source_file: bench_source,
            example_name: bench_name.to_owned(),
            file_name,
            args: test_args,
            expected_result: expected,
        });
    }
    Ok(examples)
}
