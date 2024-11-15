use std::path::PathBuf;

use driver::Driver;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let checked = drv.checked(&cmd.filepath);
    if let Err(err) = checked {
        return Err(drv.error_to_report(err, &cmd.filepath));
    }
    Ok(())
}
