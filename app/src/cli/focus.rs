//! This module contains the command for focusing the definitions of a file.

use std::path::PathBuf;

use super::print_res;
use driver::{Driver, PrintMode};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args, colored: bool) -> miette::Result<()> {
    let mut drv = Driver::new();
    let focused = drv.focused(&cmd.filepath);
    let focused = match focused {
        Ok(focused) => focused,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_focused(&cmd.filepath, PrintMode::Textual)?;
    print_res(&focused, colored);
    Ok(())
}
