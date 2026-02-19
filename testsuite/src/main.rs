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

fn lift_err<T>(res: Result<T, Error>, step: &str) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error while {step}:\n\n{err}");
            std::process::exit(1);
        }
    }
}

fn main() {
    lift_err(setup(), "setting up harness");
    let tests = lift_err(load_all(), "loading tests");

    println!("Running Fun tests");
    let fun_results = fun_tests::run_tests(&tests);
    lift_err(TestResult::report(fun_results), "running Fun tests");

    println!("Running end-to-end tests");
    let compile_results = end_to_end_tests::run_tests(&tests.end_to_end_tests);
    lift_err(
        TestResult::report(compile_results),
        "running end-to-end tests",
    );
}
