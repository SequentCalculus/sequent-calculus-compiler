use core::naming_transformation::{NamingTransformation, TransformState};
use std::collections::HashSet;
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
    let mut trans_state = TransformState {
        used_vars: HashSet::default(),
        used_covars: HashSet::default(),
    };
    let focused = compiled.transform(&mut trans_state);
    println!("{}", focused)
}
