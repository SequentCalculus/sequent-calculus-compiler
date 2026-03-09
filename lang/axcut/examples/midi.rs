use axcut::syntax::statements::*;
use axcut::syntax::*;
use printer::Print;

use std::collections::HashSet;
use std::rc::Rc;

fn main() {
    let prog = axcut_examples::midi_print();
    println!("{}", prog.linearize().print_to_string(None))
}
