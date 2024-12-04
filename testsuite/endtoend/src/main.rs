mod compile_examples;
mod fun_tests;

fn main() {
    fun_tests::run_tests();
    compile_examples::run_tests();
}
