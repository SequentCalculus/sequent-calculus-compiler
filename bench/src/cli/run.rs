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

    for bench in benchmarks {
        #[cfg(target_arch = "x86_64")]
        let _ = driver.compile_x86_64(&bench.bench_path, false);

        #[cfg(target_arch = "aarch64")]
        let _ = driver.compile_aarch64(&bench.bench_path, false);

        bench.run_hyperfine();
    }

    Ok(())
}
