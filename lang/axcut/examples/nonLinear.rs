use axcut::syntax::statements::*;
use printer::Print;

use axcut_macros::{
    bind, clause, def, exit, letin, lit, prd, prog, sum, switch, ty, ty_decl, xtor_sig,
};

fn main() {
    let ty_box = ty_decl!("Box", [xtor_sig!("B", [bind!("b", 0)])]);
    let ty_box_box = ty_decl!(
        "BoxBox",
        [xtor_sig!("BB", [bind!("bb", 0, prd!(), ty!("Box"))])]
    );

    let main_body_switch_switch = Statement::Switch(switch!(
        ("a", 2),
        ty!("Box"),
        [clause!(
            "B",
            [bind!("y", 2)],
            switch!(
                ("a", 1),
                ty!("Box"),
                [clause!(
                    "B",
                    [bind!("y", 1)],
                    sum!(("y", 1), ("y", 2), ("res", 0), exit!(("res", 0)))
                )]
            )
        )]
    ));
    let main_body_switch = switch!(
        ("bb", 0),
        ty!("BoxBox"),
        [clause!(
            "BB",
            [bind!("b", 1, prd!(), ty!("Box"))],
            switch!(
                ("b", 1),
                ty!("Box"),
                [clause!(
                    "B",
                    [bind!("x", 1)],
                    letin!(
                        ("d", 1),
                        ty!("Box"),
                        "B",
                        [bind!("x", 1)],
                        letin!(
                            ("dd", 1),
                            ty!("BoxBox"),
                            "BB",
                            [bind!("d", 1, prd!(), ty!("Box"))],
                            lit!(
                                4,
                                ("y", 0),
                                letin!(
                                    ("a", 1),
                                    ty!("Box"),
                                    "B",
                                    [bind!("y", 0)],
                                    switch!(
                                        ("bb", 0),
                                        ty!("BoxBox"),
                                        [clause!(
                                            "BB",
                                            [bind!("b", 2, prd!(), ty!("Box"))],
                                            switch!(
                                                ("b", 2),
                                                ty!("Box"),
                                                [clause!(
                                                    "B",
                                                    [bind!("x", 2)],
                                                    letin!(
                                                        ("a", 2),
                                                        ty!("Box"),
                                                        "B",
                                                        [bind!("x", 2)],
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
                )]
            )
        )]
    );
    let main_body = lit!(
        3,
        ("f", 1),
        lit!(
            3,
            ("f", 2),
            lit!(
                3,
                ("x", 0),
                letin!(
                    ("b", 0),
                    ty!("Box"),
                    "B",
                    [bind!("x", 0)],
                    letin!(
                        ("bb", 0),
                        ty!("BoxBox"),
                        "BB",
                        [bind!("b", 1, prd!(), ty!("Box"))],
                        main_body_switch
                    )
                )
            )
        )
    );
    let main = def!(
        "main",
        [],
        main_body,
        [
            ("bb", 0),
            ("a", 2),
            ("f", 1),
            ("b", 1),
            ("b", 0),
            ("b", 2),
            ("y", 1),
            ("a", 1),
            ("y", 0),
            ("res", 0),
            ("dd", 1),
            ("x", 1),
            ("x", 2),
            ("d", 1),
            ("x", 0),
            ("f", 2),
            ("y", 2)
        ]
    );

    let program = prog!([main], [ty_box, ty_box_box]);

    println!("{}", program.linearize().print_to_string(None))
}
