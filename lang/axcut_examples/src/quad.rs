use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

pub fn quad_print() -> axcut::syntax::Prog {
    quad(println_i64!(id!("e", 11), lit!(0, id!("ret", 12), exit!(id!("ret", 12)))).into())
}

pub fn quad_exit() -> axcut::syntax::Prog {
    quad(exit!(id!("e", 11)).into())
}

pub fn quad(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_quad = ty_decl!(
        id!("Quad"),
        [xtor_sig!(
            id!("Q"),
            [
                bind!(id!("d")),
                bind!(id!("c")),
                bind!(id!("b")),
                bind!(id!("a"))
            ]
        )]
    );

    let main_body = lit!(
        8,
        id!("z", 1),
        lit!(
            6,
            id!("y", 2),
            lit!(
                4,
                id!("x", 3),
                lit!(
                    2,
                    id!("w", 4),
                    letin!(
                        id!("q", 5),
                        ty!(id!("Quad")),
                        id!("Q"),
                        [
                            bind!(id!("z", 1)),
                            bind!(id!("y", 2)),
                            bind!(id!("x", 3)),
                            bind!(id!("w", 4))
                        ],
                        switch!(
                            id!("q", 5),
                            ty!(id!("Quad")),
                            [clause!(
                                id!("Q"),
                                [
                                    bind!(id!("d", 6)),
                                    bind!(id!("c", 7)),
                                    bind!(id!("b", 8)),
                                    bind!(id!("a", 9))
                                ],
                                lit!(
                                    7,
                                    id!("z", 10),
                                    sum!(id!("d", 6), id!("z", 10), id!("e", 11), exit_stmt)
                                )
                            )]
                        )
                    )
                )
            )
        )
    );
    let main = def!(id!("main"), [], main_body);

    prog!([main], [ty_quad], 12)
}
