use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

#[test]
fn test_either() {
    let program = axcut2backend::examples::either_exit();
    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("either.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
