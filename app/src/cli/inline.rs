//! This module contains the command for inlining the definitions of a file.

use super::print_res;
use driver::{Driver, PrintMode};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, colored: bool) -> miette::Result<()> {
    let mut drv = Driver::new();
    let inlined = drv.inlined(&cmd.filepath);
    let inlined = match inlined {
        Ok(inlined) => inlined,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_inlined(&cmd.filepath, PrintMode::Textual)?;
    print_res(&inlined, colored);
    Ok(())
}
