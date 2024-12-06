use clap::{Parser, Subcommand};

mod run;

pub fn exec() -> miette::Result<()> {
    use Command::*;
    let cli = Cli::parse();
    match cli.command {
        Run(args) => run::exec(args),
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
    /// Run the benchmark suite
    Run(run::Args),
}
