use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

use axcut_macros::{bind, clause, def, exit, letin, lit, prog, sum, switch, ty, ty_decl, xtor_sig};

#[test]
fn test_quad() {
    let ty_quad = ty_decl!(
        "Quad",
        [xtor_sig!(
            "Q",
            [bind!("d"), bind!("c"), bind!("b"), bind!("a")]
        )],
    );

    let main_body = lit!(
        8,
        "z",
        lit!(
            6,
            "y",
            lit!(
                4,
                "x",
                lit!(
                    2,
                    "w",
                    letin!(
                        "q",
                        ty!("Quad"),
                        "Q",
                        [bind!("z"), bind!("y"), bind!("x"), bind!("w")],
                        switch!(
                            "q",
                            ty!("Quad"),
                            [clause!(
                                "Q",
                                [bind!("d"), bind!("c"), bind!("b"), bind!("a")],
                                lit!(7, "z", sum!("d", "z", "e", exit!("e")))
                            )]
                        )
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_quad]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("quad.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
