use std::{collections::HashMap, fmt};

mod benchmark;
mod compile_scc;
mod config;
mod errors;
mod examples;

use benchmark::benchmark_examples;
use compile_scc::compile_versions;
use config::EvalConfig;
use errors::Error;
use examples::{compile_examples, load_examples};

const CONFIG_PATH: &str = "evaluation/config.toml";
const SCC_BIN: &str = "target/release/scc";
const BIN_OUT: &str = "target_scc/versions";
const EXAMPLES_PATH: &str = "examples";
const BENCHMARK_PATH: &str = "benchmarks/suite";
const EXAMPLES_OUT: &str = "target_scc/bin/";
const EXAMPLES_X86: &str = "x86_64";
const EXAMPLES_AARCH: &str = "aarch_64";

pub struct EvalResults {
    optimization_stats: HashMap<String, u64>,
}

fn main() -> Result<(), Error> {
    let mut results = EvalResults {
        optimization_stats: HashMap::new(),
    };
    println!("Loading configuration...");
    let config = EvalConfig::load()?;
    println!("Loading examples...");
    let examples = load_examples()?;
    println!("Compiling compiler versions...");
    compile_versions(&config.version_git_hashes)?;
    println!("Compiling examples...");
    let version_names: Vec<String> = config.version_git_hashes.keys().cloned().collect();
    compile_examples(&examples, &version_names, &mut results)?;
    println!("Benchmarking examples...");
    benchmark_examples(&examples, &version_names)?;
    println!("Results: {results}");
    Ok(())
}

impl fmt::Display for EvalResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=== Evaluation Results ===")?;
        for (example, num_passes) in self.optimization_stats.iter() {
            write!(f, "{example}: {num_passes} passes")?;
        }
        Ok(())
    }
}
