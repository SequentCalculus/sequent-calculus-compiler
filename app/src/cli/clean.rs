//! This module contains the command for deleting all intermediate files.

use driver::Driver;

#[derive(clap::Args)]
pub struct Args {}

pub fn exec(_cmd: Args) -> miette::Result<()> {
    Driver::clean();
    Ok(())
}
