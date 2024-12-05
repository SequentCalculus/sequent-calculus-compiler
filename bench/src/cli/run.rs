use driver::paths::Paths;
use driver::{paths, Driver};
use std::path::PathBuf;
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {}

const EXAMPLE: &str = "FibonacciRecursive.sc";

pub fn exec(_cmd: Args) -> miette::Result<()> {
    let example: PathBuf = PathBuf::from(paths::EXAMPLES_PATH).join(EXAMPLE);

    #[cfg(target_arch = "x86_64")]
    bench_x86_64(&example);

    #[cfg(target_arch = "aarch64")]
    bench_aarch64(&example);

    Ok(())
}

#[cfg(target_arch = "x86_64")]
fn bench_x86_64(example: &PathBuf) {
    let mut drv = Driver::new();
    let _ = drv.compile_x86_64(example, false);

    let mut bin_name = PathBuf::from(EXAMPLE);
    bin_name.set_extension("");
    let bin_path = Paths::x86_64_binary_dir().join(bin_name);

    Command::new("hyperfine")
        .arg(format!("{} 40", bin_path.to_str().unwrap()))
        .status()
        .expect("Failed to execute hyperfine");
}

#[cfg(target_arch = "aarch64")]
fn bench_aarch64(example: &PathBuf) {
    let mut drv = Driver::new();
    let _ = drv.compile_aarch64(example, false);

    let mut bin_name = PathBuf::from(EXAMPLE);
    bin_name.set_extension("");
    let bin_path = Paths::aarch64_binary_dir().join(bin_name);

    Command::new("hyperfine")
        .arg(format!("{} 40", bin_path.to_str().unwrap()))
        .status()
        .expect("Failed to execute hyperfine");
}
