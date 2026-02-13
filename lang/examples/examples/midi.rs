use printer::Print;
fn main() {
    let program = examples::midi();
    println!("{}", program.linearize().print_to_string(None))
}
