mod end_to_end_tests;
mod errors;
mod fun_tests;
mod load_tests;

use errors::Error;
use fun_tests::TestResult;
use load_tests::load_all;

fn setup() -> Result<(), Error> {
    let working_dir = std::env::current_dir()
        .map_err(|err| Error::working_dir("get", err))
        .map(|dir| dir.join("../"))?;
    std::env::set_current_dir(working_dir).map_err(|err| Error::working_dir("set", err))?;
    Ok(())
}

fn main() -> Result<(), Error> {
    setup()?;

    let tests = load_all()?;

    println!("Running Fun tests");
    let fun_results = fun_tests::run_tests(&tests);
    TestResult::report(fun_results)?;

    println!("Running end-to-end tests");
    let compile_results = end_to_end_tests::run_tests(&tests.end_to_end_tests);
    TestResult::report(compile_results)
}
