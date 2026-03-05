use axcut_macros::{
    bind, clause, cns, create, def, exit, id, invoke, lit, prd, println_i64, prog, substitute, sum,
    ty, ty_decl, xtor_sig,
};

pub fn closure_print() -> axcut::syntax::Prog {
    closure(println_i64!(id!("r", 7), lit!(0, id!("ret", 8), exit!(id!("ret", 8)))).into())
}

pub fn closure_exit() -> axcut::syntax::Prog {
    closure(exit!(id!("r", 7)).into())
}

pub fn closure(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_cont = ty_decl!(id!("Cont"), [xtor_sig!(id!("Ret"), [bind!(id!("r"))])],);
    let ty_func = ty_decl!(
        id!("Fun"),
        [xtor_sig!(
            id!("apply"),
            [bind!(id!("x")), bind!(id!("k"), cns!(), ty!(id!("Cont")))]
        )],
    );

    let main_body = lit!(
        9,
        id!("a", 1),
        create!(
            id!("f", 2),
            ty!(id!("Fun")),
            [bind!(id!("a", 1))],
            [clause!(
                id!("apply"),
                [
                    bind!(id!("x", 3)),
                    bind!(id!("k", 4), cns!(), ty!(id!("Cont")))
                ],
                sum!(
                    id!("a", 1),
                    id!("x", 3),
                    id!("b", 5),
                    substitute!(
                        [
                            (bind!(id!("b", 5)), id!("b", 5)),
                            (bind!(id!("k", 4), cns!(), ty!(id!("Cont"))), id!("k", 4))
                        ],
                        invoke!(id!("k", 4), id!("Ret"), ty!(id!("Cont")), []),
                    )
                )
            )],
            create!(
                id!("k", 6),
                ty!(id!("Cont")),
                [],
                [clause!(id!("Ret"), [bind!(id!("r", 7))], exit_stmt,),],
                lit!(
                    1,
                    id!("y", 9),
                    substitute!(
                        [
                            (bind!(id!("y", 9)), id!("y", 9)),
                            (bind!(id!("k", 6), cns!(), ty!(id!("Cont"))), id!("k", 6)),
                            (bind!(id!("f", 2), prd!(), ty!(id!("Fun"))), id!("f", 2)),
                        ],
                        invoke!(id!("f", 2), id!("apply"), ty!(id!("Fun")), [])
                    )
                ),
            ),
        )
    );
    let main = def!(id!("main"), [], main_body);

    prog!([main], [ty_cont, ty_func], 9)
}
