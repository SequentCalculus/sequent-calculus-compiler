use std::fmt;

use driver::Driver;
use printer::PrintCfg;
use std::path::PathBuf;

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
    let cfg = PrintCfg {
        width: cmd.width,
        latex: true,
        omit_decl_sep: true,
        indent: cmd.indent,
    };

    let mut drv = Driver::new();
    let _ = drv.print_compiled(&cmd.filepath, driver::PrintMode::Latex);
    let _ = drv.print_focused(&cmd.filepath, driver::PrintMode::Latex);
    let _ = drv.print_linearized(&cmd.filepath, driver::PrintMode::Latex);
    let _ = drv.print_shrunk(&cmd.filepath, driver::PrintMode::Latex);
    let _ = drv.print_parsed_tex(&cmd.filepath, &cfg, format!("{}", cmd.fontsize));
    let _ = drv.print_latex_all(&cmd.filepath);

    Ok(())
}
