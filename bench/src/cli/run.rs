use super::examples::{load_examples, Example};
use driver::Driver;

#[derive(clap::Args)]
pub struct Args {}

pub fn exec(_cmd: Args) -> miette::Result<()> {
    let examples = load_examples();
    let mut driver = Driver::new();

    for example in examples {
        #[cfg(target_arch = "x86_64")]
        let _ = driver.compile_x86_64(&example.example_path, false);

        #[cfg(target_arch = "aarch64")]
        let _ = driver.compile_aarch64(&example.example_path, false);

        example.run_hyperfine();
    }

    Ok(())
}
