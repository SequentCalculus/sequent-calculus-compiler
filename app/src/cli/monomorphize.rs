//! This module contains the command for monomorphizing a file.

use std::path::PathBuf;

use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let monomorphized = drv.monomorphized(&cmd.filepath);
    if let Err(err) = monomorphized {
        return Err(drv.error_to_report(err, &cmd.filepath));
    }
    Ok(())
}
