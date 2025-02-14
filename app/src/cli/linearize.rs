use driver::{Driver, PrintMode};
use printer::{ColorChoice, Print, PrintCfg, StandardStream};
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let linearized = drv.linearized(&cmd.filepath);
    let linearized = match linearized {
        Ok(linearized) => linearized,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    let _ = drv.print_linearized(&cmd.filepath, PrintMode::Textual);

    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    let _ = linearized.print_colored(&PrintCfg::default(), &mut stream);
    Ok(())
}
