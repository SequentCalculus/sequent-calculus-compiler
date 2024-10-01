use std::path::PathBuf;

use std::fs;

use fun::syntax::declarations::Module;
use fun::{parser::parse_module, typing::check::check_module};

use clap::{Parser, Subcommand};

mod check;
mod compile;
mod focus;

fn parse_and_check_from_file(filepath: PathBuf) -> miette::Result<Module> {
    let content = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let parsed_result = parse_module(&content);
    let miette_error: miette::Error = match parsed_result {
        Ok(module) => match check_module(&module) {
            Ok(()) => return Ok(module),
            Err(err) => err.into(),
        },
        Err(err) => err.into(),
    };
    let report = miette_error.with_source_code(content);
    Err(report)
}

pub fn exec() -> miette::Result<()> {
    use Command::*;
    let cli = Cli::parse();
    match cli.command {
        Check(args) => check::exec(args),
        Compile(args) => compile::exec(args),
        Focus(args) => focus::exec(args),
    }
}

#[derive(Parser)]
#[clap(version, author, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Typecheck a file
    Check(check::Args),
    /// Compile a file to Core
    Compile(compile::Args),
    /// Focus the definitions of a file
    Focus(focus::Args),
}
