mod compile_examples;
mod examples;
mod fun_tests;

fn main() {
    let working_dir = std::env::current_dir()
        .expect("Could not get working dir")
        .join("../../");
    std::env::set_current_dir(working_dir).expect("Could not set working dir");

    fun_tests::run_tests();
    compile_examples::run_tests();
}
