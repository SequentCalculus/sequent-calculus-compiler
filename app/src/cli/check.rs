//! This module contains the command for typechecking a file.

use std::path::PathBuf;

use driver::Driver;
use printer::{ColorChoice, Print, PrintCfg, StandardStream, WriteColor};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let checked = drv.inferred(&cmd.filepath);
    if let Err(err) = checked {
        return Err(drv.error_to_report(err, &cmd.filepath));
    } else if let Ok(inferred_prog) = checked {
        // Write to file or to stdout
        let mut stream: Box<dyn WriteColor> = Box::new(StandardStream::stdout(ColorChoice::Auto));

        let cfg = PrintCfg {
            width: terminal_width(),
            allow_linebreaks: true,
            latex: false,
            omit_decl_sep: false,
            indent: 2,
        };
        
        inferred_prog
            .print_colored(&cfg, &mut stream)
            .expect("Failed to print to stdout");
    }
    Ok(())
}

fn terminal_width() -> usize {
    termsize::get()
        .map(|size| size.cols as usize)
        .unwrap_or(printer::DEFAULT_WIDTH)
}