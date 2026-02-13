use axcut_macros::{
    bind, clause, def, exit, letin, lit, prd, println_i64, prog, substitute, sum, switch, ty,
    ty_decl, xtor_sig,
};

pub fn non_linear_print() -> axcut::syntax::Prog {
    non_linear(println_i64!("res", lit!(0, "ret", exit!("ret"))).into())
}

pub fn non_linear_exit() -> axcut::syntax::Prog {
    non_linear(exit!("res").into())
}

pub fn non_linear(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_box = ty_decl!("Box", [xtor_sig!("B", [bind!("b")])]);
    let ty_box_box = ty_decl!(
        "BoxBox",
        [xtor_sig!("BB", [bind!("bb", prd!(), ty!("Box"))])]
    );

    let main_body_switch_switch = switch!(
        "a2",
        ty!("Box"),
        [clause!(
            "B",
            [bind!("x2")],
            substitute!(
                [(bind!("x2"), "x2"), (bind!("a1", prd!(), ty!("Box")), "a1"),],
                switch!(
                    "a1",
                    ty!("Box"),
                    [clause!(
                        "B",
                        [bind!("x1")],
                        sum!("x1", "x2", "res", exit_stmt)
                    )]
                )
            )
        )]
    );

    let main_body_switch = switch!(
        "bb1",
        ty!("BoxBox"),
        [clause!(
            "BB",
            [bind!("b1", prd!(), ty!("Box"))],
            switch!(
                "b1",
                ty!("Box"),
                [clause!(
                    "B",
                    [bind!("x1")],
                    letin!(
                        "d1",
                        ty!("Box"),
                        "B",
                        [bind!("x1")],
                        letin!(
                            "dd1",
                            ty!("BoxBox"),
                            "BB",
                            [bind!("d1", prd!(), ty!("Box"))],
                            substitute!(
                                [(bind!("bb2", prd!(), ty!("BoxBox")), "bb2",)],
                                lit!(
                                    4,
                                    "y",
                                    letin!(
                                        "a1",
                                        ty!("Box"),
                                        "B",
                                        [bind!("y")],
                                        substitute!(
                                            [
                                                (bind!("a1", prd!(), ty!("Box")), "a1"),
                                                (bind!("bb2", prd!(), ty!("BoxBox")), "bb2"),
                                            ],
                                            switch!(
                                                "bb2",
                                                ty!("BoxBox"),
                                                [clause!(
                                                    "BB",
                                                    [bind!("b2", prd!(), ty!("Box"))],
                                                    switch!(
                                                        "b2",
                                                        ty!("Box"),
                                                        [clause!(
                                                            "B",
                                                            [bind!("x2")],
                                                            letin!(
                                                                "a2",
                                                                ty!("Box"),
                                                                "B",
                                                                [bind!("x2")],
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
        "f1",
        lit!(
            3,
            "f2",
            lit!(
                3,
                "f3",
                lit!(
                    3,
                    "f4",
                    lit!(
                        3,
                        "f5",
                        lit!(
                            3,
                            "f6",
                            lit!(
                                3,
                                "f7",
                                lit!(
                                    3,
                                    "x",
                                    letin!(
                                        "b",
                                        ty!("Box"),
                                        "B",
                                        [bind!("x")],
                                        letin!(
                                            "bb",
                                            ty!("BoxBox"),
                                            "BB",
                                            [bind!("b", prd!(), ty!("Box"))],
                                            substitute!(
                                                [
                                                    (bind!("f1"), "f1"),
                                                    (bind!("f2"), "f2"),
                                                    (bind!("f3"), "f3"),
                                                    (bind!("f5"), "f5"),
                                                    (bind!("f6"), "f6"),
                                                    (bind!("f7"), "f7"),
                                                    (bind!("f4"), "f4"),
                                                    (bind!("bb3", prd!(), ty!("BoxBox")), "bb",),
                                                    (bind!("bb2", prd!(), ty!("BoxBox")), "bb",),
                                                    (bind!("bb1", prd!(), ty!("BoxBox")), "bb",),
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

    prog!([main], [ty_box, ty_box_box])
}
