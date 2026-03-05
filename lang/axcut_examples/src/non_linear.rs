use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, prd, println_i64, prog, substitute, sum, switch, ty,
    ty_decl, xtor_sig,
};

pub fn non_linear_print() -> axcut::syntax::Prog {
    non_linear(
        println_i64!(
            id!("res", 25),
            lit!(0, id!("ret", 26), exit!(id!("ret", 26)))
        )
        .into(),
    )
}

pub fn non_linear_exit() -> axcut::syntax::Prog {
    non_linear(exit!(id!("res", 25)).into())
}

pub fn non_linear(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_box = ty_decl!(id!("Box"), [xtor_sig!(id!("B"), [bind!(id!("b"))])]);
    let ty_box_box = ty_decl!(
        id!("BoxBox"),
        [xtor_sig!(
            id!("BB"),
            [bind!(id!("bb"), prd!(), ty!(id!("Box")))]
        )]
    );

    let main_body_switch_switch = switch!(
        id!("a", 22),
        ty!(id!("Box")),
        [clause!(
            id!("B"),
            [bind!(id!("x", 23))],
            substitute!(
                [
                    (bind!(id!("x", 23)), id!("x", 23)),
                    (bind!(id!("a", 19), prd!(), ty!(id!("Box"))), id!("a", 19)),
                ],
                switch!(
                    id!("a", 19),
                    ty!(id!("Box")),
                    [clause!(
                        id!("B"),
                        [bind!(id!("x", 24))],
                        sum!(id!("x", 24), id!("x", 23), id!("res", 25), exit_stmt)
                    )]
                )
            )
        )]
    );

    let main_body_switch = switch!(
        id!("bb", 11),
        ty!(id!("BoxBox")),
        [clause!(
            id!("BB"),
            [bind!(id!("b", 14), prd!(), ty!(id!("Box")))],
            switch!(
                id!("b", 14),
                ty!(id!("Box")),
                [clause!(
                    id!("B"),
                    [bind!(id!("x", 15))],
                    letin!(
                        id!("d", 16),
                        ty!(id!("Box")),
                        id!("B"),
                        [bind!(id!("x", 15))],
                        letin!(
                            id!("dd", 17),
                            ty!(id!("BoxBox")),
                            id!("BB"),
                            [bind!(id!("d", 16), prd!(), ty!(id!("Box")))],
                            substitute!(
                                [(
                                    bind!(id!("bb", 12), prd!(), ty!(id!("BoxBox"))),
                                    id!("bb", 12)
                                )],
                                lit!(
                                    4,
                                    id!("y", 18),
                                    letin!(
                                        id!("a", 19),
                                        ty!(id!("Box")),
                                        id!("B"),
                                        [bind!(id!("y", 18))],
                                        substitute!(
                                            [
                                                (
                                                    bind!(id!("a", 19), prd!(), ty!(id!("Box"))),
                                                    id!("a", 19)
                                                ),
                                                (
                                                    bind!(
                                                        id!("bb", 12),
                                                        prd!(),
                                                        ty!(id!("BoxBox"))
                                                    ),
                                                    id!("bb", 12)
                                                ),
                                            ],
                                            switch!(
                                                id!("bb", 12),
                                                ty!(id!("BoxBox")),
                                                [clause!(
                                                    id!("BB"),
                                                    [bind!(id!("b", 20), prd!(), ty!(id!("Box")))],
                                                    switch!(
                                                        id!("b", 20),
                                                        ty!(id!("Box")),
                                                        [clause!(
                                                            id!("B"),
                                                            [bind!(id!("x", 21))],
                                                            letin!(
                                                                id!("a", 22),
                                                                ty!(id!("Box")),
                                                                id!("B"),
                                                                [bind!(id!("x", 21))],
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
        id!("f", 1),
        lit!(
            3,
            id!("f", 2),
            lit!(
                3,
                id!("f", 3),
                lit!(
                    3,
                    id!("f", 4),
                    lit!(
                        3,
                        id!("f", 5),
                        lit!(
                            3,
                            id!("f", 6),
                            lit!(
                                3,
                                id!("f", 7),
                                lit!(
                                    3,
                                    id!("x", 8),
                                    letin!(
                                        id!("b", 9),
                                        ty!(id!("Box")),
                                        id!("B"),
                                        [bind!(id!("x", 8))],
                                        letin!(
                                            id!("bb", 10),
                                            ty!(id!("BoxBox")),
                                            id!("BB"),
                                            [bind!(id!("b", 9), prd!(), ty!(id!("Box")))],
                                            substitute!(
                                                [
                                                    (bind!(id!("f", 1)), id!("f", 1)),
                                                    (bind!(id!("f", 2)), id!("f", 2)),
                                                    (bind!(id!("f", 3)), id!("f", 3)),
                                                    (bind!(id!("f", 5)), id!("f", 5)),
                                                    (bind!(id!("f", 6)), id!("f", 6)),
                                                    (bind!(id!("f", 7)), id!("f", 7)),
                                                    (bind!(id!("f", 4)), id!("f", 4)),
                                                    (
                                                        bind!(
                                                            id!("bb", 13),
                                                            prd!(),
                                                            ty!(id!("BoxBox"))
                                                        ),
                                                        id!("bb", 10)
                                                    ),
                                                    (
                                                        bind!(
                                                            id!("bb", 12),
                                                            prd!(),
                                                            ty!(id!("BoxBox"))
                                                        ),
                                                        id!("bb", 10)
                                                    ),
                                                    (
                                                        bind!(
                                                            id!("bb", 11),
                                                            prd!(),
                                                            ty!(id!("BoxBox"))
                                                        ),
                                                        id!("bb", 10)
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
    let main = def!(id!("main"), [], main_body);

    prog!([main], [ty_box, ty_box_box], 26)
}
