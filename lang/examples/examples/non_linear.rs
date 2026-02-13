use printer::Print;

fn main() {
    let program = examples::non_linear_print();
    println!("{}", program.print_to_string(None))
}
