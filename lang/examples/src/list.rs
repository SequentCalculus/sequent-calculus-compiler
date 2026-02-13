use axcut_macros::{
    bind, clause, def, exit, letin, lit, prd, println_i64, prog, switch, ty, ty_decl, xtor_sig,
};

pub fn list_print() -> axcut::syntax::Prog {
    list(println_i64!("a", lit!(0, "ret", exit!("ret"))).into())
}

pub fn list_exit() -> axcut::syntax::Prog {
    list(exit!("a").into())
}

pub fn list(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_list = ty_decl!(
        "List",
        [
            xtor_sig!("Nil", []),
            xtor_sig!("Cons", [bind!("xs", prd!(), ty!("List")), bind!("x")]),
        ],
    );

    let main_body = letin!(
        "ws",
        ty!("List"),
        "Nil",
        [],
        lit!(
            5,
            "z",
            letin!(
                "zs",
                ty!("List"),
                "Cons",
                [bind!("z"), bind!("ws", prd!(), ty!("List"))],
                lit!(
                    7,
                    "y",
                    letin!(
                        "ys",
                        ty!("List"),
                        "Cons",
                        [bind!("y"), bind!("zs", prd!(), ty!("List"))],
                        lit!(
                            9,
                            "x",
                            letin!(
                                "xs",
                                ty!("List"),
                                "Cons",
                                [bind!("x"), bind!("ys", prd!(), ty!("List"))],
                                switch!(
                                    "xs",
                                    ty!("List"),
                                    [
                                        clause!("Nil", [], lit!(-1, "err", exit!("err"))),
                                        clause!(
                                            "Cons",
                                            [bind!("as", prd!(), ty!("List")), bind!("a"),],
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
    let main = def!("main", [], main_body);

    prog!([main], [ty_list])
}
