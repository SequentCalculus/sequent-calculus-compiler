use axcut2aarch64::Backend;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2backend::coder::compile;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{
    bind, clause, cns, create, def, exit, invoke, lit, prd, println_i64, prog, substitute, sum, ty,
    ty_decl, xtor_sig,
};
#[test]
fn test_closure() {
    let ty_cont = ty_decl!("Cont", [xtor_sig!("Ret", [bind!("r")])],);
    let ty_func = ty_decl!(
        "Fun",
        [xtor_sig!(
            "apply",
            [bind!("x"), bind!("k", cns!(), ty!("Cont"))]
        )],
    );

    let main_body = lit!(
        9,
        "a",
        create!(
            "f",
            ty!("Fun"),
            [bind!("a")],
            [clause!(
                "apply",
                [bind!("x"), bind!("k", cns!(), ty!("Cont"))],
                sum!(
                    "a",
                    "x",
                    "b",
                    substitute!(
                        [(bind!("b"), "b"), (bind!("k", cns!(), ty!("Cont")), "k")],
                        invoke!("k", "Ret", ty!("Cont"), []),
                    )
                )
            )],
            create!(
                "k",
                ty!("Cont"),
                [],
                [clause!(
                    "Ret",
                    [bind!("r")],
                    println_i64!("r", lit!(0, "ret", exit!("ret"))),
                ),],
                lit!(
                    1,
                    "y",
                    substitute!(
                        [
                            (bind!("y"), "y"),
                            (bind!("k", cns!(), ty!("Cont")), "k"),
                            (bind!("f", prd!(), ty!("Fun")), "f"),
                        ],
                        invoke!("f", "apply", ty!("Fun"), [])
                    )
                ),
            ),
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_cont, ty_func]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("closure.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
