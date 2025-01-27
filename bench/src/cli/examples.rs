use super::example_config::ExampleConfig;
use driver::paths::{Paths, BENCH_PATH, BENCH_RESULTS};
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    process::Command,
};

pub struct Example {
    pub example_path: PathBuf,
    pub bin_path: String,
    pub result_path: PathBuf,
    pub conf: ExampleConfig,
}

impl Example {
    pub fn new(name: &str) -> Option<Example> {
        let mut path = PathBuf::from(BENCH_PATH).join(name).join(name);
        path.set_extension("sc");
        if !path.exists() {
            return None;
        }

        let bin_path = Self::bin_name(path.clone());

        let mut result_path = PathBuf::from(BENCH_RESULTS).join(name);
        result_path.set_extension("csv");

        let mut args_file = path.clone();
        args_file.set_extension("args");
        let conf = ExampleConfig::from_file(args_file);

        Some(Example {
            example_path: path,
            bin_path: bin_path.to_str().unwrap().to_owned(),
            result_path,
            conf,
        })
    }

    fn bin_name(example: PathBuf) -> PathBuf {
        let mut bin_name = example;
        bin_name.set_extension("");

        #[cfg(target_arch = "x86_64")]
        let bin_path = Paths::x86_64_binary_dir().join(bin_name.file_name().unwrap());
        #[cfg(target_arch = "aarch64")]
        let bin_path = Paths::aarch64_binary_dir().join(bin_name.file_name().unwrap());
        bin_path
    }

    pub fn run_hyperfine(&self) {
        create_dir_all(self.result_path.parent().unwrap()).unwrap();
        let mut cmd = Command::new("hyperfine");
        for arg in self.conf.args.iter() {
            cmd.arg(format!("{} {}", &self.bin_path, arg));
        }
        cmd.arg("--runs");
        cmd.arg(self.conf.runs.to_string());
        cmd.arg("--export-csv");
        cmd.arg(self.result_path.to_str().unwrap());

        cmd.status().expect("Failed to execute hyperfine");
    }

    pub fn load_all() -> Vec<Example> {
        let mut paths = vec![];
        for path in read_dir(BENCH_PATH).unwrap() {
            let path = path.unwrap().path();
            let name = path.file_name().unwrap().to_str().unwrap();
            if !path.is_dir() {
                continue;
            }

            let next_example = Example::new(name);
            if let Some(ex) = next_example {
                paths.push(ex)
            }
        }
        paths
    }

    pub fn load(name: Option<String>) -> Vec<Example> {
        match name {
            Some(name) => {
                let example = Self::new(&name).expect("Could not find benchmark {name}");
                vec![example]
            }
            None => Self::load_all(),
        }
    }
}
