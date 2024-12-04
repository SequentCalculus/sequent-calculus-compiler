mod compile_examples;
mod examples;
mod fun_tests;

use examples::{load_examples, ExampleResult};

fn main() {
    let working_dir = std::env::current_dir()
        .expect("Could not get working dir")
        .join("../../");
    std::env::set_current_dir(working_dir).expect("Could not set working dir");

    let examples = load_examples();
    let fun_results = fun_tests::run_tests(&examples);
    let compile_results = compile_examples::run_tests(&examples);
    ExampleResult::assert_success(fun_results);
    ExampleResult::assert_success(compile_results);
}
