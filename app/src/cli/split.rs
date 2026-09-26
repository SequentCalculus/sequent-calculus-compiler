//! This module contains the command for type-splitting a file.

use std::path::PathBuf;

use super::print_stdout;
use driver::{Driver, PrintMode};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, colored: bool) -> miette::Result<()> {
    let mut drv = Driver::new();
    let split = drv.split(&cmd.filepath);
    let split = match split {
        Ok(split) => split,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_split(&cmd.filepath, PrintMode::Textual)?;
    print_stdout(&split, colored);
    Ok(())
}
