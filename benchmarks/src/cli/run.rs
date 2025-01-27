use super::benchmark::Benchmark;
use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut driver = Driver::new();
    let benchmarks = Benchmark::load(cmd.name);

    for benchmark in benchmarks {
        #[cfg(target_arch = "x86_64")]
        let _ = driver.compile_x86_64(&benchmark.benchmark_path, false);

        #[cfg(target_arch = "aarch64")]
        let _ = driver.compile_aarch64(&benchmark.benchmark_path, false);

        benchmark.run_hyperfine();
    }

    Ok(())
}
