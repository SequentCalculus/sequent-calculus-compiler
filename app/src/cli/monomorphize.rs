//! This module contains the command for monomorphizing a file.

use std::path::PathBuf;

use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    #[arg(long = "viz", num_args(0..=1))]
    viz: Option<Option<PathBuf>>,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let monomorphized = drv.monomorphized(&cmd.filepath, cmd.viz);
    if let Err(err) = monomorphized {
        return Err(drv.error_to_report(err, &cmd.filepath));
    }
    Ok(())
}
