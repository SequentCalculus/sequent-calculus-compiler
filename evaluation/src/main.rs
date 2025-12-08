use std::collections::HashMap;

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
#[allow(unused)]
const EXAMPLES_X86: &str = "x86_64";
#[allow(unused)]
const EXAMPLES_AARCH: &str = "aarch_64";

#[derive(Debug)]
pub struct EvalResult {
    example: String,
    num_passes: u64,
    lifted_create: u64,
    lifted_switch: u64,
    benchmark_times: HashMap<String, f64>,
}

fn main() -> Result<(), Error> {
    println!("Loading configuration...");
    let config = EvalConfig::load()?;
    println!("Loading examples...");
    let examples = load_examples()?;
    let mut results = Vec::with_capacity(examples.len());
    println!("Compiling compiler versions...");
    compile_versions(&config.version_git_hashes)?;
    println!("Compiling examples...");
    let version_names: Vec<String> = config.version_git_hashes.keys().cloned().collect();
    compile_examples(&examples, &version_names, &mut results)?;
    println!("Benchmarking examples...");
    benchmark_examples(&examples, &version_names, &mut results)?;
    println!("Results: {results:?}");
    Ok(())
}
