use std::path::PathBuf;

use std::{fs, process};

use fun::parser::fun::ProgParser;
use fun::program::Prog;
use fun2core::program::compile_prog;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) {
    let content = fs::read_to_string(cmd.filepath).expect("Should have been able to read the file");
    let parser: ProgParser = ProgParser::new();
    let parsed: Prog<()> = match parser.parse(&content) {
        Ok(tm) => tm,
        Err(err) => {
            println!("{}", err);
            process::exit(0)
        }
    };
    let compiled = compile_prog(parsed);
    println!("{}", compiled)
}
