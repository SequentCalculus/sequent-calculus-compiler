use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

#[test]
fn test_midi() {
    let program = examples::midi_print();
    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("midi.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
