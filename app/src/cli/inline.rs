use driver::{Driver, PrintMode};
use std::path::PathBuf;
use super::print_stdout;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(args: Args, colored: bool) -> miette::Result<()> {
    let mut drv = Driver::new();
    let inlined = drv.inlined(&args.filepath);
    let inlined = match inlined {
        Ok(inlined) => inlined,
        Err(err) => return Err(drv.error_to_report(err, &args.filepath)),
    };
    drv.print_inlined(&args.filepath, PrintMode::Textual)?;
    print_stdout(&inlined, colored);
    Ok(())
}