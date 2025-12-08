//! This module contains the command for linearizing the definitions of a file.

use super::print_stdout;
use driver::{Driver, PrintMode};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, colored: bool, opt_passes: u64, print_opt: bool) -> miette::Result<()> {
    let mut drv = Driver::new(opt_passes);
    let linearized = drv.linearized(&cmd.filepath);
    let linearized = match linearized {
        Ok(linearized) => linearized,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_linearized(&cmd.filepath, PrintMode::Textual)?;
    if print_opt {
        drv.print_opt_stats(&cmd.filepath)?;
    }
    print_stdout(&linearized, colored);
    Ok(())
}
