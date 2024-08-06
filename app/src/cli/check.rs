use std::path::PathBuf;

use std::{fs, process};

use fun::parser::fun::ProgParser;
use fun::program::Prog;
use fun::types::{infer_types, Ty};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) {
    let content = fs::read_to_string(cmd.filepath).expect("Should have been able to read the file");
    dispatch(content)
}

fn dispatch(arg: String) {
    let parser: ProgParser = ProgParser::new();
    let parsed: Prog<()> = match parser.parse(&arg) {
        Ok(tm) => tm,
        Err(err) => {
            println!("{}", err);
            process::exit(0)
        }
    };

    let m_prog_typed: fun::program::Prog<Ty> = infer_types(parsed).unwrap();

    print!("{}", m_prog_typed);
}
