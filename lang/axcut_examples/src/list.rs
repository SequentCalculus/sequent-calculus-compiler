use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, prd, println_i64, prog, switch, ty, ty_decl, xtor_sig,
};

pub fn list_print() -> axcut::syntax::Prog {
    list(println_i64!(id!("a", 10), lit!(0, id!("ret", 11), exit!(id!("ret", 11)))).into())
}

pub fn list_exit() -> axcut::syntax::Prog {
    list(exit!(id!("a", 10)).into())
}

pub fn list(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_list = ty_decl!(
        id!("List"),
        [
            xtor_sig!(id!("Nil"), []),
            xtor_sig!(
                id!("Cons"),
                [bind!(id!("xs"), prd!(), ty!(id!("List"))), bind!(id!("x"))]
            ),
        ],
    );

    let main_body = letin!(
        id!("ws", 1),
        ty!(id!("List")),
        id!("Nil"),
        [],
        lit!(
            5,
            id!("z", 2),
            letin!(
                id!("zs", 3),
                ty!(id!("List")),
                id!("Cons"),
                [
                    bind!(id!("z", 2)),
                    bind!(id!("ws", 1), prd!(), ty!(id!("List")))
                ],
                lit!(
                    7,
                    id!("y", 4),
                    letin!(
                        id!("ys", 5),
                        ty!(id!("List")),
                        id!("Cons"),
                        [
                            bind!(id!("y", 4)),
                            bind!(id!("zs", 3), prd!(), ty!(id!("List")))
                        ],
                        lit!(
                            9,
                            id!("x", 6),
                            letin!(
                                id!("xs", 7),
                                ty!(id!("List")),
                                id!("Cons"),
                                [
                                    bind!(id!("x", 6)),
                                    bind!(id!("ys", 5), prd!(), ty!(id!("List")))
                                ],
                                switch!(
                                    id!("xs", 7),
                                    ty!(id!("List")),
                                    [
                                        clause!(
                                            id!("Nil"),
                                            [],
                                            lit!(-1, id!("err", 8), exit!(id!("err", 8)))
                                        ),
                                        clause!(
                                            id!("Cons"),
                                            [
                                                bind!(id!("as", 9), prd!(), ty!(id!("List"))),
                                                bind!(id!("a", 10))
                                            ],
                                            exit_stmt
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
    let main = def!(id!("main"), [], main_body);

    prog!([main], [ty_list], 11)
}
