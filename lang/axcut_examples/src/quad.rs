use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

pub fn quad_exit() -> axcut::syntax::Prog {
    quad(exit!(id!("e")).into())
}

pub fn quad_print() -> axcut::syntax::Prog {
    quad(println_i64!(id!("e"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
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
        id!("z"),
        lit!(
            6,
            id!("y"),
            lit!(
                4,
                id!("x"),
                lit!(
                    2,
                    id!("w"),
                    letin!(
                        id!("q"),
                        ty!(id!("Quad")),
                        id!("Q"),
                        [
                            bind!(id!("z")),
                            bind!(id!("y")),
                            bind!(id!("x")),
                            bind!(id!("w"))
                        ],
                        switch!(
                            id!("q"),
                            ty!(id!("Quad")),
                            [clause!(
                                id!("Q"),
                                [
                                    bind!(id!("d")),
                                    bind!(id!("c")),
                                    bind!(id!("b")),
                                    bind!(id!("a"))
                                ],
                                lit!(7, id!("z"), sum!(id!("d"), id!("z"), id!("e"), exit_stmt))
                            )]
                        )
                    )
                )
            )
        )
    );
    let main = def!(id!("main"), [], main_body);
    prog!([main], [ty_quad])
}
