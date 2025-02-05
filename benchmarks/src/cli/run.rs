use super::benchmark::Benchmark;
use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut driver = Driver::new();
    let mut benchmarks = Benchmark::load(cmd.name);
    benchmarks.sort_by(|bench1, bench2| bench1.config.suite.cmp(&bench2.config.suite));

    let mut current_suite = "".to_owned();
    for benchmark in benchmarks {
        if benchmark.config.suite != current_suite {
            current_suite = benchmark.config.suite.clone();
            println!("Running benchmarks for suite: {}", current_suite);
        }

        #[cfg(target_arch = "x86_64")]
        let _ = driver.compile_x86_64(&benchmark.path);
        #[cfg(target_arch = "aarch64")]
        let _ = driver.compile_aarch64(&benchmark.path);

        benchmark.run_hyperfine();
    }

    Ok(())
}
