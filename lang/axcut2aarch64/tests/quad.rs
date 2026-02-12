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
            [bind!("d", 0), bind!("c", 0), bind!("b", 0), bind!("a", 0)]
        )]
    );

    let main_body = lit!(
        8,
        ("z", 0),
        lit!(
            6,
            ("y", 0),
            lit!(
                4,
                ("x", 0),
                lit!(
                    2,
                    ("w", 0),
                    letin!(
                        ("q", 0),
                        ty!("Quad"),
                        "Q",
                        [bind!("z", 0), bind!("y", 0), bind!("x", 0), bind!("w", 0)],
                        switch!(
                            ("q", 0),
                            ty!("Quad"),
                            [clause!(
                                "Q",
                                [bind!("d", 0), bind!("c", 0), bind!("b", 0), bind!("a", 0)],
                                lit!(
                                    7,
                                    ("z", 0),
                                    sum!(
                                        ("d", 0),
                                        ("z", 0),
                                        ("e", 0),
                                        println_i64!(
                                            ("e", 0),
                                            lit!(0, ("ret", 0), exit!(("ret", 0)))
                                        )
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
