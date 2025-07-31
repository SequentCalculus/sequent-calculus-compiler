//! This crate is the binary crate of the project, generating the resulting `scc` compiler binary
//! that can be called from the command line.

mod cli;
mod utils;

fn main() -> miette::Result<()> {
    cli::exec()
}
