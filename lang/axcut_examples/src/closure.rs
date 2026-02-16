use axcut_macros::{
    bind, clause, cns, create, def, exit, id, invoke, lit, prd, println_i64, prog, substitute, sum,
    ty, ty_decl, xtor_sig,
};

pub fn closure_print() -> axcut::syntax::Prog {
    closure(println_i64!(id!("r"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
}

pub fn closure_exit() -> axcut::syntax::Prog {
    closure(exit!(id!("r")).into())
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
        id!("a"),
        create!(
            id!("f"),
            ty!(id!("Fun")),
            [bind!(id!("a"))],
            [clause!(
                id!("apply"),
                [bind!(id!("x")), bind!(id!("k"), cns!(), ty!(id!("Cont")))],
                sum!(
                    id!("a"),
                    id!("x"),
                    id!("b"),
                    substitute!(
                        [
                            (bind!(id!("b")), id!("b")),
                            (bind!(id!("k"), cns!(), ty!(id!("Cont"))), id!("k"))
                        ],
                        invoke!(id!("k"), id!("Ret"), ty!(id!("Cont")), []),
                    )
                )
            )],
            create!(
                id!("k"),
                ty!(id!("Cont")),
                [],
                [clause!(id!("Ret"), [bind!(id!("r"))], exit_stmt,),],
                lit!(
                    1,
                    id!("y"),
                    substitute!(
                        [
                            (bind!(id!("y")), id!("y")),
                            (bind!(id!("k"), cns!(), ty!(id!("Cont"))), id!("k")),
                            (bind!(id!("f"), prd!(), ty!(id!("Fun"))), id!("f")),
                        ],
                        invoke!(id!("f"), id!("apply"), ty!(id!("Fun")), [])
                    )
                ),
            ),
        )
    );
    let main = def!(id!("main"), [], main_body);

    prog!([main], [ty_cont, ty_func])
}
