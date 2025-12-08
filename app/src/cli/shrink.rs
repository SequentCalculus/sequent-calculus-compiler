//! This module contains the command for shrinking the definitions of a file to AxCut.

use super::print_stdout;
use driver::{Driver, PrintMode};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, colored: bool, opt_passes: u64, print_opt: bool) -> miette::Result<()> {
    let mut drv = Driver::new(opt_passes);
    let shrunk = drv.shrunk(&cmd.filepath);
    let shrunk = match shrunk {
        Ok(shrunk) => shrunk,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_shrunk(&cmd.filepath, PrintMode::Textual)?;
    if print_opt {
        drv.print_opt_stats(&cmd.filepath)?;
    }
    print_stdout(&shrunk, colored);

    Ok(())
}
