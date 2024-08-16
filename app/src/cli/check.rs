use std::path::PathBuf;

use super::parse_from_file;

use fun::typing::infer_types;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) {
    let parsed = parse_from_file(cmd.filepath);
    let prog_typed = infer_types(parsed).unwrap();
    print!("{}", prog_typed);
}
