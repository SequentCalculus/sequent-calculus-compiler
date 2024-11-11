use std::path::PathBuf;

use super::parse_and_check_from_file;

use fun2core::program::compile_prog;
use printer::{ColorChoice, Print, StandardStream};

#[derive(clap::Args)]
pub struct Args {
    filepath: PathBuf,
}

pub fn exec(cmd: Args) -> miette::Result<()> {
    let parsed = parse_and_check_from_file(cmd.filepath)?;
    let compiled = compile_prog(parsed);
    let mut stream = Box::new(StandardStream::stdout(ColorChoice::Auto));
    let _ = compiled.print_colored(&Default::default(), &mut stream);
    Ok(())
}
