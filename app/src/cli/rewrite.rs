//! This module contains the command for inlining the definitions of a file.

use super::print_stdout;
use driver::{Driver, PrintMode};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, colored: bool, opt_passes: u64) -> miette::Result<()> {
    let mut drv = Driver::new(opt_passes);
    let rewritten = drv.rewritten(&cmd.filepath);
    let rewritten = match rewritten {
        Ok(rewritten) => rewritten,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_rewritten(&cmd.filepath, PrintMode::Textual)?;
    print_stdout(&rewritten, colored);
    Ok(())
}
