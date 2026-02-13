use printer::Print;

fn main() {
    let program = examples::arith_print();
    println!("{}", program.print_to_string(None))
}
