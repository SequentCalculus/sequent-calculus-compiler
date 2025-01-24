use driver::paths::{Paths, BENCH_PATH, BENCH_RESULTS};
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
    process::Command,
};

pub struct Example {
    pub example_path: PathBuf,
    pub bin_path: String,
    pub result_path: PathBuf,
    pub args: Vec<String>,
}

impl Example {
    pub fn new(name: &str) -> Option<Example> {
        let mut path = PathBuf::from(BENCH_PATH).join(name);
        path.set_extension("sc");
        if !path.exists() {
            return None;
        }

        let bin_path = Self::bin_name(path.clone());
        let mut result_path = PathBuf::from(BENCH_RESULTS).join(name);
        result_path.set_extension("csv");

        let args = Self::load_args(path.clone());

        Some(Example {
            example_path: path,
            bin_path: bin_path.to_str().unwrap().to_owned(),
            result_path,
            args,
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

    fn load_args(example: PathBuf) -> Vec<String> {
        let mut args_file = example;
        args_file.set_extension("args");
        if !args_file.exists() {
            return vec!["".to_owned()];
        }
        let contents = read_to_string(args_file).unwrap();
        let args = contents
            .lines()
            .filter_map(|s| (!s.is_empty()).then_some(s.to_owned()));
        args.collect()
    }

    pub fn run_hyperfine(&self) {
        let mut cmd = Command::new("hyperfine");
        for arg in self.args.iter() {
            cmd.arg(format!("{} {}", &self.bin_path, arg));
        }
        cmd.arg("--export-csv");
        cmd.arg(self.result_path.to_str().unwrap());

        cmd.status().expect("Failed to execute hyperfine");
    }

    pub fn load_examples() -> Vec<Example> {
        let mut paths = vec![];
        for path in read_dir(BENCH_PATH).unwrap() {
            let path = path.unwrap().path();
            if path.is_dir() || path.extension().unwrap() != "sc" {
                continue;
            }

            let next_example = Example::new(path.file_name().unwrap().to_str().unwrap());
            if let Some(ex) = next_example {
                paths.push(ex)
            }
        }
        paths
    }
}
