use axcut2backend::{code::pretty, coder::compile};
use driver::Driver;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    backend: Backend,
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
    let linearized = match linearized {
        Ok(linearized) => linearized,
        Err(err) => return Err(drv.error_to_report(err, &cmd.filepath)),
    };

    match cmd.backend {
        Backend::Aarch64 => {
            let code = compile(linearized, &axcut2aarch64::Backend);
            //let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
            //let _ = code.print_colored(&Default::default(), &mut stream);
            println!(
                "{}",
                axcut2aarch64::into_routine::into_aarch64_routine(
                    "filename",
                    &pretty(code.0),
                    code.1
                )
            );
        }
        Backend::Rv64 => {
            let code = compile(linearized, &axcut2rv64::Backend);
            //let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
            //let _ = code.print_colored(&Default::default(), &mut stream);
            println!(
                "{}",
                axcut2rv64::into_routine::into_rv64_routine("filename", &pretty(code.0), code.1)
            );
        }
        Backend::X86_64 => {
            let code = compile(linearized, &axcut2x86_64::Backend);
            //let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
            //let _ = code.print_colored(&Default::default(), &mut stream);
            println!(
                "{}",
                axcut2x86_64::into_routine::into_x86_64_routine(
                    "filename",
                    &pretty(code.0),
                    code.1
                )
            );
        }
    }
    Ok(())
}
