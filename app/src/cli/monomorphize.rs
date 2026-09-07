//! This module contains the command for monomorphizing a file.

use std::path::PathBuf;

use driver::{Driver, PrintMode, VizOutput};

use crate::cli::print_stdout;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    #[arg(long = "viz", short = 'v', num_args(0..=1), require_equals = true)]
    viz: Option<Option<PathBuf>>,
    #[arg(long = "debug", short = 'd')]
    debug: bool,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let viz = VizOutput::from_cli_flag(cmd.viz);
    let monomorphized = drv.monomorphized(&cmd.filepath, viz, cmd.debug);
    let monomorphized = match monomorphized {
        Ok(mono_prog) => mono_prog,
        Err(err) => {
            return Err(drv.error_to_report(err, &cmd.filepath));
        }
    };
    drv.print_compiled(&cmd.filepath, PrintMode::Textual)?;
    print_stdout(&monomorphized, true);
    Ok(())
}
