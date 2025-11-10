//! This module contains the command for compiling a file to Core.

use super::print_res;
use std::path::PathBuf;

use driver::{Driver, PrintMode};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, colored: bool) -> miette::Result<()> {
    let mut drv = Driver::new();
    let compiled = drv.compiled(&cmd.filepath);
    let compiled = match compiled {
        Ok(compiled) => compiled,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_compiled(&cmd.filepath, PrintMode::Textual)?;
    print_res(&compiled, colored);
    Ok(())
}
