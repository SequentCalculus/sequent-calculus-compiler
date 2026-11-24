//! This module contains the command for performing optimizations of a file.

use super::print_stdout;
use driver::{Driver, PrintMode};
use optimizations::OptMode;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    /// maximum number of optimization passes to run
    #[clap(short, long, default_value_t = 10, group = "mode")]
    opt_passes: u64,
    #[clap(long, group = "mode")]
    inline_only: bool,
    #[clap(long, group = "mode")]
    eliminate_only: bool,
    #[clap(long, group = "mode")]
    dead_code_only: bool,
}

pub fn exec(cmd: Args, colored: bool) -> miette::Result<()> {
    let mode = if cmd.inline_only {
        OptMode::InlineOnly
    } else if cmd.eliminate_only {
        OptMode::EliminateOnly
    } else if cmd.dead_code_only {
        OptMode::DeadCodeOnly
    } else {
        OptMode::Normal(cmd.opt_passes)
    };
    let mut drv = Driver::new_with_opt_mode(mode);
    let optimized = drv.optimized(&cmd.filepath);
    let optimized = match optimized {
        Ok(optimized) => optimized,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_optimized(&cmd.filepath, PrintMode::Textual)?;
    print_stdout(&optimized, colored);
    Ok(())
}
