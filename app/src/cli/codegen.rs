use driver::{Driver, PrintMode};
use std::path::PathBuf;

use crate::utils::cli_options::Backend;

#[derive(clap::Args)]
pub struct Args {
    /// Which file to compile
    filepath: PathBuf,
    /// Which backend to use
    backend: Backend,
    /// Write intermediate representations to disk
    #[arg(long)]
    print_ir: bool,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let linearized = drv.linearized(&cmd.filepath);
    let _linearized = match linearized {
        Ok(linearized) => linearized,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    if cmd.print_ir {
        let _ = drv.print_compiled(&cmd.filepath, PrintMode::Textual);
        let _ = drv.print_focused(&cmd.filepath, PrintMode::Textual);
        let _ = drv.print_shrunk(&cmd.filepath, PrintMode::Textual);
        let _ = drv.print_linearized(&cmd.filepath, PrintMode::Textual);
    }

    match cmd.backend {
        Backend::Aarch64 => {
            let _ = drv.compile_aarch64(&cmd.filepath);
        }
        Backend::Rv64 => {
            let _ = drv.print_rv_64(&cmd.filepath, PrintMode::Textual);
        }
        Backend::X86_64 => {
            let _ = drv.compile_x86_64(&cmd.filepath);
        }
    }
    Ok(())
}
