use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

pub fn either_print() -> axcut::syntax::Prog {
    either(println_i64!(id!("c", 7), lit!(0, id!("ret", 8), exit!(id!("ret", 8)))).into())
}

pub fn either_exit() -> axcut::syntax::Prog {
    either(exit!(id!("c", 7)).into())
}

pub fn either(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_either = ty_decl!(
        id!("Either"),
        [
            xtor_sig!(id!("Left"), [bind!(id!("x"))]),
            xtor_sig!(id!("Right"), [bind!(id!("y"))])
        ]
    );

    let main_body = lit!(
        1,
        id!("z", 1),
        lit!(
            9,
            id!("x", 2),
            letin!(
                id!("p", 3),
                ty!(id!("Either")),
                id!("Right"),
                [bind!(id!("x", 2))],
                switch!(
                    id!("p", 3),
                    ty!(id!("Either")),
                    [
                        clause!(
                            id!("Left"),
                            [bind!(id!("a", 4))],
                            lit!(-1, id!("err", 5), exit!(id!("err", 5)))
                        ),
                        clause!(
                            id!("Right"),
                            [bind!(id!("b", 6))],
                            sum!(id!("b", 6), id!("z", 1), id!("c", 7), exit_stmt)
                        )
                    ]
                )
            ),
        ),
    );

    let main = def!(id!("main"), [], main_body);

    prog!([main], [ty_either], 8)
}
