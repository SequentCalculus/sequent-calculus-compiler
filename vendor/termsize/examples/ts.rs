pub fn main() {
    match termsize::get() {
        Some(size) => println!("{:?}", size),
        _ => println!("not a term"),
    }
}
