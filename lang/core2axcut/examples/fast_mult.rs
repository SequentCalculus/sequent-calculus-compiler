use core_lang::syntax::terms::*;

use printer::Print;

use core_macros::{
    bind, call, case, clause, cns, covar, ctor, ctor_sig, cut, data, def, ife, lit, mu, prd, prod,
    prog, ty, var,
};

fn main() {
    let ty_list = data!(
        "ListInt",
        [
            ctor_sig!("Nil", []),
            ctor_sig!(
                "Cons",
                [
                    bind!("x", 0, prd!()),
                    bind!("xs", 0, prd!(), ty!("ListInt"))
                ]
            ),
        ]
    );

    let fmult = def!(
        "fmult",
        [bind!("l", 0, prd!(), ty!("ListInt")), bind!("a", 0, cns!())],
        cut!(
            mu!(
                ("a", 1),
                call!(
                    "mult",
                    [var!("l", 0, ty!("ListInt")), covar!("a", 1), covar!("a", 1)]
                )
            ),
            covar!("a", 0)
        ),
        [("l", 0), ("a", 1), ("a", 0)]
    );

    let mult = def!(
        "mult",
        [
            bind!("l", 0, prd!(), ty!("ListInt")),
            bind!("a", 1, cns!()),
            bind!("a", 0, cns!()),
        ],
        cut!(
            var!("l", 0, ty!("ListInt")),
            case!(
                [
                    clause!(Cns, "Nil", [], cut!(lit!(1), covar!("a", 0))),
                    clause!(
                        Cns,
                        "Cons",
                        [
                            bind!("x", 0, prd!()),
                            bind!("xs", 0, prd!(), ty!("ListInt"))
                        ],
                        ife!(
                            var!("x", 0),
                            cut!(lit!(0), covar!("a", 2)),
                            cut!(
                                prod!(
                                    var!("x", 0),
                                    mu!(
                                        ("a", 2),
                                        call!(
                                            "mult",
                                            [
                                                var!("xs", 0, ty!("ListInt")),
                                                covar!("a", 2),
                                                covar!("a", 1)
                                            ]
                                        )
                                    )
                                ),
                                covar!("a", 0)
                            )
                        )
                    ),
                ],
                ty!("ListInt")
            ),
            ty!("ListInt")
        ),
        [("l", 0), ("a", 1), ("a", 0), ("a", 2), ("x", 0), ("xs", 0)]
    );

    let nil = ctor!("Nil", [], ty!("ListInt"));
    let cons1 = ctor!("Cons", [lit!(3), nil], ty!("ListInt"));
    let cons2 = ctor!("Cons", [lit!(3), cons1]);
    let cons3 = ctor!("Cons", [lit!(0), cons2]);
    let cons4 = ctor!("Cons", [lit!(2), cons3]);

    let main = def!(
        "main",
        [bind!("a", 0, cns!())],
        call!("fmult", [cons4, covar!("a", 0)]),
        [("a", 0)]
    );

    let program = prog!([main, mult, fmult], [ty_list], []);

    println!("{}\n", program.print_to_string(None));
    let program = program.focus();
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::shrink_prog(program);
    println!("{}", program.print_to_string(None))
}
