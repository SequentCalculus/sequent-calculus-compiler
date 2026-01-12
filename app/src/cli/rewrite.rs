//! This module contains the command for inlining the definitions of a file.

use super::print_stdout;
use driver::{Driver, PrintMode};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    /// maximum number of optimization passes to run
    #[clap(short, long, default_value_t = 10)]
    opt_passes: u64,
    /// Print stats of optimization
    #[clap(short, long)]
    print_opt: bool,
}

pub fn exec(cmd: Args, colored: bool) -> miette::Result<()> {
    let mut drv = Driver::new_with_num_passes(cmd.opt_passes);
    let rewritten = drv.rewritten(&cmd.filepath);
    let rewritten = match rewritten {
        Ok(rewritten) => rewritten,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_rewritten(&cmd.filepath, PrintMode::Textual)?;
    if cmd.print_opt {
        drv.print_opt_stats(&cmd.filepath)?;
    }
    print_stdout(&rewritten, colored);
    Ok(())
}
