use clap::{Parser, Subcommand};

mod check;
mod compile;
mod focus;

pub fn exec() {
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
