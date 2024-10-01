use std::path::PathBuf;

use fun::typing::check::check_module;

use super::parse_and_check_from_file;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let parsed = parse_and_check_from_file(cmd.filepath)?;
    Ok(())
}
