use printer::Print;

fn main() {
    let program = examples::midi_print();
    println!("{}", program.print_to_string(None))
}
