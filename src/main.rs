pub mod compiler;
pub mod core;
pub mod fun;
pub mod grammar;
use grammar::fun::TermParser;

use std::env;

fn main() {
    let arg: String = env::args().next().unwrap();
    let parser = TermParser::new();
    let parsed = parser.parse(&arg).unwrap();
    println!("{}", parsed);
}
