use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

use axcut_macros::{
    bind, clause, cns, create, def, exit, invoke, lit, prd, prog, substitute, sum, ty, ty_decl,
    xtor_sig,
};

#[test]
fn test_closure() {
    let ty_cont = ty_decl!("Cont", [xtor_sig!("Ret", [bind!("r", 0)])]);

    let ty_func = ty_decl!(
        "Fun",
        [xtor_sig!(
            "apply",
            [bind!("x", 0), bind!("k", 0, cns!(), ty!("Cont"))]
        )]
    );

    let main_body = lit!(
        9,
        ("a", 0),
        create!(
            ("f", 0),
            ty!("Fun"),
            [bind!("a", 0)],
            [clause!(
                "apply",
                [bind!("x", 0), bind!("k", 0, cns!(), ty!("Cont"))],
                sum!(
                    ("a", 0),
                    ("x", 0),
                    ("b", 0),
                    substitute!(
                        [
                            (bind!("b", 0), ("b", 0)),
                            (bind!("k", 0, cns!(), ty!("Cont")), ("k", 0))
                        ],
                        invoke!(("k", 0), "Ret", ty!("Cont"), [])
                    )
                )
            )],
            create!(
                ("k", 0),
                ty!("Cont"),
                [],
                [clause!("Ret", [bind!("r", 0)], exit!(("r", 0)))],
                lit!(
                    1,
                    ("y", 0),
                    substitute!(
                        [
                            (bind!("y", 0), ("y", 0)),
                            (bind!("k", 0, cns!(), ty!("Cont")), ("k", 0)),
                            (bind!("f", 0, prd!(), ty!("Fun")), ("f", 0)),
                        ],
                        invoke!(("f", 0), "apply", ty!("Fun"), [])
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_cont, ty_func]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("closure.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
