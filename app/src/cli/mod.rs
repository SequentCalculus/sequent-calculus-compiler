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
mod linearize;
mod rewrite;
mod shrink;
mod texify;

/// This function prints a given `printable` object to stdout. The `colored` flag controls whether
/// the result of printing is colorful.
pub fn print_stdout<P: Print>(printable: &P, colored: bool) {
    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    if colored {
        printable
            .print_colored(&PrintCfg::default(), &mut stream)
            .expect("Failed to print to stdout");
    } else {
        printable
            .print_io(&PrintCfg::default(), &mut stream)
            .expect("Failed to print to stdout");
    }
}

/// This function executes the compiler. It parses the command to perform and then performs it.
pub fn exec() -> miette::Result<()> {
    use Command::*;
    let cli = Cli::parse();
    match cli.command {
        Check(args) => check::exec(args, cli.opt_passes),
        Clean(args) => clean::exec(args),
        Codegen(args) => codegen::exec(args, cli.opt_passes, cli.print_opt),
        Compile(args) => compile::exec(args, !cli.no_color, cli.opt_passes, cli.print_opt),
        Focus(args) => focus::exec(args, !cli.no_color, cli.opt_passes, cli.print_opt),
        Fmt(args) => fmt::exec(args, !cli.no_color, cli.opt_passes),
        Linearize(args) => linearize::exec(args, !cli.no_color, cli.opt_passes, cli.print_opt),
        Rewrite(args) => rewrite::exec(args, !cli.no_color, cli.opt_passes, cli.print_opt),
        Shrink(args) => shrink::exec(args, !cli.no_color, cli.opt_passes, cli.print_opt),
        Texify(args) => texify::exec(args, cli.opt_passes),
        GenerateCompletion(args) => gen_completions::exec(args),
    }
}

/// This is the scc compiler.
#[derive(Parser)]
#[clap(version, author, about, long_about = None)]
struct Cli {
    /// Command to execute
    #[clap(subcommand)]
    command: Command,
    /// Print output without color
    #[clap(short, long)]
    no_color: bool,
    /// maximum number of optimization passes to run
    #[clap(short, long, default_value_t = 10)]
    opt_passes: u64,
    /// Print stats of optimization
    #[clap(short, long)]
    print_opt: bool,
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
    /// Rewrite the definitions of a file
    Rewrite(rewrite::Args),
    /// Print program representations as LaTeX code
    Texify(texify::Args),
    /// Generate completion scripts for various shells
    GenerateCompletion(gen_completions::Args),
}
