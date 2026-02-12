use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{
    bind, clause, def, exit, letin, lit, prd, println_i64, prog, substitute, sum, switch, ty,
    ty_decl, xtor_sig,
};

#[test]
fn test_non_linear() {
    let ty_box = ty_decl!("Box", [xtor_sig!("B", [bind!("b", 0)])]);
    let ty_box_box = ty_decl!(
        "BoxBox",
        [xtor_sig!("BB", [bind!("bb", 0, prd!(), ty!("Box"))])],
    );

    let main_body_switch_switch = switch!(
        ("a", 2),
        ty!("Box"),
        [clause!(
            "B",
            [bind!("x", 2)],
            substitute!(
                [
                    (bind!("x", 2), ("x", 2)),
                    (bind!("a", 1, prd!(), ty!("Box")), ("a", 1)),
                ],
                switch!(
                    ("a", 1),
                    ty!("Box"),
                    [clause!(
                        "B",
                        [bind!("x", 1)],
                        sum!(
                            ("x", 1),
                            ("x", 2),
                            ("res", 0),
                            println_i64!(("res", 0), lit!(0, ("ret", 0), exit!(("ret", 0))))
                        )
                    )]
                )
            )
        )]
    );

    let main_body_switch = switch!(
        ("bb", 1),
        ty!("BoxBox"),
        [clause!(
            "BB",
            [bind!("b", 1, prd!(), ty!("Box"))],
            switch!(
                ("b", 1),
                ty!("Box"),
                [clause!(
                    "B",
                    [bind!("x", 1)],
                    letin!(
                        ("d", 1),
                        ty!("Box"),
                        "B",
                        [bind!("x", 1)],
                        letin!(
                            ("dd", 1),
                            ty!("BoxBox"),
                            "BB",
                            [bind!("d", 1, prd!(), ty!("Box"))],
                            substitute!(
                                [(bind!("bb", 2, prd!(), ty!("BoxBox")), ("bb", 2),)],
                                lit!(
                                    4,
                                    ("y", 0),
                                    letin!(
                                        ("a", 1),
                                        ty!("Box"),
                                        "B",
                                        [bind!("y", 0)],
                                        substitute!(
                                            [
                                                (bind!("a", 1, prd!(), ty!("Box")), ("a", 1)),
                                                (bind!("bb", 2, prd!(), ty!("BoxBox")), ("bb", 2)),
                                            ],
                                            switch!(
                                                ("bb", 2),
                                                ty!("BoxBox"),
                                                [clause!(
                                                    "BB",
                                                    [bind!("b", 2, prd!(), ty!("Box"))],
                                                    switch!(
                                                        ("b", 2),
                                                        ty!("Box"),
                                                        [clause!(
                                                            "B",
                                                            [bind!("x", 2)],
                                                            letin!(
                                                                ("a", 2),
                                                                ty!("Box"),
                                                                "B",
                                                                [bind!("x", 2)],
                                                                main_body_switch_switch
                                                            )
                                                        )]
                                                    )
                                                )]
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )]
            )
        )]
    );
    let main_body = lit!(
        3,
        ("f", 1),
        lit!(
            3,
            ("f", 2),
            lit!(
                3,
                ("f", 3),
                lit!(
                    3,
                    ("f", 4),
                    lit!(
                        3,
                        ("f", 5),
                        lit!(
                            3,
                            ("f", 6),
                            lit!(
                                3,
                                ("f", 7),
                                lit!(
                                    3,
                                    ("x", 0),
                                    letin!(
                                        ("b", 0),
                                        ty!("Box"),
                                        "B",
                                        [bind!("x", 0)],
                                        letin!(
                                            ("bb", 0),
                                            ty!("BoxBox"),
                                            "BB",
                                            [bind!("b", 0, prd!(), ty!("Box"))],
                                            substitute!(
                                                [
                                                    (bind!("f", 1), ("f", 1)),
                                                    (bind!("f", 2), ("f", 2)),
                                                    (bind!("f", 3), ("f", 3)),
                                                    (bind!("f", 5), ("f", 5)),
                                                    (bind!("f", 6), ("f", 6)),
                                                    (bind!("f", 7), ("f", 7)),
                                                    (bind!("f", 4), ("f", 4)),
                                                    (
                                                        bind!("bb", 3, prd!(), ty!("BoxBox")),
                                                        ("bb", 0)
                                                    ),
                                                    (
                                                        bind!("bb", 2, prd!(), ty!("BoxBox")),
                                                        ("bb", 0)
                                                    ),
                                                    (
                                                        bind!("bb", 1, prd!(), ty!("BoxBox")),
                                                        ("bb", 0)
                                                    ),
                                                ],
                                                main_body_switch
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_box, ty_box_box]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("nonLinear.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
