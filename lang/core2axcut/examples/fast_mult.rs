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
                [bind!("x", prd!()), bind!("xs", prd!(), ty!("ListInt"))]
            ),
        ]
    );

    let fmult = def!(
        "fmult",
        [bind!("l", prd!(), ty!("ListInt")), bind!("a0", cns!())],
        cut!(
            mu!(
                "a",
                call!(
                    "mult",
                    [var!("l", ty!("ListInt")), covar!("a"), covar!("a")]
                )
            ),
            covar!("a0")
        ),
        ["l", "a", "a0"]
    );

    let mult = def!(
        "mult",
        [
            bind!("l", prd!(), ty!("ListInt")),
            bind!("a", cns!()),
            bind!("a0", cns!()),
        ],
        cut!(
            var!("l", ty!("ListInt")),
            case!(
                [
                    clause!(Cns, "Nil", [], cut!(lit!(1), covar!("a0"))),
                    clause!(
                        Cns,
                        "Cons",
                        [bind!("x", prd!()), bind!("xs", prd!(), ty!("ListInt"))],
                        ife!(
                            var!("x"),
                            cut!(lit!(0), covar!("a")),
                            cut!(
                                prod!(
                                    var!("x"),
                                    mu!(
                                        "a1",
                                        call!(
                                            "mult",
                                            [var!("xs", ty!("ListInt")), covar!("a"), covar!("a1")]
                                        )
                                    )
                                ),
                                covar!("a0")
                            )
                        )
                    ),
                ],
                ty!("ListInt")
            ),
            ty!("ListInt")
        ),
        ["l", "a", "a0", "a1", "x", "xs"]
    );

    let nil = ctor!("Nil", [], ty!("ListInt"));
    let cons1 = ctor!("Cons", [lit!(3), nil], ty!("ListInt"));
    let cons2 = ctor!("Cons", [lit!(3), cons1]);
    let cons3 = ctor!("Cons", [lit!(0), cons2]);
    let cons4 = ctor!("Cons", [lit!(2), cons3]);

    let main = def!(
        "main",
        [bind!("a0", cns!())],
        call!("fmult", [cons4, covar!("a0")]),
        ["a0"]
    );

    let program = prog!([main, mult, fmult], [ty_list], []);

    println!("{}\n", program.print_to_string(None));
    let program = program.focus();
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::shrink_prog(program);
    println!("{}", program.print_to_string(None))
}
