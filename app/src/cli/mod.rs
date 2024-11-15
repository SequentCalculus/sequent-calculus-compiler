use clap::{Parser, Subcommand};

mod check;
mod clean;
mod codegen;
mod compile;
mod fmt;
mod focus;
mod linearize;
mod shrink;
mod texify;

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
    /// Delete all intermediate files
    Clean(clean::Args),
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
