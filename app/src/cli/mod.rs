use std::path::PathBuf;

use std::{fs, process};

use fun::parser::fun::ProgParser;
use fun::syntax::declarations::Prog;

use clap::{Parser, Subcommand};

mod compile;
mod focus;

fn parse_from_file(filepath: PathBuf) -> Prog {
    let content = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let parser: ProgParser = ProgParser::new();
    match parser.parse(&content) {
        Ok(tm) => tm,
        Err(err) => {
            println!("{}", err);
            process::exit(0)
        }
    }
}

pub fn exec() {
    use Command::*;
    let cli = Cli::parse();
    match cli.command {
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
    /// Compile a file to Core
    Compile(compile::Args),
    /// Focus the definitions of a file
    Focus(focus::Args),
}
