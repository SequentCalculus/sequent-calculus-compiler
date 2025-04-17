use clap::{Parser, Subcommand};

mod benchmark;
mod config;
mod report;
mod run;

pub fn exec() -> miette::Result<()> {
    use Command::Run;
    let cli = Cli::parse();
    match cli.command {
        Run(args) => run::exec(args),
        Report(args) => report::exec(args),
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
    /// Report the benchmarking results
    Report(report::Args),
}
