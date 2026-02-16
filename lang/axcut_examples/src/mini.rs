use axcut_macros::{bind, call, def, exit, id, lit, println_i64, prog, sum};

pub fn mini_exit() -> axcut::syntax::Prog {
    mini(exit!(id!("z")).into())
}

pub fn mini_print() -> axcut::syntax::Prog {
    mini(println_i64!(id!("z"), lit!(0, id!("ret"), exit!(id!("ret")))).into())
}

fn mini(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let main_body = call!(id!("l"), []);
    let main = def!(id!("main"), [], main_body);

    let l_body = lit!(1, id!("x"), lit!(9, id!("y"), call!(id!("j"), [])));
    let l = def!(id!("l"), [], l_body);

    let j_body = sum!(id!("x"), id!("y"), id!("z"), exit_stmt);
    let j = def!(id!("j"), [bind!(id!("y")), bind!(id!("x"))], j_body);

    prog!([main, l, j], [])
}
