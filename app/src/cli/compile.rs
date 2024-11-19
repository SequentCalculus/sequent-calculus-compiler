use std::path::PathBuf;

use driver::{Driver, PrintMode};
use printer::{ColorChoice, Print, StandardStream};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let compiled = drv.compiled(&cmd.filepath);
    let compiled = match compiled {
        Ok(compiled) => compiled,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    let _ = drv.print_compiled(&cmd.filepath, PrintMode::Textual);

    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    let _ = compiled.print_colored(&Default::default(), &mut stream);
    Ok(())
}
