use std::path::PathBuf;

use super::parse_from_file;

use core::transform::transform_prog;
use fun2core::program::compile_prog;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let parsed = parse_from_file(cmd.filepath)?;
    let compiled = compile_prog(parsed);
    let focused = transform_prog(compiled);
    println!("{}", focused);
    Ok(())
}
