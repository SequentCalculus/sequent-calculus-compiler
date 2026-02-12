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
            xtor_sig!("Cons", [bind!("xs", 0, prd!(), ty!("List")), bind!("x", 0)]),
        ],
    );

    let main_body = letin!(
        ("ws", 0),
        ty!("List"),
        "Nil",
        [],
        lit!(
            5,
            ("z", 0),
            letin!(
                ("zs", 0),
                ty!("List"),
                "Cons",
                [bind!("z", 0), bind!("ws", 0, prd!(), ty!("List"))],
                lit!(
                    7,
                    ("y", 0),
                    letin!(
                        ("ys", 0),
                        ty!("List"),
                        "Cons",
                        [bind!("y", 0), bind!("zs", 0, prd!(), ty!("List"))],
                        lit!(
                            9,
                            ("x", 0),
                            letin!(
                                ("xs", 0),
                                ty!("List"),
                                "Cons",
                                [bind!("x", 0), bind!("ys", 0, prd!(), ty!("List"))],
                                switch!(
                                    ("xs", 0),
                                    ty!("List"),
                                    [
                                        clause!("Nil", [], lit!(-1, ("err", 0), exit!(("err", 0)))),
                                        clause!(
                                            "Cons",
                                            [bind!("as", 0, prd!(), ty!("List")), bind!("a", 0)],
                                            println_i64!(
                                                ("a", 0),
                                                lit!(0, ("ret", 0), exit!(("ret", 0)))
                                            )
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
