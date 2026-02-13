use axcut_macros::{
    bind, clause, def, exit, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

pub fn either_print() -> axcut::syntax::Prog {
    either(println_i64!("c", lit!(0, "ret", exit!("ret"))).into())
}

pub fn either_exit() -> axcut::syntax::Prog {
    either(exit!("c").into())
}

pub fn either(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_either = ty_decl!(
        "Either",
        [
            xtor_sig!("Left", [bind!("x")]),
            xtor_sig!("Right", [bind!("y")])
        ]
    );

    let main_body = lit!(
        1,
        "z",
        lit!(
            9,
            "x",
            letin!(
                "p",
                ty!("Either"),
                "Right",
                [bind!("x")],
                switch!(
                    "p",
                    ty!("Either"),
                    [
                        clause!("Left", [bind!("a")], lit!(-1, "err", exit!("err"))),
                        clause!("Right", [bind!("b")], sum!("b", "z", "c", exit_stmt))
                    ]
                )
            ),
        ),
    );

    let main = def!("main", [], main_body);

    prog!([main], [ty_either])
}
