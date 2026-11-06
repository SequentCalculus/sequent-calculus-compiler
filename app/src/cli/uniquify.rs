use super::print_stdout;
use driver::{Driver, PrintMode};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    /// Print how long each compilation stage took to stderr, one stage per line: its name
    /// and its duration in microseconds.
    #[arg(long)]
    timings: bool,
}

pub fn exec(cmd: Args, colored: bool) -> miette::Result<()> {
    let mut drv = Driver::new();
    let uniquified = drv.uniquified(&cmd.filepath);
    let uniquified = match uniquified {
        Ok(unique) => unique,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_uniquified(&cmd.filepath, PrintMode::Textual)?;
    print_stdout(&uniquified, colored);
    if cmd.timings {
        eprint!("{}", drv.timings_report());
    }
    Ok(())
}
