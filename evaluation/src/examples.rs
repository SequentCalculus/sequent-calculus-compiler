use crate::{BENCHMARK_PATH, EXAMPLES_PATH, errors::Error};
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

pub struct Example {
    source_path: PathBuf,
    config: ExampleConfig,
}

#[derive(serde::Deserialize)]
struct ExampleConfig {
    test_args: Vec<String>,
    heap_size: Option<u64>,
}

impl Example {
    pub fn from_dir(dir: &Path) -> Result<Example, Error> {
        let name = dir
            .file_name()
            .ok_or(Error::read_file_name(dir))?
            .to_str()
            .ok_or(Error::read_file_name(dir))?;
        let mut source_path = dir.join(name);
        source_path.set_extension("sc");

        let mut config_path = dir.join(name);
        config_path.set_extension("args");
        let config_contents =
            read_to_string(&config_path).map_err(|err| Error::read_conf(&config_path, err))?;
        let config = basic_toml::from_str::<ExampleConfig>(&config_contents)
            .map_err(|err| Error::toml(&config_path, err))?;

        Ok(Example {
            source_path,
            config,
        })
    }
}

pub fn load_examples() -> Result<Vec<Example>, Error> {
    let mut examples = vec![];

    let examples_path = PathBuf::from(EXAMPLES_PATH);
    for example_dir in
        read_dir(&examples_path).map_err(|err| Error::read_dir(&examples_path, err))?
    {
        let dir_path = example_dir
            .map_err(|err| Error::read_dir(&examples_path, err))?
            .path();
        if dir_path.is_file() {
            continue;
        }

        examples.push(Example::from_dir(&dir_path)?);
    }

    let bench_path = PathBuf::from(BENCHMARK_PATH);
    for benchmark_dir in read_dir(&bench_path).map_err(|err| Error::read_dir(&bench_path, err))? {
        let dir_path = benchmark_dir
            .map_err(|err| Error::read_dir(&bench_path, err))?
            .path();
        if dir_path.is_file() {
            continue;
        }

        examples.push(Example::from_dir(&dir_path)?);
    }

    Ok(examples)
}
