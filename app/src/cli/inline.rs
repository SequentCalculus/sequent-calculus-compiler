//! This module contains the command for inlining the definitions of a file.

use driver::{Driver, PrintMode};
use printer::{ColorChoice, Print, PrintCfg, StandardStream};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let inlined = drv.inlined(&cmd.filepath);
    let inlined = match inlined {
        Ok(inlined) => inlined,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_inlined(&cmd.filepath, PrintMode::Textual)?;

    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    let _ = inlined.print_colored(&PrintCfg::default(), &mut stream);
    Ok(())
}
