mod compile_scc;
mod config;
mod errors;
mod examples;
use compile_scc::compile_versions;
use config::EvalConfig;
use errors::Error;
use examples::load_examples;

const CONFIG_PATH: &str = "evaluation/config.toml";
const SCC_BIN: &str = "target/release/scc";
const BIN_OUT: &str = "target_scc/versions";
const EXAMPLES_PATH: &str = "examples";
const BENCHMARK_PATH: &str = "benchmarks/suite";

fn main() -> Result<(), Error> {
    println!("Loading configuration...");
    let config = EvalConfig::load()?;
    println!("Loading examples...");
    let examples = load_examples()?;
    println!("Compiling versions...");
    compile_versions(&config.version_git_hashes)?;
    Ok(())
}
