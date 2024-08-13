use clap::{Parser, Subcommand};

mod check;
mod compile;

pub fn exec() {
    use Command::*;
    let cli = Cli::parse();
    match cli.command {
        Check(args) => check::exec(args),
        Compile(args) => compile::exec(args),
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
}
