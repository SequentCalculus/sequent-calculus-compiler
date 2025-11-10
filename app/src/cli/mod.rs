//! This module defines the command-line interface of the `scc` compiler.

use clap::{Parser, Subcommand};
use printer::{ColorChoice, Print, PrintCfg, StandardStream};

mod check;
mod clean;
mod codegen;
mod compile;
mod fmt;
mod focus;
mod gen_completions;
mod inline;
mod linearize;
mod shrink;
mod texify;

pub fn print_res<T>(t: &T, color: bool)
where
    T: Print,
{
    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    if color {
        let _ = t.print_colored(&PrintCfg::default(), &mut stream);
    } else {
        println!("{}", t.print_to_string(Some(&PrintCfg::default())));
    }
}

/// This functions executes the compiler. It parsed the command to perform and then performs it.
pub fn exec() -> miette::Result<()> {
    use Command::*;
    let cli = Cli::parse();
    match cli.command {
        Check(args) => check::exec(args),
        Clean(args) => clean::exec(args),
        Codegen(args) => codegen::exec(args),
        Compile(args) => compile::exec(args, !cli.no_color),
        Focus(args) => focus::exec(args, !cli.no_color),
        Fmt(args) => fmt::exec(args),
        Linearize(args) => linearize::exec(args, !cli.no_color),
        Shrink(args) => shrink::exec(args, !cli.no_color),
        Texify(args) => texify::exec(args),
        GenerateCompletion(args) => gen_completions::exec(args),
        Inline(args) => inline::exec(args, !cli.no_color),
    }
}

/// This is the scc compiler.
#[derive(Parser)]
#[clap(version, author, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
    #[clap(short, long)]
    no_color: bool,
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
    /// Inline definitions of a file
    Inline(inline::Args),
}
