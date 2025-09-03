//! This module contains the command for formatting a Fun source code file.

use std::fs::File;
use std::path::PathBuf;

use driver::Driver;
use printer::{ColorChoice, Print, PrintCfg, StandardStream, WriteColor};

use crate::utils::ignore_colors::IgnoreColors;

#[derive(clap::Args)]
pub struct Args {
    #[clap(value_parser, value_name = "FILE")]
    filepath: PathBuf,
    #[clap(long)]
    width: Option<usize>,
    #[clap(long, num_args = 0)]
    inplace: bool,
    #[clap(long, default_value_t = 4)]
    indent: isize,
    #[clap(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

/// This function computes the output stream for the "fmt" subcommand. If an output filepath is
/// specified, then that filepath is used. Otherwise, the formatted output is printed on the
/// terminal. If the `--inplace` flag is specified, then the input file is overwritten.
fn compute_output_stream(cmd: &Args) -> Box<dyn WriteColor> {
    if cmd.inplace {
        return Box::new(IgnoreColors::new(
            File::create(cmd.filepath.clone()).expect("Failed to create file"),
        ));
    }
    match &cmd.output {
        Some(path) => Box::new(IgnoreColors::new(
            File::create(path).expect("Failed to create file"),
        )),
        None => Box::new(StandardStream::stdout(ColorChoice::Auto)),
    }
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let parsed = drv.parsed(&cmd.filepath);
    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };

    // Write to file or to stdout
    let mut stream: Box<dyn WriteColor> = compute_output_stream(&cmd);

    let cfg = PrintCfg {
        width: cmd.width.unwrap_or_else(terminal_width),
        allow_linebreaks: true,
        latex: false,
        omit_decl_sep: false,
        indent: cmd.indent,
    };

    parsed
        .print_colored(&cfg, &mut stream)
        .expect("Failed to print to stdout");
    println!();
    Ok(())
}

fn terminal_width() -> usize {
    termsize::get()
        .map(|size| size.cols as usize)
        .unwrap_or(printer::DEFAULT_WIDTH)
}
