use axcut::syntax::statements::*;
use axcut2aarch64::Backend;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2backend::coder::compile;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{bind, call, def, exit, lit, println_i64, prog, sum};

#[test]
fn test_mini() {
    let main_body = call!("l", []);
    let main = def!("main", [], main_body);

    let l_body = Statement::Literal(lit!(1, ("x", 0), lit!(9, ("y", 0), call!("j", []))));
    let l = def!("l", [], l_body);

    let j_body = Statement::Op(sum!(
        ("x", 0),
        ("y", 0),
        ("z", 0),
        println_i64!(("z", 0), lit!(0, ("ret", 0), exit!(("ret", 0))),)
    ));
    let j = def!("j", [bind!("y", 0), bind!("x", 0)], j_body);

    let program = prog!([main, l, j], []);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("mini.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
