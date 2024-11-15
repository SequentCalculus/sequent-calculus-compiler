use std::path::PathBuf;

use std::fs;

use fun::syntax::declarations::Module;
use fun::{parser::parse_module, typing::check::check_module};

use clap::{Parser, Subcommand};

mod check;
mod codegen;
mod compile;
mod fmt;
mod focus;
mod ignore_colors;
mod linearize;
mod shrink;
mod texify;

fn parse_and_check_from_file(filepath: PathBuf) -> miette::Result<Module> {
    let content = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let parsed_result = parse_module(&content);
    let miette_error: miette::Error = match parsed_result {
        Ok(module) => match check_module(module) {
            Ok(module) => return Ok(module),
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
        Codegen(args) => codegen::exec(args),
        Compile(args) => compile::exec(args),
        Focus(args) => focus::exec(args),
        Fmt(args) => fmt::exec(args),
        Linearize(args) => linearize::exec(args),
        Shrink(args) => shrink::exec(args),
        Texify(args) => texify::exec(args),
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
    /// Generate assembly codefor a file
    Codegen(codegen::Args),
    /// Compile a file to Core
    Compile(compile::Args),
    /// Focus the definitions of a file
    Focus(focus::Args),
    /// Format a source code file
    Fmt(fmt::Args),
    /// Linearize the definitions of a file
    Linearize(linearize::Args),
    /// Shrink the definitions of a file to AxCut
    Shrink(shrink::Args),
    /// Convert source code file to latex
    Texify(texify::Args),
}
