//! This module contains the command for typechecking a file.

use std::path::PathBuf;

use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, opt_passes: u64) -> miette::Result<()> {
    let mut drv = Driver::new(opt_passes);
    let checked = drv.checked(&cmd.filepath);
    if let Err(err) = checked {
        return Err(drv.error_to_report(err, &cmd.filepath));
    }
    Ok(())
}
