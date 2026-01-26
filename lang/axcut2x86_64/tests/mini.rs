use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{bind, call, def, exit, lit, println_i64, prog, sum};

#[test]
fn test_mini() {
    let main_body = call!("l", []);
    let main = def!("main", [], main_body);

    let l_body = lit!(1, "x", lit!(9, "y", call!("j", [])));
    let l = def!("l", [], l_body);

    let j_body = sum!(
        "x",
        "y",
        "z",
        println_i64!("z", lit!(0, "ret", exit!("ret")))
    );
    let j = def!("j", [bind!("y"), bind!("x")], j_body);

    let program = prog!([main, l, j], []);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("mini.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
