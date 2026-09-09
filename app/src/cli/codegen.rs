//! This module contains the command for generating assembly code for a file.

use driver::{Driver, PrintMode};
use std::path::PathBuf;

use crate::utils::cli_options::Backend;

#[derive(clap::Args)]
pub struct Args {
    /// Which file to compile
    filepath: PathBuf,
    /// Which backend to use
    backend: Backend,
    /// Optional heap size in MB, default is 32
    #[arg(long)]
    heap_size: Option<usize>,
    /// Write intermediate representations to disk
    #[arg(long)]
    print_ir: bool,
    /// Disable type splitting: a growing cycle is erased directly on the unsplit program instead
    /// (widening more declarations than necessary). On by default, since splitting only ever
    /// helps precision; useful for comparing against the unsplit baseline in benchmarks.
    #[arg(long = "no-split")]
    no_split: bool,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    drv.set_split(!cmd.no_split);
    let linearized = drv.linearized(&cmd.filepath);
    let _linearized = match linearized {
        Ok(linearized) => linearized,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    if cmd.print_ir {
        drv.print_compiled(&cmd.filepath, PrintMode::Textual)?;
        drv.print_focused(&cmd.filepath, PrintMode::Textual)?;
        drv.print_shrunk(&cmd.filepath, PrintMode::Textual)?;
        drv.print_linearized(&cmd.filepath, PrintMode::Textual)?;
    }

    match cmd.backend {
        Backend::Aarch64 => {
            drv.compile_aarch64(&cmd.filepath, cmd.heap_size)?;
        }
        Backend::Rv64 => {
            drv.print_rv_64(&cmd.filepath, PrintMode::Textual)?;
        }
        Backend::X86_64 => {
            drv.compile_x86_64(&cmd.filepath, cmd.heap_size)?;
        }
    }
    Ok(())
}
