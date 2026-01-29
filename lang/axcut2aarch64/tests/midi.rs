use axcut::syntax::statements::*;
use axcut2aarch64::Backend;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2backend::coder::compile;
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
            xtor_sig!("Cons", [bind!("xs", prd!(), ty!("List")), bind!("x")])
        ]
    );

    let ty_cont_list = ty_decl!(
        "ContList",
        [xtor_sig!("Retl", [bind!("kl", prd!(), ty!("List"))]),]
    );

    let ty_cont_int = ty_decl!("ContInt", [xtor_sig!("Reti", [bind!("ki")]),]);

    let main_body = create!(
        "t",
        ty!("ContInt"),
        [],
        [clause!(
            "Reti",
            [bind!("r")],
            println_i64!("r", lit!(0, "ret", exit!("ret")))
        )],
        create!(
            "k",
            ty!("ContList"),
            [bind!("t", cns!(), ty!("ContInt"))],
            [clause!(
                "Retl",
                [bind!("as", prd!(), ty!("List"))],
                substitute!(
                    [
                        (bind!("t", cns!(), ty!("ContInt")), "t"),
                        (bind!("as", prd!(), ty!("List")), "as"),
                    ],
                    call!("sum", [])
                )
            )],
            letin!(
                "zs",
                ty!("List"),
                "Nil",
                [],
                lit!(
                    3,
                    "n",
                    substitute!(
                        [
                            (bind!("k", cns!(), ty!("ContInt")), "k"),
                            (bind!("zs", prd!(), ty!("List")), "zs"),
                            (bind!("n"), "n",),
                        ],
                        call!("range", [])
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let range_body = Statement::IfC(ife!(
        "i",
        substitute!(
            [
                (bind!("xs", prd!(), ty!("List")), "xs"),
                (bind!("k", cns!(), ty!("ContList")), "k"),
            ],
            invoke!("k", "Retl", ty!("ContList"), [])
        ),
        substitute!(
            [
                (bind!("n"), "i"),
                (bind!("k", cns!(), ty!("ContList")), "k"),
                (bind!("xs", prd!(), ty!("List")), "xs"),
                (bind!("i"), "i"),
            ],
            letin!(
                "ys",
                ty!("List"),
                "Cons",
                [bind!("xs", prd!(), ty!("List")), bind!("i")],
                lit!(
                    -1,
                    "o",
                    sum!(
                        "n",
                        "o",
                        "j",
                        substitute!(
                            [
                                (bind!("k", cns!(), ty!("ContList")), "k"),
                                (bind!("ys", prd!(), ty!("List")), "ys"),
                                (bind!("j"), "j"),
                            ],
                            call!("range", [])
                        )
                    ),
                )
            )
        ),
    ));
    let range = def!(
        "range",
        [
            bind!("k", cns!(), ty!("ContList")),
            bind!("xs", prd!(), ty!("List")),
            bind!("i"),
        ],
        range_body
    );

    let sum_body = switch!(
        "xs",
        ty!("List"),
        [
            clause!(
                "Nil",
                [],
                lit!(
                    0,
                    "z",
                    substitute!(
                        [(bind!("z"), "z"), (bind!("k", cns!(), ty!("ContInt")), "k")],
                        invoke!("k", "Reti", ty!("ContInt"), [])
                    )
                )
            ),
            clause!(
                "Cons",
                [bind!("ys", prd!(), ty!("List")), bind!("y")],
                substitute!(
                    [
                        (bind!("ys", prd!(), ty!("List")), "ys"),
                        (bind!("k", cns!(), ty!("ContInt")), "k"),
                        (bind!("y"), "y"),
                    ],
                    create!(
                        "j",
                        ty!("ContInt"),
                        [bind!("k", cns!(), ty!("ContInt")), bind!("y")],
                        [clause!(
                            "Reti",
                            [bind!("r")],
                            sum!(
                                "y",
                                "r",
                                "s",
                                substitute!(
                                    [
                                        (bind!("s"), "s"),
                                        (bind!("k", cns!(), ty!("ContInt")), "k",),
                                    ],
                                    invoke!("k", "Reti", ty!("ContInt"), [])
                                )
                            )
                        )],
                        substitute!(
                            [
                                (bind!("j", cns!(), ty!("ContInt")), "j"),
                                (bind!("ys", prd!(), ty!("List")), "ys"),
                            ],
                            call!("sum", [])
                        )
                    )
                )
            )
        ]
    );
    let sum = def!(
        "sum",
        [
            bind!("k", cns!(), ty!("ContList")),
            bind!("xs", prd!(), ty!("List"))
        ],
        sum_body
    );

    let program = prog!([main, range, sum], [ty_list, ty_cont_list, ty_cont_int]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("midi.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
