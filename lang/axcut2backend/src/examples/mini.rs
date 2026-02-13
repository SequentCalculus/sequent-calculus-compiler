use axcut_macros::{bind, call, def, exit, lit, println_i64, prog, sum};

pub fn mini_exit() -> axcut::syntax::Prog {
    mini(exit!("z").into())
}

pub fn mini_print() -> axcut::syntax::Prog {
    mini(println_i64!("z", lit!(0, "ret", exit!("ret"))).into())
}

fn mini(exit_stmt: axcut::syntax::Statement) -> axcut::syntax::Prog {
    let main_body = call!("l", []);
    let main = def!("main", [], main_body);

    let l_body = lit!(1, "x", lit!(9, "y", call!("j", [])));
    let l = def!("l", [], l_body);

    let j_body = sum!("x", "y", "z", exit_stmt);
    let j = def!("j", [bind!("y"), bind!("x")], j_body);

    prog!([main, l, j], [])
}
