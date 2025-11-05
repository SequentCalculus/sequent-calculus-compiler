//! This module contains the command for printing program representations as LaTeX code.

use std::fmt;

use driver::{Driver, latex::Arch};
use printer::PrintCfg;
use std::path::PathBuf;

use crate::utils::cli_options::Backend;

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
    /// The example to convert to latex
    #[clap(value_parser, value_name = "FILE")]
    filepath: PathBuf,
    /// Which backend to use
    backend: Backend,
    /// Width that the prettyprinter uses to layout code
    #[clap(long, default_value_t = 80)]
    width: usize,
    /// Which fontsize to use for generated snippets
    #[clap(long, default_value_t=FontSize::Scriptsize)]
    fontsize: FontSize,
    /// How many spaces of indentation to use
    #[clap(long, default_value_t = 4)]
    indent: isize,
    /// Open the generated pdf with the system viewer
    #[clap(long)]
    open: bool,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let cfg = PrintCfg {
        width: cmd.width,
        allow_linebreaks: true,
        latex: true,
        omit_decl_sep: true,
        indent: cmd.indent,
    };

    let mut drv = Driver::new();
    drv.print_compiled(&cmd.filepath, driver::PrintMode::Latex)?;
    drv.print_focused(&cmd.filepath, driver::PrintMode::Latex)?;
    drv.print_linearized(&cmd.filepath, driver::PrintMode::Latex)?;
    drv.print_inlined(&cmd.filepath, driver::PrintMode::Latex)?;
    drv.print_shrunk(&cmd.filepath, driver::PrintMode::Latex)?;
    drv.print_parsed_tex(&cmd.filepath, &cfg, &format!("{}", cmd.fontsize))?;
    match cmd.backend {
        Backend::Aarch64 => {
            drv.print_aarch64(&cmd.filepath, driver::PrintMode::Latex)?;
            drv.print_latex_all(&cmd.filepath, &Arch::AARCH64)?;
        }
        Backend::Rv64 => {}
        Backend::X86_64 => {
            drv.print_x86_64(&cmd.filepath, driver::PrintMode::Latex)?;
            drv.print_latex_all(&cmd.filepath, &Arch::X86_64)?;
        }
    }

    if cmd.open {
        drv.open_pdf(&cmd.filepath)?;
    }

    Ok(())
}
