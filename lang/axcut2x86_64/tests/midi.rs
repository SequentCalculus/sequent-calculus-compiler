use axcut::syntax::statements::*;
use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{
    bind, call, clause, cns, create, def, exit, ife, invoke, letin, lit, prd, println_i64, prog,
    substitute, sum, switch, ty, ty_decl, xtor_sig,
};

#[test]
fn test_midi() {
    let ty_list = ty_decl!(
        "List",
        [
            xtor_sig!("Nil", []),
            xtor_sig!("Cons", [bind!("xs", 0, prd!(), ty!("List")), bind!("x", 0)]),
        ],
    );

    let ty_cont_list = ty_decl!(
        "ContList",
        [xtor_sig!("Retl", [bind!("kl", 0, prd!(), ty!("List"))])]
    );

    let ty_cont_int = ty_decl!("ContInt", [xtor_sig!("Reti", [bind!("ki", 0)])]);

    let main_body = create!(
        ("t", 0),
        ty!("ContInt"),
        [],
        [clause!(
            "Reti",
            [bind!("r", 0)],
            println_i64!(("r", 0), lit!(0, ("ret", 0), exit!(("ret", 0))))
        )],
        create!(
            ("k", 0),
            ty!("ContList"),
            [bind!("t", 0, cns!(), ty!("ContInt"))],
            [clause!(
                "Retl",
                [bind!("as", 0, prd!(), ty!("List"))],
                substitute!(
                    [
                        (bind!("t", 0, cns!(), ty!("ContInt")), ("t", 0)),
                        (bind!("as", 0, prd!(), ty!("List")), ("as", 0)),
                    ],
                    call!("sum", [])
                )
            )],
            letin!(
                ("zs", 0),
                ty!("List"),
                "Nil",
                [],
                lit!(
                    3,
                    ("n", 0),
                    substitute!(
                        [
                            (bind!("k", 0, cns!(), ty!("ContInt")), ("k", 0)),
                            (bind!("zs", 0, prd!(), ty!("List")), ("zs", 0)),
                            (bind!("n", 0), ("n", 0)),
                        ],
                        call!("range", [])
                    )
                )
            )
        )
    );

    let main = def!("main", [], main_body);

    let range_body = ife!(
        ("i", 0),
        substitute!(
            [
                (bind!("xs", 0, prd!(), ty!("List")), ("xs", 0)),
                (bind!("k", 0, cns!(), ty!("ContList")), ("k", 0)),
            ],
            invoke!(("k", 0), "Retl", ty!("ContList"), [])
        ),
        substitute!(
            [
                (bind!("n", 0), ("i", 0)),
                (bind!("k", 0, cns!(), ty!("ContList")), ("k", 0)),
                (bind!("xs", 0, prd!(), ty!("List")), ("xs", 0)),
                (bind!("i", 0), ("i", 0)),
            ],
            letin!(
                ("ys", 0),
                ty!("List"),
                "Cons",
                [bind!("xs", 0, prd!(), ty!("List")), bind!("i", 0)],
                lit!(
                    -1,
                    ("o", 0),
                    sum!(
                        ("n", 0),
                        ("o", 0),
                        ("j", 0),
                        substitute!(
                            [
                                (bind!("k", 0, cns!(), ty!("ContList")), ("k", 0)),
                                (bind!("ys", 0, prd!(), ty!("List")), ("ys", 0)),
                                (bind!("j", 0), ("j", 0)),
                            ],
                            call!("range", [])
                        )
                    )
                )
            )
        )
    );
    let range = def!(
        "range",
        [
            bind!("k", 0, cns!(), ty!("ContList")),
            bind!("xs", 0, prd!(), ty!("List")),
            bind!("i", 0)
        ],
        range_body
    );

    let sum_body = switch!(
        ("xs", 0),
        ty!("List"),
        [
            clause!(
                "Nil",
                [],
                lit!(
                    0,
                    ("z", 0),
                    substitute!(
                        [
                            (bind!("z", 0), ("z", 0)),
                            (bind!("k", 0, cns!(), ty!("ContInt")), ("k", 0)),
                        ],
                        invoke!(("k", 0), "Reti", ty!("ContInt"), [])
                    )
                )
            ),
            clause!(
                "Cons",
                [bind!("ys", 0, prd!(), ty!("List")), bind!("y", 0)],
                substitute!(
                    [
                        (bind!("ys", 0, prd!(), ty!("List")), ("ys", 0)),
                        (bind!("k", 0, cns!(), ty!("ContInt")), ("k", 0)),
                        (bind!("y", 0), ("y", 0)),
                    ],
                    create!(
                        ("j", 0),
                        ty!("ContInt"),
                        [bind!("k", 0, cns!(), ty!("ContInt")), bind!("y", 0)],
                        [clause!(
                            "Reti",
                            [bind!("r", 0)],
                            sum!(
                                ("y", 0),
                                ("r", 0),
                                ("s", 0),
                                substitute!(
                                    [
                                        (bind!("s", 0), ("s", 0)),
                                        (bind!("k", 0, cns!(), ty!("ContInt")), ("k", 0)),
                                    ],
                                    invoke!(("k", 0), "Reti", ty!("ContInt"), [])
                                )
                            )
                        )],
                        substitute!(
                            [
                                (bind!("j", 0, cns!(), ty!("ContInt")), ("j", 0)),
                                (bind!("ys", 0, prd!(), ty!("List")), ("ys", 0)),
                            ],
                            call!("sum", [])
                        )
                    )
                )
            ),
        ]
    );
    let sum = def!(
        "sum",
        [
            bind!("k", 0, cns!(), ty!("ContList")),
            bind!("xs", 0, prd!(), ty!("List"))
        ],
        sum_body
    );

    let program = prog!([main, range, sum], [ty_list, ty_cont_list, ty_cont_int]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("midi.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
