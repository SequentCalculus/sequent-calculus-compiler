use axcut::syntax::statements::*;
use printer::Print;

use axcut_macros::{
    bind, call, clause, cns, create, def, exit, ife, invoke, letin, lit, prd, prog, sum, switch,
    ty, ty_decl, xtor_sig,
};

fn main() {
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
        [clause!("Reti", [bind!("r", 0)], exit!(("r", 0)))],
        create!(
            ("k", 0),
            ty!("ContList"),
            [clause!(
                "Retl",
                [bind!("as", 0, prd!(), ty!("List"))],
                call!(
                    "sum",
                    [
                        bind!("t", 0, cns!(), ty!("ContInt")),
                        bind!("as", 0, prd!(), ty!("List")),
                    ]
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
                    call!(
                        "range",
                        [
                            bind!("k", 0, cns!(), ty!("ContList")),
                            bind!("zs", 0, prd!(), ty!("List")),
                            bind!("n", 0),
                        ]
                    )
                )
            )
        )
    );
    let main = def!(
        "main",
        [],
        main_body,
        [("t", 0), ("zs", 0), ("n", 0), ("k", 0), ("as", 0), ("r", 0)]
    );

    let range_body = ife!(
        ("i", 0),
        invoke!(
            ("k", 0),
            "Retl",
            ty!("ContList"),
            [bind!("xs", 0, prd!(), ty!("List"))]
        ),
        letin!(
            ("ys", 0),
            ty!("List"),
            "Cons",
            [bind!("xs", 0, prd!(), ty!("List")), bind!("i", 0)],
            lit!(
                -1,
                ("o", 0),
                sum!(
                    ("i", 0),
                    ("o", 0),
                    ("j", 0),
                    call!(
                        "range",
                        [
                            bind!("k", 0, cns!(), ty!("ContList")),
                            bind!("ys", 0, prd!(), ty!("List")),
                            bind!("j", 0),
                        ]
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
            bind!("i", 0),
        ],
        range_body,
        [("k", 0), ("xs", 0), ("i", 0), ("j", 0), ("o", 0), ("ys", 0)]
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
                    invoke!(("k", 0), "Reti", ty!("ContInt"), [bind!("z", 0)])
                )
            ),
            clause!(
                "Cons",
                [bind!("ys", 0, prd!(), ty!("List")), bind!("y", 0)],
                create!(
                    ("j", 0),
                    ty!("ContInt"),
                    [clause!(
                        "Reti",
                        [bind!("r", 0)],
                        sum!(
                            ("y", 0),
                            ("r", 0),
                            ("s", 0),
                            invoke!(("k", 0), "Reti", ty!("ContInt"), [bind!("s", 0)])
                        )
                    )],
                    call!(
                        "sum",
                        [
                            bind!("j", 0, cns!(), ty!("ContInt")),
                            bind!("ys", 0, prd!(), ty!("List")),
                        ]
                    )
                )
            ),
        ]
    );
    let sum = def!(
        "sum",
        [
            bind!("k", 0, cns!(), ty!("ContList")),
            bind!("xs", 0, prd!(), ty!("List")),
        ],
        sum_body,
        [
            ("ys", 0),
            ("xs", 0),
            ("y", 0),
            ("j", 0),
            ("s", 0),
            ("r", 0),
            ("k", 0),
            ("z", 0)
        ]
    );

    let program = prog!([main, range, sum], [ty_list, ty_cont_list, ty_cont_int]);

    println!("{}", program.linearize().print_to_string(None))
}
