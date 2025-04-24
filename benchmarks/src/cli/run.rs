use super::benchmark::Benchmark;
use driver::{paths::BENCHMARKS_RESULTS, Driver};
use std::path::PathBuf;

const DEFAULT_HEAP_SIZE: usize = 512;

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,
    /// Optional heap size in MB, default is 512
    #[arg(long)]
    heap_size: Option<usize>,
    ///Optional skip benchmarks with existing results
    #[arg(long, short)]
    skip_existing: bool,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut driver = Driver::new();
    let mut benchmarks = Benchmark::load(cmd.name);
    benchmarks.sort_by(|bench1, bench2| bench1.config.suite.cmp(&bench2.config.suite));

    let mut current_suite = "".to_owned();
    for benchmark in benchmarks {
        let mut report_file = PathBuf::from(BENCHMARKS_RESULTS).join(
            benchmark
                .path
                .file_stem()
                .expect("Could not get benchmark path"),
        );
        report_file.set_extension("csv");

        if cmd.skip_existing && report_file.exists() {
            println!("Skipping benchmark {:?}", benchmark.path);
            continue;
        }
        if benchmark.config.suite != current_suite {
            current_suite = benchmark.config.suite.clone();
            println!("Running benchmarks for suite: {}", current_suite);
        }

        let heap_size = if cmd.heap_size.is_some() {
            cmd.heap_size
        } else if benchmark.config.heap_size.is_some() {
            benchmark.config.heap_size
        } else {
            Some(DEFAULT_HEAP_SIZE)
        };
        #[cfg(target_arch = "x86_64")]
        let _ = driver.compile_x86_64(&benchmark.path, heap_size);
        #[cfg(target_arch = "aarch64")]
        let _ = driver.compile_aarch64(&benchmark.path, heap_size);

        benchmark.run_hyperfine();
    }

    Ok(())
}
