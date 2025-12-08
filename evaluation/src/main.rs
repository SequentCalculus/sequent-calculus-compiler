mod compile_scc;
mod config;
mod errors;
use compile_scc::compile_versions;
use config::EvalConfig;
use errors::Error;

const CONFIG_PATH: &str = "evaluation/config.toml";
const SCC_BIN: &str = "target/release/scc";
const BIN_OUT: &str = "target_scc/versions";

fn main() -> Result<(), Error> {
    println!("Loading configuration");
    let config = EvalConfig::load()?;
    println!("Compiling versions");
    compile_versions(&config.version_git_hashes)?;
    Ok(())
}
