//! This module defines the command-line interface of the `scc` compiler.

use clap::{Parser, Subcommand};

mod check;
mod clean;
mod codegen;
mod compile;
mod fmt;
mod focus;
mod gen_completions;
mod linearize;
mod shrink;
mod texify;

/// This functions executes the compiler. It parsed the command to perform and then performs it.
pub fn exec() -> miette::Result<()> {
    use Command::*;
    let cli = Cli::parse();
    match cli.command {
        Check(args) => check::exec(args),
        Clean(args) => clean::exec(args),
        Codegen(args) => codegen::exec(args),
        Compile(args) => compile::exec(args),
        Focus(args) => focus::exec(args),
        Fmt(args) => fmt::exec(args),
        Linearize(args) => linearize::exec(args),
        Shrink(args) => shrink::exec(args),
        Texify(args) => texify::exec(args),
        GenerateCompletion(args) => gen_completions::exec(args),
    }
}

/// This struct is used by [`clap`] to generate the command-line interface.
#[derive(Parser)]
#[clap(version, author, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

/// This enum encodes the commands the compiler can execute.
#[derive(Subcommand)]
enum Command {
    /// Typecheck a file
    Check(check::Args),
    /// Delete all intermediate files
    Clean(clean::Args),
    /// Generate assembly code for a file
    Codegen(codegen::Args),
    /// Compile a file to Core
    Compile(compile::Args),
    /// Format a Fun source code file
    Fmt(fmt::Args),
    /// Focus the definitions of a file
    Focus(focus::Args),
    /// Linearize the definitions of a file
    Linearize(linearize::Args),
    /// Shrink the definitions of a file to AxCut
    Shrink(shrink::Args),
    /// Print program representations as LaTeX code
    Texify(texify::Args),
    /// Generate completion scripts for various shells
    GenerateCompletion(gen_completions::Args),
}
