use axcut_macros::{
    bind, clause, def, exit, letin, lit, println_i64, prog, sum, switch, ty, ty_decl, xtor_sig,
};

pub fn quad_exit() -> axcut::syntax::Prog {
    quad(exit!("e").into())
}

pub fn quad_print() -> axcut::syntax::Prog {
    quad(println_i64!("e", lit!(0, "ret", exit!("ret"))).into())
}

pub fn quad(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let ty_quad = ty_decl!(
        "Quad",
        [xtor_sig!(
            "Q",
            [bind!("d"), bind!("c"), bind!("b"), bind!("a")]
        )]
    );

    let main_body = lit!(
        8,
        "z",
        lit!(
            6,
            "y",
            lit!(
                4,
                "x",
                lit!(
                    2,
                    "w",
                    letin!(
                        "q",
                        ty!("Quad"),
                        "Q",
                        [bind!("z"), bind!("y"), bind!("x"), bind!("w")],
                        switch!(
                            "q",
                            ty!("Quad"),
                            [clause!(
                                "Q",
                                [bind!("d"), bind!("c"), bind!("b"), bind!("a")],
                                lit!(7, "z", sum!("d", "z", "e", exit_stmt))
                            )]
                        )
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);
    prog!([main], [ty_quad])
}
