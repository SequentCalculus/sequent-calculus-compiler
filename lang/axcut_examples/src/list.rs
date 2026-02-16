use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, prd, println_i64, prog, switch, ty, ty_decl, xtor_sig,
};

pub fn list_print() -> axcut::syntax::Prog {
    list(println_i64!(id!("a"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
}

pub fn list_exit() -> axcut::syntax::Prog {
    list(exit!(id!("a")).into())
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
        id!("ws"),
        ty!(id!("List")),
        id!("Nil"),
        [],
        lit!(
            5,
            id!("z"),
            letin!(
                id!("zs"),
                ty!(id!("List")),
                id!("Cons"),
                [bind!(id!("z")), bind!(id!("ws"), prd!(), ty!(id!("List")))],
                lit!(
                    7,
                    id!("y"),
                    letin!(
                        id!("ys"),
                        ty!(id!("List")),
                        id!("Cons"),
                        [bind!(id!("y")), bind!(id!("zs"), prd!(), ty!(id!("List")))],
                        lit!(
                            9,
                            id!("x"),
                            letin!(
                                id!("xs"),
                                ty!(id!("List")),
                                id!("Cons"),
                                [bind!(id!("x")), bind!(id!("ys"), prd!(), ty!(id!("List")))],
                                switch!(
                                    id!("xs"),
                                    ty!(id!("List")),
                                    [
                                        clause!(
                                            id!("Nil"),
                                            [],
                                            lit!(-1, id!("err"), exit!(id!("err")))
                                        ),
                                        clause!(
                                            id!("Cons"),
                                            [
                                                bind!(id!("as"), prd!(), ty!(id!("List"))),
                                                bind!(id!("a"))
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

    prog!([main], [ty_list])
}
