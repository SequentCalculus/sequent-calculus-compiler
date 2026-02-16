use axcut_macros::{def, exit, id, lit, println_i64, prod, prog, sub, sum};

pub fn arith_print() -> axcut::syntax::Prog {
    arith(println_i64!(id!("i"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
}

pub fn arith_exit() -> axcut::syntax::Prog {
    arith(exit!(id!("i")).into())
}

pub fn arith(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let main_body = lit!(
        1,
        id!("a"),
        lit!(
            3,
            id!("b"),
            sub!(
                id!("a"),
                id!("b"),
                id!("c"),
                lit!(
                    8,
                    id!("d"),
                    lit!(
                        -1,
                        id!("e"),
                        prod!(
                            id!("e"),
                            id!("d"),
                            id!("f"),
                            sum!(
                                id!("f"),
                                id!("c"),
                                id!("g"),
                                lit!(-6, id!("h"), prod!(id!("h"), id!("g"), id!("i"), exit_stmt))
                            )
                        )
                    )
                )
            )
        )
    );
    let main = def!(id!("main"), [], main_body);

    prog!([main], [])
}
