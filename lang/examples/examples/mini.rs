use printer::Print;

fn main() {
    let program = examples::mini_print();
    println!("{}", program.print_to_string(None))
}
