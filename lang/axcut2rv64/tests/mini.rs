use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

use axcut_macros::{bind, call, def, exit, lit, prog, sum};

#[test]
fn test_mini() {
    let main_body = call!("l", []);
    let main = def!("main", [], main_body);

    let l_body = lit!(1, ("x", 0), lit!(9, ("y", 0), call!("j", [])));
    let l = def!("l", [], l_body);

    let j_body = sum!(("x", 0), ("y", 0), ("z", 0), exit!(("z", 0)));
    let j = def!("j", [bind!("y", 0), bind!("x", 0)], j_body);

    let program = prog!([main, l, j], []);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("mini.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
