mod cli;
mod utils;

fn main() -> miette::Result<()> {
    cli::exec()
}
