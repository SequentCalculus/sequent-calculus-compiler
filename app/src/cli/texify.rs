use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use driver::latex::latex_start;
use driver::latex::LATEX_END;
use driver::Driver;
use printer::{Print, PrintCfg};

#[derive(clap::ValueEnum, Clone)]
pub enum FontSize {
    Tiny,
    Scriptsize,
    Footnotesize,
    Small,
    Normalsize,
    Large,
}

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FontSize::*;
        match self {
            Tiny => write!(f, "tiny"),
            Scriptsize => write!(f, "scriptsize"),
            Footnotesize => write!(f, "footnotesize"),
            Small => write!(f, "small"),
            Normalsize => write!(f, "normalsize"),
            Large => write!(f, "large"),
        }
    }
}

#[derive(clap::Args)]
pub struct Args {
    #[clap(value_parser, value_name = "FILE")]
    filepath: PathBuf,
    #[clap(long, default_value_t = 80)]
    width: usize,
    #[clap(long, default_value_t=FontSize::Scriptsize)]
    fontsize: FontSize,
    #[clap(long, default_value_t = 4)]
    indent: isize,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let parsed = drv.parsed(&cmd.filepath);
    let parsed = match parsed {
        Ok(parsed) => parsed,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };

    let mut fp = cmd.filepath.clone();
    fp.set_extension("tex");
    let mut stream: Box<dyn io::Write> =
        Box::new(fs::File::create(fp).expect("Failed to create file"));

    let cfg = PrintCfg {
        width: cmd.width,
        latex: true,
        omit_decl_sep: true,
        indent: cmd.indent,
    };

    let _ = drv.print_compiled(&cmd.filepath, driver::PrintMode::Latex);
    let _ = drv.print_focused(&cmd.filepath, driver::PrintMode::Latex);
    let _ = drv.print_linearized(&cmd.filepath, driver::PrintMode::Latex);
    let _ = drv.print_shrunk(&cmd.filepath, driver::PrintMode::Latex);

    stream
        .write_all(latex_start(&format!("{}", &cmd.fontsize)).as_bytes())
        .unwrap();

    parsed
        .print_latex(&cfg, &mut stream)
        .expect("Failed to print to stdout");
    println!();

    stream.write_all(LATEX_END.as_bytes()).unwrap();
    Ok(())
}
