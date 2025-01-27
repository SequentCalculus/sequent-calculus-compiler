use super::config::Config;
<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
use driver::paths::{Paths, BENCHMARKS_PATH, BENCHMARKS_RESULTS};
=======
use driver::paths::{Paths, BENCH_PATH, BENCH_RESULTS};
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
use driver::paths::{Paths, BENCHMARKS_PATH, BENCHMARKS_RESULTS};
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    process::Command,
};

pub struct Benchmark {
<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
    pub path: PathBuf,
    pub bin_path: String,
    pub result_path: PathBuf,
    pub config: Config,
=======
    pub bench_path: PathBuf,
    pub bin_path: String,
    pub result_path: PathBuf,
    pub conf: Config,
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
    pub benchmark_path: PathBuf,
    pub bin_path: String,
    pub result_path: PathBuf,
    pub config: Config,
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
}

impl Benchmark {
    pub fn new(name: &str) -> Option<Benchmark> {
<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
        let mut path = PathBuf::from(BENCHMARKS_PATH).join(name).join(name);
=======
        let mut path = PathBuf::from(BENCH_PATH).join(name).join(name);
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
        let mut path = PathBuf::from(BENCHMARKS_PATH).join(name).join(name);
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
        path.set_extension("sc");
        if !path.exists() {
            return None;
        }

        let bin_path = Self::bin_name(path.clone());

<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
        let mut result_path = PathBuf::from(BENCHMARKS_RESULTS).join(name);
=======
        let mut result_path = PathBuf::from(BENCH_RESULTS).join(name);
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
        let mut result_path = PathBuf::from(BENCHMARKS_RESULTS).join(name);
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
        result_path.set_extension("csv");

        let mut args_file = path.clone();
        args_file.set_extension("args");
<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
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
=======
        let conf = Config::from_file(args_file);
========
        let config = Config::from_file(args_file);
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs

        Some(Benchmark {
            benchmark_path: path,
            bin_path: bin_path.to_str().unwrap().to_owned(),
            result_path,
            config,
        })
    }

<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
    fn bin_name(example: PathBuf) -> PathBuf {
        let mut bin_name = example;
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
    fn bin_name(benchmark: PathBuf) -> PathBuf {
        let mut bin_name = benchmark;
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
        bin_name.set_extension("");

        #[cfg(target_arch = "x86_64")]
        let bin_path = Paths::x86_64_binary_dir().join(bin_name.file_name().unwrap());
        #[cfg(target_arch = "aarch64")]
        let bin_path = Paths::aarch64_binary_dir().join(bin_name.file_name().unwrap());
        bin_path
    }

    pub fn run_hyperfine(&self) {
        create_dir_all(self.result_path.parent().unwrap()).unwrap();
<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
        let mut command = Command::new("hyperfine");
        for arg in &self.config.args {
            command.arg(format!("{} {}", &self.bin_path, arg));
        }
        command.arg("--runs");
        command.arg(self.config.runs.to_string());
        command.arg("--export-csv");
        command.arg(self.result_path.to_str().unwrap());

        command.status().expect("Failed to execute hyperfine");
=======
        let mut cmd = Command::new("hyperfine");
        for arg in self.conf.args.iter() {
            cmd.arg(format!("{} {}", &self.bin_path, arg));
========
        let mut command = Command::new("hyperfine");
        for arg in self.config.args.iter() {
            command.arg(format!("{} {}", &self.bin_path, arg));
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
        }
        command.arg("--runs");
        command.arg(self.config.runs.to_string());
        command.arg("--export-csv");
        command.arg(self.result_path.to_str().unwrap());

<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
        cmd.status().expect("Failed to execute hyperfine");
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
        command.status().expect("Failed to execute hyperfine");
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
    }

    pub fn load_all() -> Vec<Benchmark> {
        let mut paths = vec![];
<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
        for path in read_dir(BENCHMARKS_PATH).unwrap() {
=======
        for path in read_dir(BENCH_PATH).unwrap() {
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
        for path in read_dir(BENCHMARKS_PATH).unwrap() {
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
            let path = path.unwrap().path();
            let name = path.file_name().unwrap().to_str().unwrap();
            if !path.is_dir() {
                continue;
            }

<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
            let benchmark = Benchmark::new(name);
            if let Some(benchmark) = benchmark {
                paths.push(benchmark);
=======
            let next_example = Benchmark::new(name);
            if let Some(ex) = next_example {
                paths.push(ex)
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
            let benchmark = Benchmark::new(name);
            if let Some(benchmark) = benchmark {
                paths.push(benchmark)
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
            }
        }
        paths
    }

    pub fn load(name: Option<String>) -> Vec<Benchmark> {
        match name {
            Some(name) => {
<<<<<<<< HEAD:benchmarks/suite/src/cli/benchmark.rs
<<<<<<< HEAD:bench/suite/src/cli/benchmark.rs
                let benchmark = Self::new(&name).expect("Could not find benchmark {name}");
                vec![benchmark]
=======
                let example = Self::new(&name).expect("Could not find benchmark {name}");
                vec![example]
>>>>>>> 54ebc2a (renamed example and exampleconfig):bench/src/cli/benchmark.rs
========
                let benchmark = Self::new(&name).expect("Could not find benchmark {name}");
                vec![benchmark]
>>>>>>>> c04bb23 (Change naming a bit):benchmarks/src/cli/benchmark.rs
            }
            None => Self::load_all(),
        }
    }
}
