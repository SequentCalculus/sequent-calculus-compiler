use axcut_macros::{
    bind, call, clause, cns, create, def, exit, id, ife, invoke, letin, lit, prd, println_i64,
    prog, substitute, sum, switch, ty, ty_decl, xtor_sig,
};
pub fn midi_print() -> axcut::syntax::Prog {
    midi(println_i64!(id!("r", 2), lit!(0, id!("ret", 3), exit!(id!("ret", 3)))).into())
}

pub fn midi_exit() -> axcut::syntax::Prog {
    midi(exit!(id!("r", 2)).into())
}

pub fn midi(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_list = ty_decl!(
        id!("List"),
        [
            xtor_sig!(id!("Nil"), []),
            xtor_sig!(
                id!("Cons"),
                [bind!(id!("xs"), prd!(), ty!(id!("List"))), bind!(id!("x"))]
            )
        ]
    );

    let ty_cont_list = ty_decl!(
        id!("ContList"),
        [xtor_sig!(
            id!("Retl"),
            [bind!(id!("kl"), prd!(), ty!(id!("List")))]
        )]
    );

    let ty_cont_int = ty_decl!(id!("ContInt"), [xtor_sig!(id!("Reti"), [bind!(id!("ki"))])]);

    let main_body = create!(
        id!("t", 1),
        ty!(id!("ContInt")),
        [],
        [clause!(id!("Reti"), [bind!(id!("r", 2))], exit_stmt)],
        create!(
            id!("k", 4),
            ty!(id!("ContList")),
            [bind!(id!("t", 1), cns!(), ty!(id!("ContInt")))],
            [clause!(
                id!("Retl"),
                [bind!(id!("as", 5), prd!(), ty!(id!("List")))],
                substitute!(
                    [
                        (bind!(id!("t", 1), cns!(), ty!(id!("ContInt"))), id!("t", 1)),
                        (bind!(id!("as", 5), prd!(), ty!(id!("List"))), id!("as", 5)),
                    ],
                    call!(id!("sum"), [])
                )
            )],
            letin!(
                id!("zs", 6),
                ty!(id!("List")),
                id!("Nil"),
                [],
                lit!(
                    3,
                    id!("n", 7),
                    substitute!(
                        [
                            (bind!(id!("k", 4), cns!(), ty!(id!("ContInt"))), id!("k", 4)),
                            (bind!(id!("zs", 6), prd!(), ty!(id!("List"))), id!("zs", 6)),
                            (bind!(id!("n", 7)), id!("n", 7)),
                        ],
                        call!(id!("range"), [])
                    )
                )
            )
        )
    );
    let main = def!(id!("main"), [], main_body);

    let range_body = ife!(
        id!("i", 10),
        substitute!(
            [
                (bind!(id!("xs", 9), prd!(), ty!(id!("List"))), id!("xs", 9)),
                (
                    bind!(id!("k", 8), cns!(), ty!(id!("ContList"))),
                    id!("k", 8)
                )
            ],
            invoke!(id!("k", 8), id!("Retl"), ty!(id!("ContList")), [])
        ),
        substitute!(
            [
                (bind!(id!("n", 11)), id!("i", 10)),
                (
                    bind!(id!("k", 8), cns!(), ty!(id!("ContList"))),
                    id!("k", 8)
                ),
                (bind!(id!("xs", 9), prd!(), ty!(id!("List"))), id!("xs", 9)),
                (bind!(id!("i", 10)), id!("i", 10)),
            ],
            letin!(
                id!("ys", 12),
                ty!(id!("List")),
                id!("Cons"),
                [
                    bind!(id!("xs", 9), prd!(), ty!(id!("List"))),
                    bind!(id!("i", 10))
                ],
                lit!(
                    -1,
                    id!("o", 13),
                    sum!(
                        id!("n", 11),
                        id!("o", 13),
                        id!("j", 14),
                        substitute!(
                            [
                                (
                                    bind!(id!("k", 8), cns!(), ty!(id!("ContList"))),
                                    id!("k", 8)
                                ),
                                (
                                    bind!(id!("ys", 12), prd!(), ty!(id!("List"))),
                                    id!("ys", 12)
                                ),
                                (bind!(id!("j", 14)), id!("j", 14))
                            ],
                            call!(id!("range"), [])
                        )
                    )
                )
            )
        )
    );
    let range = def!(
        id!("range"),
        [
            bind!(id!("k", 8), cns!(), ty!(id!("ContList"))),
            bind!(id!("xs", 9), prd!(), ty!(id!("List"))),
            bind!(id!("i", 10))
        ],
        range_body
    );

    let sum_body = switch!(
        id!("xs", 16),
        ty!(id!("List")),
        [
            clause!(
                id!("Nil"),
                [],
                lit!(
                    0,
                    id!("z", 17),
                    substitute!(
                        [
                            (bind!(id!("z", 17)), id!("z", 17)),
                            (
                                bind!(id!("k", 15), cns!(), ty!(id!("ContInt"))),
                                id!("k", 15)
                            )
                        ],
                        invoke!(id!("k", 15), id!("Reti"), ty!(id!("ContInt")), [])
                    )
                )
            ),
            clause!(
                id!("Cons"),
                [
                    bind!(id!("ys", 18), prd!(), ty!(id!("List"))),
                    bind!(id!("y", 19))
                ],
                substitute!(
                    [
                        (
                            bind!(id!("ys", 18), prd!(), ty!(id!("List"))),
                            id!("ys", 18)
                        ),
                        (
                            bind!(id!("k", 15), cns!(), ty!(id!("ContInt"))),
                            id!("k", 15)
                        ),
                        (bind!(id!("y", 19)), id!("y", 19))
                    ],
                    create!(
                        id!("j", 20),
                        ty!(id!("ContInt")),
                        [
                            bind!(id!("k", 15), cns!(), ty!(id!("ContInt"))),
                            bind!(id!("y", 19))
                        ],
                        [clause!(
                            id!("Reti"),
                            [bind!(id!("r", 21))],
                            sum!(
                                id!("y", 19),
                                id!("r", 21),
                                id!("s", 22),
                                substitute!(
                                    [
                                        (bind!(id!("s", 22)), id!("s", 22)),
                                        (
                                            bind!(id!("k", 15), cns!(), ty!(id!("ContInt"))),
                                            id!("k", 15)
                                        )
                                    ],
                                    invoke!(id!("k", 15), id!("Reti"), ty!(id!("ContInt")), [])
                                )
                            )
                        )],
                        substitute!(
                            [
                                (
                                    bind!(id!("j", 20), cns!(), ty!(id!("ContInt"))),
                                    id!("j", 20)
                                ),
                                (
                                    bind!(id!("ys", 18), prd!(), ty!(id!("List"))),
                                    id!("ys", 18)
                                )
                            ],
                            call!(id!("sum"), [])
                        )
                    )
                )
            )
        ]
    );
    let sum = def!(
        id!("sum"),
        [
            bind!(id!("k", 15), cns!(), ty!(id!("ContList"))),
            bind!(id!("xs", 16), prd!(), ty!(id!("List")))
        ],
        sum_body
    );

    prog!([main, range, sum], [ty_list, ty_cont_list, ty_cont_int], 22)
}
