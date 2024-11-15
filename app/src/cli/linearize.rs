use driver::Driver;
use printer::{ColorChoice, Print, StandardStream};
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
    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    let _ = linearized.print_colored(&Default::default(), &mut stream);
    Ok(())
}
