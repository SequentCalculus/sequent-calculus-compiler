use axcut_macros::{def, exit, lit, println_i64, prod, prog, sub, sum};

pub fn arith_print() -> axcut::syntax::Prog {
    arith(println_i64!("i", lit!(0, "ret", exit!("ret"))).into())
}

pub fn arith_exit() -> axcut::syntax::Prog {
    arith(exit!("i").into())
}

pub fn arith(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let main_body = lit!(
        1,
        "a",
        lit!(
            3,
            "b",
            sub!(
                "a",
                "b",
                "c",
                lit!(
                    8,
                    "d",
                    lit!(
                        -1,
                        "e",
                        prod!(
                            "e",
                            "d",
                            "f",
                            sum!(
                                "f",
                                "c",
                                "g",
                                lit!(-6, "h", prod!("h", "g", "i", exit_stmt))
                            )
                        )
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    prog!([main], [])
}
