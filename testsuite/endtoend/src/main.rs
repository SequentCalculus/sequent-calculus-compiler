mod compile_examples;
mod examples;
mod fun_tests;

use examples::{load_examples, load_fail, load_success, ExampleResult};

fn main() -> Result<(), String> {
    let working_dir = std::env::current_dir()
        .map_err(|err| format!("Could not get working dir: {err}"))
        .map(|dir| dir.join("../../"))?;
    std::env::set_current_dir(working_dir)
        .map_err(|err| format!("Could not set working dir: {err}"))?;

    let examples = load_examples()?;
    let success_examples = load_success()?;
    let fail_examples = load_fail()?;
    let fun_results = fun_tests::run_tests(&examples, success_examples, fail_examples);
    let compile_results = compile_examples::run_tests(&examples);
    ExampleResult::assert_success(fun_results);
    ExampleResult::assert_success(compile_results);
    Ok(())
}
