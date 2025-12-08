use std::{path::PathBuf, process::Command};

mod compile_scc;
mod config;
mod errors;
mod examples;

use compile_scc::compile_versions;
use config::EvalConfig;
use errors::Error;
use examples::{Example, load_examples};

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
    println!("Compiling compiler versions...");
    compile_versions(&config.version_git_hashes)?;
    println!("Compiling examples...");
    compile_examples(examples, &config.version_git_hashes)?;
    Ok(())
}

fn compile_examples(examples: Vec<Example>, hashes: &[String]) -> Result<(), Error> {
    let compiler_bins: Vec<PathBuf> = hashes
        .iter()
        .enumerate()
        .map(|(ind, _)| PathBuf::from(BIN_OUT).join(format!("scc_{ind}")))
        .collect();

    for example in examples {
        for compiler_bin in compiler_bins.iter() {
            let mut compile_cmd = Command::new(compiler_bin);
            compile_cmd.arg("codegen").arg(&example.source_path);

            #[cfg(target_arch = "x86_64")]
            compile_cmd.arg("x86-64");
            #[cfg(target_arch = "aarch64")]
            compile_cmd.arg("aarch64");

            if let Some(size) = example.config.heap_size {
                compile_cmd.arg("--heap_size").arg(size.to_string());
            }

            let compile_res = compile_cmd.status().map_err(|err| {
                Error::cmd(
                    "scc",
                    &format!("Compile exmaple {}", example.source_path.display()),
                    err,
                )
            })?;
            if !compile_res.success() {
                return Err(Error::cmd(
                    "scc",
                    &format!("Compile exmaple {}", example.source_path.display()),
                    format!("Exited with code {compile_res}"),
                ));
            }
        }
    }
    Ok(())
}
