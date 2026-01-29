use axcut2aarch64::Backend;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2backend::coder::compile;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{
    bind, clause, def, exit, letin, lit, prd, println_i64, prog, switch, ty, ty_decl, xtor_sig,
};

#[test]
fn test_list() {
    let ty_list = ty_decl!(
        "List",
        [
            xtor_sig!("Nil", []),
            xtor_sig!("Cons", [bind!("xs", prd!(), ty!("List")), bind!("x")]),
        ],
    );

    let main_body = letin!(
        "ws",
        ty!("List"),
        "Nil",
        [],
        lit!(
            5,
            "z",
            letin!(
                "zs",
                ty!("List"),
                "Cons",
                [bind!("z"), bind!("ws", prd!(), ty!("List"))],
                lit!(
                    7,
                    "y",
                    letin!(
                        "ys",
                        ty!("List"),
                        "Cons",
                        [bind!("y"), bind!("zs", prd!(), ty!("List"))],
                        lit!(
                            9,
                            "x",
                            letin!(
                                "xs",
                                ty!("List"),
                                "Cons",
                                [bind!("x"), bind!("ys", prd!(), ty!("List"))],
                                switch!(
                                    "xs",
                                    ty!("List"),
                                    [
                                        clause!("Nil", [], lit!(-1, "err", exit!("err"))),
                                        clause!(
                                            "Cons",
                                            [bind!("as", prd!(), ty!("List")), bind!("a"),],
                                            println_i64!("a", lit!(0, "ret", exit!("ret")))
                                        ),
                                    ]
                                )
                            )
                        )
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_list]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("list.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
