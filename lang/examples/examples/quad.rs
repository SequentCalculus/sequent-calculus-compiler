use printer::Print;

fn main() {
    let program = examples::quad_print();
    println!("{}", program.print_to_string(None))
}
