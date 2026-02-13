use axcut_macros::{
    bind, clause, cns, create, def, exit, invoke, lit, prd, println_i64, prog, substitute, sum, ty,
    ty_decl, xtor_sig,
};

pub fn closure_print() -> axcut::syntax::Prog {
    closure(println_i64!("r", lit!(0, "ret", exit!("ret"))).into())
}

pub fn closure_exit() -> axcut::syntax::Prog {
    closure(exit!("r").into())
}

pub fn closure(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_cont = ty_decl!("Cont", [xtor_sig!("Ret", [bind!("r")])],);
    let ty_func = ty_decl!(
        "Fun",
        [xtor_sig!(
            "apply",
            [bind!("x"), bind!("k", cns!(), ty!("Cont"))]
        )],
    );

    let main_body = lit!(
        9,
        "a",
        create!(
            "f",
            ty!("Fun"),
            [bind!("a")],
            [clause!(
                "apply",
                [bind!("x"), bind!("k", cns!(), ty!("Cont"))],
                sum!(
                    "a",
                    "x",
                    "b",
                    substitute!(
                        [(bind!("b"), "b"), (bind!("k", cns!(), ty!("Cont")), "k")],
                        invoke!("k", "Ret", ty!("Cont"), []),
                    )
                )
            )],
            create!(
                "k",
                ty!("Cont"),
                [],
                [clause!("Ret", [bind!("r")], exit_stmt,),],
                lit!(
                    1,
                    "y",
                    substitute!(
                        [
                            (bind!("y"), "y"),
                            (bind!("k", cns!(), ty!("Cont")), "k"),
                            (bind!("f", prd!(), ty!("Fun")), "f"),
                        ],
                        invoke!("f", "apply", ty!("Fun"), [])
                    )
                ),
            ),
        )
    );
    let main = def!("main", [], main_body);

    prog!([main], [ty_cont, ty_func])
}
