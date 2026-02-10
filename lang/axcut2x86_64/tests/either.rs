use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{
    bind, clause, def, exit, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

#[test]
fn test_either() {
    let ty_either = ty_decl!(
        "Either",
        [
            xtor_sig!("Left", [bind!("x")]),
            xtor_sig!("Right", [bind!("y")]),
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
                        clause!(
                            "Right",
                            [bind!("b")],
                            sum!(
                                "b",
                                "z",
                                "c",
                                println_i64!("c", lit!(0, "ret", exit!("ret")))
                            )
                        ),
                    ]
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_either]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("either.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
