//! This module contains the command for typechecking a file.

use std::path::PathBuf;

use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    /// Print how long each compilation stage took to stderr, one stage per line: its name
    /// and its duration in microseconds.
    #[arg(long)]
    timings: bool,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let checked = drv.checked(&cmd.filepath);
    if let Err(err) = checked {
        return Err(drv.error_to_report(err, &cmd.filepath));
    }
    if cmd.timings {
        eprint!("{}", drv.timings_report());
    }
    Ok(())
}
