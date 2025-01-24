use driver::paths::{Paths, BENCH_PATH};
use std::{fs::read_dir, path::PathBuf, process::Command};

pub struct Example {
    pub example_path: PathBuf,
    pub bin_path: String,
}

impl Example {
    pub fn run_hyperfine(&self) {
        Command::new("hyperfine")
            .arg(format!("{} 40", &self.bin_path))
            .status()
            .expect("Failed to execute hyperfine");
    }
}

pub fn load_examples() -> Vec<Example> {
    let mut paths = vec![];
    for path in read_dir(BENCH_PATH).unwrap() {
        let path = path.unwrap().path();
        if path.extension().unwrap() != "sc" {
            continue;
        }
        let mut bin_name = path.clone();
        bin_name.set_extension("");

        #[cfg(target_arch = "x86_64")]
        let bin_path = Paths::x86_64_binary_dir().join(bin_name.file_name().unwrap());
        #[cfg(target_arch = "aarch64")]
        let bin_path = Paths::aarch64_binary_dir().join(bin_name.file_name().unwrap());

        let next_example = Example {
            example_path: path,
            bin_path: bin_path.to_str().unwrap().to_owned(),
        };

        paths.push(next_example)
    }
    paths
}
