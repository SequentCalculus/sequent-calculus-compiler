use axcut_macros::{def, exit, id, lit, println_i64, prod, prog, sub, sum};

pub fn arith_print() -> axcut::syntax::Prog {
    arith(println_i64!(id!("i", 9), lit!(0, id!("ret", 10), exit!(id!("ret", 10)))).into())
}

pub fn arith_exit() -> axcut::syntax::Prog {
    arith(exit!(id!("i", 9)).into())
}

pub fn arith(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let main_body = lit!(
        1,
        id!("a", 1),
        lit!(
            3,
            id!("b", 2),
            sub!(
                id!("a", 1),
                id!("b", 2),
                id!("c", 3),
                lit!(
                    8,
                    id!("d", 4),
                    lit!(
                        -1,
                        id!("e", 5),
                        prod!(
                            id!("e", 5),
                            id!("d", 4),
                            id!("f", 6),
                            sum!(
                                id!("f", 6),
                                id!("c", 3),
                                id!("g", 7),
                                lit!(
                                    -6,
                                    id!("h", 8),
                                    prod!(id!("h", 8), id!("g", 7), id!("i", 9), exit_stmt)
                                )
                            )
                        )
                    )
                )
            )
        )
    );
    let main = def!(id!("main"), [], main_body);

    prog!([main], [], 10)
}
