use axcut2aarch64::Backend;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2backend::coder::compile;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{
    bind, clause, def, exit, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

#[test]
fn test_quad() {
    let ty_quad = ty_decl!(
        "Quad",
        [xtor_sig!(
            "Q",
            [bind!("d"), bind!("c"), bind!("b"), bind!("a")]
        )]
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
                                lit!(
                                    7,
                                    "z",
                                    sum!(
                                        "d",
                                        "z",
                                        "e",
                                        println_i64!("e", lit!(0, "ret", exit!("ret")))
                                    )
                                )
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
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("quad.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
