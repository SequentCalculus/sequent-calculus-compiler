use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, prd, println_i64, prog, substitute, sum, switch, ty,
    ty_decl, xtor_sig,
};

pub fn non_linear_print() -> axcut::syntax::Prog {
    non_linear(println_i64!(id!("res"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
}

pub fn non_linear_exit() -> axcut::syntax::Prog {
    non_linear(exit!(id!("res")).into())
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
        id!("a", 2),
        ty!(id!("Box")),
        [clause!(
            id!("B"),
            [bind!(id!("x", 2))],
            substitute!(
                [
                    (bind!(id!("x", 2)), id!("x", 2)),
                    (bind!(id!("a", 1), prd!(), ty!(id!("Box"))), id!("a", 1)),
                ],
                switch!(
                    id!("a", 1),
                    ty!(id!("Box")),
                    [clause!(
                        id!("B"),
                        [bind!(id!("x", 1))],
                        sum!(id!("x", 1), id!("x", 2), id!("res"), exit_stmt)
                    )]
                )
            )
        )]
    );

    let main_body_switch = switch!(
        id!("bb", 1),
        ty!(id!("BoxBox")),
        [clause!(
            id!("BB"),
            [bind!(id!("b", 1), prd!(), ty!(id!("Box")))],
            switch!(
                id!("b", 1),
                ty!(id!("Box")),
                [clause!(
                    id!("B"),
                    [bind!(id!("x", 1))],
                    letin!(
                        id!("d", 1),
                        ty!(id!("Box")),
                        id!("B"),
                        [bind!(id!("x", 1))],
                        letin!(
                            id!("dd", 1),
                            ty!(id!("BoxBox")),
                            id!("BB"),
                            [bind!(id!("d", 1), prd!(), ty!(id!("Box")))],
                            substitute!(
                                [(
                                    bind!(id!("bb", 2), prd!(), ty!(id!("BoxBox"))),
                                    id!("bb", 2)
                                )],
                                lit!(
                                    4,
                                    id!("y"),
                                    letin!(
                                        id!("a", 1),
                                        ty!(id!("Box")),
                                        id!("B"),
                                        [bind!(id!("y"))],
                                        substitute!(
                                            [
                                                (
                                                    bind!(id!("a", 1), prd!(), ty!(id!("Box"))),
                                                    id!("a", 1)
                                                ),
                                                (
                                                    bind!(id!("bb", 2), prd!(), ty!(id!("BoxBox"))),
                                                    id!("bb", 2)
                                                ),
                                            ],
                                            switch!(
                                                id!("bb", 2),
                                                ty!(id!("BoxBox")),
                                                [clause!(
                                                    id!("BB"),
                                                    [bind!(id!("b", 2), prd!(), ty!(id!("Box")))],
                                                    switch!(
                                                        id!("b", 2),
                                                        ty!(id!("Box")),
                                                        [clause!(
                                                            id!("B"),
                                                            [bind!(id!("x", 2))],
                                                            letin!(
                                                                id!("a", 2),
                                                                ty!(id!("Box")),
                                                                id!("B"),
                                                                [bind!(id!("x", 2))],
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
                                    id!("x"),
                                    letin!(
                                        id!("b"),
                                        ty!(id!("Box")),
                                        id!("B"),
                                        [bind!(id!("x"))],
                                        letin!(
                                            id!("bb"),
                                            ty!(id!("BoxBox")),
                                            id!("BB"),
                                            [bind!(id!("b"), prd!(), ty!(id!("Box")))],
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
                                                            id!("bb", 3),
                                                            prd!(),
                                                            ty!(id!("BoxBox"))
                                                        ),
                                                        id!("bb")
                                                    ),
                                                    (
                                                        bind!(
                                                            id!("bb", 2),
                                                            prd!(),
                                                            ty!(id!("BoxBox"))
                                                        ),
                                                        id!("bb")
                                                    ),
                                                    (
                                                        bind!(
                                                            id!("bb", 1),
                                                            prd!(),
                                                            ty!(id!("BoxBox"))
                                                        ),
                                                        id!("bb")
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

    prog!([main], [ty_box, ty_box_box])
}
