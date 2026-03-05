use axcut_macros::{bind, call, def, exit, id, lit, println_i64, prog, sum};

pub fn mini_print() -> axcut::syntax::Prog {
    mini(println_i64!(id!("z", 5), lit!(0, id!("ret", 6), exit!(id!("ret", 6)))).into())
}

pub fn mini_exit() -> axcut::syntax::Prog {
    mini(exit!(id!("z", 5)).into())
}

fn mini(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let main_body = call!(id!("l"), []);
    let main = def!(id!("main"), [], main_body);

    let l_body = lit!(1, id!("x", 1), lit!(9, id!("y", 2), call!(id!("j"), [])));
    let l = def!(id!("l"), [], l_body);

    let j_body = sum!(id!("x", 4), id!("y", 3), id!("z", 5), exit_stmt);
    let j = def!(id!("j"), [bind!(id!("y", 3)), bind!(id!("x", 4))], j_body);

    prog!([main, l, j], [], 6)
}
