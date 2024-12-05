mod cli;

fn main() -> miette::Result<()> {
    cli::exec()
}
