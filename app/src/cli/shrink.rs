use driver::{Driver, PrintMode};
use std::path::PathBuf;

use printer::{ColorChoice, Print, PrintCfg, StandardStream};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let shrunk = drv.shrunk(&cmd.filepath);
    let shrunk = match shrunk {
        Ok(shrunk) => shrunk,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    drv.print_shrunk(&cmd.filepath, PrintMode::Textual)?;

    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    let _ = shrunk.print_colored(&PrintCfg::default(), &mut stream);
    Ok(())
}
