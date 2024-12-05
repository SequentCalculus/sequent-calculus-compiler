use driver::Driver;
use std::path::PathBuf;
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {}

pub fn exec(_cmd: Args) -> miette::Result<()> {
    let example: PathBuf = "examples/FibonacciRecursive.sc".into();

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

    Command::new("hyperfine")
        .arg("target_grk/bin/x86_64/FibonacciRecursive 40")
        .status()
        .expect("Failed to execute hyperfine");
}

#[cfg(target_arch = "aarch64")]
fn bench_aarch64(example: &PathBuf) {
    let mut drv = Driver::new();
    let _ = drv.compile_aarch64(example, false);

    Command::new("hyperfine")
        .arg("target_grk/bin/aarch64/FibonacciRecursive 40")
        .status()
        .expect("Failed to execute hyperfine");
}
