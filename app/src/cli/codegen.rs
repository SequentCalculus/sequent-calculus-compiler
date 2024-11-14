use axcut2backend::{code::pretty, coder::compile};
use std::path::PathBuf;

use super::parse_and_check_from_file;

use axcut::syntax::program::linearize;
use core::syntax::program::transform_prog;
use core2axcut::program::translate_prog;
use fun2core::program::compile_prog;
//use printer::{ColorChoice, Print, StandardStream};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
    backend: String,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let parsed = parse_and_check_from_file(cmd.filepath)?;
    let compiled = compile_prog(parsed);
    let focused = transform_prog(compiled);
    let shrunk = translate_prog(focused);
    let linearized = linearize(shrunk);
    match cmd.backend.as_str() {
        "aarch64" => {
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
        "rv64" => {
            let code = compile(linearized, &axcut2rv64::Backend);
            //let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
            //let _ = code.print_colored(&Default::default(), &mut stream);
            println!(
                "{}",
                axcut2rv64::into_routine::into_rv64_routine(
                    "filename",
                    &pretty(code.0),
                    code.1
                )
            );
        }
        "x86_64" => {
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
        _ => {}
    }
    Ok(())
}
