use driver::Driver;
use std::path::PathBuf;

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

#[derive(clap::ValueEnum, Clone)]
pub enum Backend {
    Aarch64,
    Rv64,
    X86_64,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let mut drv = Driver::new();
    let linearized = drv.linearized(&cmd.filepath);
    let _linearized = match linearized {
        Ok(linearized) => linearized,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };
    if cmd.print_ir {
        let _ = drv.print_compiled(&cmd.filepath);
        let _ = drv.print_focused(&cmd.filepath);
        let _ = drv.print_shrunk(&cmd.filepath);
        let _ = drv.print_linearized(&cmd.filepath);
    }

    match cmd.backend {
        Backend::Aarch64 => {
            let _ = drv.compile_aarch64(&cmd.filepath);
        }
        Backend::Rv64 => {
            let _ = drv.print_rv_64(&cmd.filepath);
        }
        Backend::X86_64 => {
            let _ = drv.compile_x86_64(&cmd.filepath);
        }
    }
    Ok(())
}
