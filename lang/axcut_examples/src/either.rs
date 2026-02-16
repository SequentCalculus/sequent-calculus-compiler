use axcut_macros::{
    bind, clause, def, exit, id, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

pub fn either_print() -> axcut::syntax::Prog {
    either(println_i64!(id!("c"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
}

pub fn either_exit() -> axcut::syntax::Prog {
    either(exit!(id!("c")).into())
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
        id!("z"),
        lit!(
            9,
            id!("x"),
            letin!(
                id!("p"),
                ty!(id!("Either")),
                id!("Right"),
                [bind!(id!("x"))],
                switch!(
                    id!("p"),
                    ty!(id!("Either")),
                    [
                        clause!(
                            id!("Left"),
                            [bind!(id!("a"))],
                            lit!(-1, id!("err"), exit!(id!("err")))
                        ),
                        clause!(
                            id!("Right"),
                            [bind!(id!("b"))],
                            sum!(id!("b"), id!("z"), id!("c"), exit_stmt)
                        )
                    ]
                )
            ),
        ),
    );

    let main = def!(id!("main"), [], main_body);

    prog!([main], [ty_either])
}
