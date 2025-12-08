use crate::{
    BENCHMARK_PATH, BIN_OUT, EXAMPLES_AARCH, EXAMPLES_OUT, EXAMPLES_PATH, EXAMPLES_X86,
    errors::Error,
};
use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string, rename},
    path::{Path, PathBuf},
    process::Command,
};

pub struct Example {
    pub name: String,
    pub source_path: PathBuf,
    pub config: ExampleConfig,
}

#[derive(serde::Deserialize)]
pub struct ExampleConfig {
    pub test_args: Vec<String>,
    pub heap_size: Option<u64>,
}

impl Example {
    pub fn from_dir(dir: &Path) -> Result<Example, Error> {
        let name = dir
            .file_name()
            .ok_or(Error::read_file_name(dir))?
            .to_str()
            .ok_or(Error::read_file_name(dir))?
            .to_owned();
        let mut source_path = dir.join(&name);
        source_path.set_extension("sc");

        let mut config_path = dir.join(&name);
        config_path.set_extension("args");
        let config_contents =
            read_to_string(&config_path).map_err(|err| Error::read_conf(&config_path, err))?;
        let config = basic_toml::from_str::<ExampleConfig>(&config_contents)
            .map_err(|err| Error::toml(&config_path, err))?;

        Ok(Example {
            name,
            source_path,
            config,
        })
    }

    pub fn compiled_path(&self, compiler_name: &str) -> PathBuf {
        #[cfg(target_arch = "x86_64")]
        let out_path = PathBuf::from(EXAMPLES_OUT).join(EXAMPLES_X86);
        #[cfg(target_arch = "aarch64")]
        let out_path = PathBuf::from(EXAMPLES_OUT).join(EXAMPLES_AARCH);

        out_path.join(format!("{}_{}", self.name, compiler_name))
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

pub fn compile_examples(examples: &[Example], compiler_names: &[String]) -> Result<(), Error> {
    let compiler_bins: Vec<(&String, PathBuf)> = compiler_names
        .iter()
        .map(|name| (name, PathBuf::from(BIN_OUT).join(format!("scc_{name}"))))
        .collect();

    #[cfg(target_arch = "x86_64")]
    let out_path = PathBuf::from(EXAMPLES_OUT).join(EXAMPLES_X86);
    #[cfg(target_arch = "aarch64")]
    let out_path = PathBuf::from(EXAMPLES_OUT).join(EXAMPLES_AARCH);

    for example in examples {
        for (compiler_name, bin_path) in compiler_bins.iter() {
            println!("Compiling {} with compiler {compiler_name}", example.name);
            let mut compile_cmd = Command::new(bin_path);
            compile_cmd.arg("codegen").arg(&example.source_path);

            #[cfg(target_arch = "x86_64")]
            compile_cmd.arg("x86-64");
            #[cfg(target_arch = "aarch64")]
            compile_cmd.arg("aarch64");

            if let Some(size) = example.config.heap_size {
                compile_cmd.arg("--heap-size").arg(size.to_string());
            }

            let compile_res = compile_cmd.output().map_err(|err| {
                Error::start_cmd(
                    &format!("scc_{compiler_name}"),
                    &format!("Compile example {}", example.source_path.display()),
                    err,
                )
            })?;
            if !compile_res.status.success() {
                let stdout_str = String::from_utf8(compile_res.stdout)
                    .map_err(|err| Error::parse_out("scc", err))?;
                let stderr_str = String::from_utf8(compile_res.stderr)
                    .map_err(|err| Error::parse_out("scc", err))?;
                return Err(Error::run_cmd(
                    &format!("scc_{compiler_name}"),
                    compile_res.status,
                    &stdout_str,
                    &stderr_str,
                ));
            }

            let example_from = out_path.join(&example.name);
            let example_to = example.compiled_path(&compiler_name);
            rename(&example_from, &example_to)
                .map_err(|err| Error::move_file(&example_from, &example_to, err))?;
        }
    }
    Ok(())
}
