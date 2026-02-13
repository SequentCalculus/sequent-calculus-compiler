use printer::Print;

fn main() {
    let program = examples::either_print();
    println!("{}", program.print_to_string(None))
}
