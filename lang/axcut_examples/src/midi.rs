use axcut_macros::{
    bind, call, clause, cns, create, def, exit, id, ife, invoke, letin, lit, prd, println_i64,
    prog, substitute, sum, switch, ty, ty_decl, xtor_sig,
};
pub fn midi_print() -> axcut::syntax::Prog {
    midi(println_i64!(id!("r"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
}

pub fn midi_exit() -> axcut::syntax::Prog {
    midi(exit!(id!("r")).into())
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
        id!("t"),
        ty!(id!("ContInt")),
        [],
        [clause!(id!("Reti"), [bind!(id!("r"))], exit_stmt)],
        create!(
            id!("k"),
            ty!(id!("ContList")),
            [bind!(id!("t"), cns!(), ty!(id!("ContInt")))],
            [clause!(
                id!("Retl"),
                [bind!(id!("as"), prd!(), ty!(id!("List")))],
                substitute!(
                    [
                        (bind!(id!("t"), cns!(), ty!(id!("ContInt"))), id!("t")),
                        (bind!(id!("as"), prd!(), ty!(id!("List"))), id!("as")),
                    ],
                    call!(id!("sum"), [])
                )
            )],
            letin!(
                id!("zs"),
                ty!(id!("List")),
                id!("Nil"),
                [],
                lit!(
                    3,
                    id!("n"),
                    substitute!(
                        [
                            (bind!(id!("k"), cns!(), ty!(id!("ContInt"))), id!("k")),
                            (bind!(id!("zs"), prd!(), ty!(id!("List"))), id!("zs")),
                            (bind!(id!("n")), id!("n")),
                        ],
                        call!(id!("range"), [])
                    )
                )
            )
        )
    );
    let main = def!(id!("main"), [], main_body);

    let range_body = ife!(
        id!("i"),
        substitute!(
            [
                (bind!(id!("xs"), prd!(), ty!(id!("List"))), id!("xs")),
                (bind!(id!("k"), cns!(), ty!(id!("ContList"))), id!("k"))
            ],
            invoke!(id!("k"), id!("Retl"), ty!(id!("ContList")), [])
        ),
        substitute!(
            [
                (bind!(id!("n")), id!("i")),
                (bind!(id!("k"), cns!(), ty!(id!("ContList"))), id!("k")),
                (bind!(id!("xs"), prd!(), ty!(id!("List"))), id!("xs")),
                (bind!(id!("i")), id!("i")),
            ],
            letin!(
                id!("ys"),
                ty!(id!("List")),
                id!("Cons"),
                [bind!(id!("xs"), prd!(), ty!(id!("List"))), bind!(id!("i"))],
                lit!(
                    -1,
                    id!("o"),
                    sum!(
                        id!("n"),
                        id!("o"),
                        id!("j"),
                        substitute!(
                            [
                                (bind!(id!("k"), cns!(), ty!(id!("ContList"))), id!("k")),
                                (bind!(id!("ys"), prd!(), ty!(id!("List"))), id!("ys")),
                                (bind!(id!("j")), id!("j"))
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
            bind!(id!("k"), cns!(), ty!(id!("ContList"))),
            bind!(id!("xs"), prd!(), ty!(id!("List"))),
            bind!(id!("i"))
        ],
        range_body
    );

    let sum_body = switch!(
        id!("xs"),
        ty!(id!("List")),
        [
            clause!(
                id!("Nil"),
                [],
                lit!(
                    0,
                    id!("z"),
                    substitute!(
                        [
                            (bind!(id!("z")), id!("z")),
                            (bind!(id!("k"), cns!(), ty!(id!("ContInt"))), id!("k"))
                        ],
                        invoke!(id!("k"), id!("Reti"), ty!(id!("ContInt")), [])
                    )
                )
            ),
            clause!(
                id!("Cons"),
                [bind!(id!("ys"), prd!(), ty!(id!("List"))), bind!(id!("y"))],
                substitute!(
                    [
                        (bind!(id!("ys"), prd!(), ty!(id!("List"))), id!("ys")),
                        (bind!(id!("k"), cns!(), ty!(id!("ContInt"))), id!("k")),
                        (bind!(id!("y")), id!("y"))
                    ],
                    create!(
                        id!("j"),
                        ty!(id!("ContInt")),
                        [
                            bind!(id!("k"), cns!(), ty!(id!("ContInt"))),
                            bind!(id!("y"))
                        ],
                        [clause!(
                            id!("Reti"),
                            [bind!(id!("r"))],
                            sum!(
                                id!("y"),
                                id!("r"),
                                id!("s"),
                                substitute!(
                                    [
                                        (bind!(id!("s")), id!("s")),
                                        (bind!(id!("k"), cns!(), ty!(id!("ContInt"))), id!("k"))
                                    ],
                                    invoke!(id!("k"), id!("Reti"), ty!(id!("ContInt")), [])
                                )
                            )
                        )],
                        substitute!(
                            [
                                (bind!(id!("j"), cns!(), ty!(id!("ContInt"))), id!("j")),
                                (bind!(id!("ys"), prd!(), ty!(id!("List"))), id!("ys"))
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
            bind!(id!("k"), cns!(), ty!(id!("ContList"))),
            bind!(id!("xs"), prd!(), ty!(id!("List")))
        ],
        sum_body
    );

    prog!([main, range, sum], [ty_list, ty_cont_list, ty_cont_int])
}
