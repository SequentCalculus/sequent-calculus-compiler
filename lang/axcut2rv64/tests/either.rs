use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

use axcut_macros::{bind, clause, def, exit, letin, lit, prog, sum, switch, ty, ty_decl, xtor_sig};

#[test]
fn test_either() {
    let ty_either = ty_decl!(
        "Either",
        [
            xtor_sig!("Left", [bind!("x")]),
            xtor_sig!("Right", [bind!("y")],),
        ],
    );

    let main_body = lit!(
        1,
        "z",
        lit!(
            9,
            "x",
            letin!(
                "p",
                ty!("Either"),
                "Right",
                [bind!("x")],
                switch!(
                    "p",
                    ty!("Either"),
                    [
                        clause!("Left", [bind!("a")], lit!(-1, "err", exit!("err"))),
                        clause!("Right", [bind!("b")], sum!("b", "z", "c", exit!("c"))),
                    ]
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_either]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("either.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
