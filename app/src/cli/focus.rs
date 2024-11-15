use std::path::PathBuf;

use driver::Driver;
use printer::{ColorChoice, Print, StandardStream};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let focused = drv.focused(&cmd.filepath);
    let focused = match focused {
        Ok(focused) => focused,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    let _ = drv.print_focused(&cmd.filepath);

    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    let _ = focused.print_colored(&Default::default(), &mut stream);
    Ok(())
}
