mod compile_examples;
mod errors;
mod examples;
mod fun_tests;
mod load_examples;

use errors::Error;
use examples::ExampleResult;
use load_examples::{load_all, load_bench};

fn setup() -> Result<(), Error> {
    let working_dir = std::env::current_dir()
        .map_err(|err| Error::working_dir("get", err))
        .map(|dir| dir.join("../../"))?;
    std::env::set_current_dir(working_dir).map_err(|err| Error::working_dir("set", err))?;
    Ok(())
}

fn main() -> Result<(), Error> {
    setup()?;
    let benchmarks = load_bench()?;
    return Ok(());

    let examples = load_all()?;
    println!("Running fun tests");
    let fun_results = fun_tests::run_tests(&examples);
    ExampleResult::report(fun_results)?;

    println!("Running compile tests");
    let compile_results = compile_examples::run_tests(&examples.examples);
    ExampleResult::report(compile_results)
}
