use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

use axcut_macros::{bind, clause, def, exit, letin, lit, prog, sum, switch, ty, ty_decl, xtor_sig};

#[test]
fn test_either() {
    let ty_either = ty_decl!(
        "Either",
        [
            xtor_sig!("Left", [bind!("x", 0)]),
            xtor_sig!("Right", [bind!("y", 0)],),
        ],
    );

    let main_body = lit!(
        1,
        ("z", 0),
        lit!(
            9,
            ("x", 0),
            letin!(
                ("p", 0),
                ty!("Either"),
                "Right",
                [bind!("x", 0)],
                switch!(
                    ("p", 0),
                    ty!("Either"),
                    [
                        clause!(
                            "Left",
                            [bind!("a", 0)],
                            lit!(-1, ("err", 0), exit!(("err", 0)))
                        ),
                        clause!(
                            "Right",
                            [bind!("b", 0)],
                            sum!(("b", 0), ("z", 0), ("c", 0), exit!(("c", 0)))
                        ),
                    ]
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_either]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("either.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
