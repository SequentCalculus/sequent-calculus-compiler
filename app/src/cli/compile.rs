use std::path::PathBuf;

use super::parse_from_file;

use fun2core::program::compile_prog;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) {
    let parsed = parse_from_file(cmd.filepath);
    let compiled = compile_prog(parsed);
    println!("{}", compiled)
}
