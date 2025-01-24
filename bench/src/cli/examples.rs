use driver::paths::{Paths, BENCH_PATH};
use std::{fs::read_dir, path::PathBuf, process::Command};

pub struct Example {
    pub example_path: PathBuf,
    pub bin_path: String,
}

impl Example {
    pub fn new(name: &str) -> Option<Example> {
        let mut path = PathBuf::from(BENCH_PATH).join(name);
        path.set_extension("sc");
        if !path.exists() {
            return None;
        }

        let bin_path = Self::bin_name(path.clone());
        Some(Example {
            example_path: path,
            bin_path: bin_path.to_str().unwrap().to_owned(),
        })
    }

    pub fn bin_name(example: PathBuf) -> PathBuf {
        let mut bin_name = example;
        bin_name.set_extension("");

        #[cfg(target_arch = "x86_64")]
        let bin_path = Paths::x86_64_binary_dir().join(bin_name.file_name().unwrap());
        #[cfg(target_arch = "aarch64")]
        let bin_path = Paths::aarch64_binary_dir().join(bin_name.file_name().unwrap());
        bin_path
    }

    pub fn run_hyperfine(&self) {
        Command::new("hyperfine")
            .arg(format!("{} 40", &self.bin_path))
            .status()
            .expect("Failed to execute hyperfine");
    }

    pub fn load_examples() -> Vec<Example> {
        let mut paths = vec![];
        for path in read_dir(BENCH_PATH).unwrap() {
            let path = path.unwrap().path();

            let next_example = Example::new(path.file_name().unwrap().to_str().unwrap());
            if let Some(ex) = next_example {
                paths.push(ex)
            }
        }
        paths
    }
}
