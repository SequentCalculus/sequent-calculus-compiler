use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

use axcut_macros::{
    bind, clause, def, exit, letin, lit, prog, substitute, sum, switch, ty, ty_decl, xtor_sig,
};

#[test]
fn test_non_linear() {
    let ty_box = ty_decl!("Box", [xtor_sig!("B", [bind!("b")])]);
    let ty_box_box = ty_decl!(
        "BoxBox",
        [xtor_sig!("BB", [bind!("bb", Chirality::Prd, ty!("Box"))])]
    );

    let main_body_switch_switch = Statement::Switch(switch!(
        "a2",
        ty!("Box"),
        [clause!(
            "B",
            [bind!("x2")],
            substitute!(
                [
                    (bind!("x2"), "x2"),
                    (bind!("a1", Chirality::Prd, ty!("Box")), "a1"),
                ],
                switch!(
                    "a1",
                    ty!("Box"),
                    [clause!(
                        "B",
                        [bind!("x1")],
                        sum!("x1", "x2", "res", exit!("res"))
                    )]
                )
            )
        )]
    ));
    let main_body_switch = Statement::Switch(switch!(
        "bb1",
        ty!("BoxBox"),
        [clause!(
            "BB",
            [bind!("b1", Chirality::Prd, ty!("Box"))],
            switch!(
                "b1",
                ty!("Box"),
                [clause!(
                    "B",
                    [bind!("x1")],
                    letin!(
                        "d1",
                        ty!("Box"),
                        "B",
                        [bind!("x1")],
                        letin!(
                            "dd1",
                            ty!("BoxBox"),
                            "BB",
                            [bind!("d1", Chirality::Prd, ty!("Box"))],
                            substitute!(
                                [(bind!("bb2", Chirality::Prd, ty!("BoxBox")), "bb2",)],
                                lit!(
                                    4,
                                    "y",
                                    letin!(
                                        "a1",
                                        ty!("Box"),
                                        "B",
                                        [bind!("y")],
                                        substitute!(
                                            [
                                                (bind!("a1", Chirality::Prd, ty!("Box")), "a1",),
                                                (
                                                    bind!("bb2", Chirality::Prd, ty!("BoxBox")),
                                                    "bb2",
                                                ),
                                            ],
                                            switch!(
                                                "bb2",
                                                ty!("BoxBox"),
                                                [clause!(
                                                    "BB",
                                                    [bind!("b2", Chirality::Prd, ty!("Box"))],
                                                    switch!(
                                                        "b2",
                                                        ty!("Box"),
                                                        [clause!(
                                                            "B",
                                                            [bind!("x2")],
                                                            letin!(
                                                                "a2",
                                                                ty!("Box"),
                                                                "B",
                                                                [bind!("x2")],
                                                                main_body_switch_switch
                                                            )
                                                        )]
                                                    )
                                                )]
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )]
            )
        )]
    ));
    let main_body = Statement::Literal(lit!(
        3,
        "f1",
        lit!(
            3,
            "f2",
            lit!(
                3,
                "f3",
                lit!(
                    3,
                    "f4",
                    lit!(
                        3,
                        "f5",
                        lit!(
                            3,
                            "f6",
                            lit!(
                                3,
                                "f7",
                                lit!(
                                    3,
                                    "x",
                                    letin!(
                                        "b",
                                        ty!("Box"),
                                        "B",
                                        [bind!("x")],
                                        letin!(
                                            "bb",
                                            ty!("BoxBox"),
                                            "BB",
                                            [bind!("b", Chirality::Prd, ty!("Box"))],
                                            substitute!(
                                                [
                                                    (bind!("f1"), "f1"),
                                                    (bind!("f2"), "f2"),
                                                    (bind!("f3"), "f3"),
                                                    (bind!("f5"), "f5"),
                                                    (bind!("f6"), "f6"),
                                                    (bind!("f7"), "f7"),
                                                    (bind!("f4"), "f4"),
                                                    (
                                                        bind!("bb3", Chirality::Prd, ty!("BoxBox")),
                                                        "bb",
                                                    ),
                                                    (
                                                        bind!("bb2", Chirality::Prd, ty!("BoxBox")),
                                                        "bb",
                                                    ),
                                                    (
                                                        bind!("bb1", Chirality::Prd, ty!("BoxBox")),
                                                        "bb",
                                                    ),
                                                ],
                                                main_body_switch
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            )
        )
    ));
    let main = def!("main", [], main_body);

    let program = prog!([main], [ty_box, ty_box_box]);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("nonLinear.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
