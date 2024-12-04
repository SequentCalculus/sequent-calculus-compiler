mod compile_examples;
mod examples;
mod fun_tests;
mod load_examples;

use examples::ExampleResult;
use load_examples::load_all;

fn setup() -> Result<(), String> {
    let working_dir = std::env::current_dir()
        .map_err(|err| format!("Could not get working dir: {err}"))
        .map(|dir| dir.join("../../"))?;
    std::env::set_current_dir(working_dir)
        .map_err(|err| format!("Could not set working dir: {err}"))?;
    Ok(())
}

fn main() -> Result<(), String> {
    setup()?;
    let examples = load_all()?;
    let fun_results = fun_tests::run_tests(&examples);
    let compile_results = compile_examples::run_tests(&examples.examples);
    ExampleResult::assert_success(fun_results);
    ExampleResult::assert_success(compile_results);
    Ok(())
}
