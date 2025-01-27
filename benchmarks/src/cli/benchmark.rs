use super::config::Config;
use driver::paths::{Paths, BENCHMARKS_PATH, BENCHMARKS_RESULTS};
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    process::Command,
};

pub struct Benchmark {
    pub path: PathBuf,
    pub bin_path: String,
    pub result_path: PathBuf,
    pub config: Config,
}

impl Benchmark {
    pub fn new(name: &str) -> Option<Benchmark> {
        let mut path = PathBuf::from(BENCHMARKS_PATH).join(name).join(name);
        path.set_extension("sc");
        if !path.exists() {
            return None;
        }

        let bin_path = Self::bin_name(path.clone());

        let mut result_path = PathBuf::from(BENCHMARKS_RESULTS).join(name);
        result_path.set_extension("csv");

        let mut args_file = path.clone();
        args_file.set_extension("args");
        let config = Config::from_file(args_file);

        Some(Benchmark {
            path,
            bin_path: bin_path.to_str().unwrap().to_owned(),
            result_path,
            config,
        })
    }

    fn bin_name(benchmark: PathBuf) -> PathBuf {
        let mut bin_name = benchmark;
        bin_name.set_extension("");

        #[cfg(target_arch = "x86_64")]
        let bin_path = Paths::x86_64_binary_dir().join(bin_name.file_name().unwrap());
        #[cfg(target_arch = "aarch64")]
        let bin_path = Paths::aarch64_binary_dir().join(bin_name.file_name().unwrap());
        bin_path
    }

    pub fn run_hyperfine(&self) {
        create_dir_all(self.result_path.parent().unwrap()).unwrap();
        let mut command = Command::new("hyperfine");
        for arg in &self.config.args {
            command.arg(format!("{} {}", &self.bin_path, arg));
        }
        command.arg("--runs");
        command.arg(self.config.runs.to_string());
        command.arg("--export-csv");
        command.arg(self.result_path.to_str().unwrap());

        command.status().expect("Failed to execute hyperfine");
    }

    pub fn load_all() -> Vec<Benchmark> {
        let mut paths = vec![];
        for path in read_dir(BENCHMARKS_PATH).unwrap() {
            let path = path.unwrap().path();
            let name = path.file_name().unwrap().to_str().unwrap();
            if !path.is_dir() {
                continue;
            }

            let benchmark = Benchmark::new(name);
            if let Some(benchmark) = benchmark {
                paths.push(benchmark);
            }
        }
        paths
    }

    pub fn load(name: Option<String>) -> Vec<Benchmark> {
        match name {
            Some(name) => {
                let benchmark = Self::new(&name).expect("Could not find benchmark {name}");
                vec![benchmark]
            }
            None => Self::load_all(),
        }
    }
}
