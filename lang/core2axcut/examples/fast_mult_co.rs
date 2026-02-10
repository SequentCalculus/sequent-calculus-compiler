use core_lang::syntax::terms::*;

use printer::Print;

use core_macros::{
    bind, call, clause, cns, cocase, codata, covar, cut, def, dtor, dtor_sig, ife, lit, mu, prd,
    prod, prog, ty, var,
};

fn main() {
    let ty_list = codata!(
        "ListInt",
        [
            dtor_sig!("Nil", []),
            dtor_sig!(
                "Cons",
                [bind!("x", prd!()), bind!("xs", prd!(), ty!("ListInt"))]
            ),
        ]
    );
    let fmult = def!(
        "fmult",
        [bind!("l", cns!(), ty!("ListInt")), bind!("a0", cns!())],
        cut!(
            mu!(
                "a",
                call!(
                    "mult",
                    [covar!("l", ty!("ListInt")), covar!("a"), covar!("a")]
                )
            ),
            covar!("a0")
        ),
        ["l", "a", "a0"]
    );

    let mult = def!(
        "mult",
        [
            bind!("l", cns!(), ty!("ListInt")),
            bind!("a", cns!()),
            bind!("a0", cns!()),
        ],
        cut!(
            cocase!(
                [
                    clause!(Prd, "Nil", [], cut!(lit!(1), covar!("a0"))),
                    clause!(
                        Prd,
                        "Cons",
                        [bind!("x", prd!()), bind!("xs", cns!(), ty!("ListInt"))],
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
                                            [
                                                covar!("xs", ty!("ListInt")),
                                                covar!("a"),
                                                covar!("a1")
                                            ]
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
            covar!("l", ty!("ListInt")),
            ty!("ListInt")
        ),
        ["l", "a", "a0", "a1", "x", "xs"]
    );

    let nil = dtor!("Nil", [], ty!("ListInt"));
    let cons1 = dtor!("Cons", [lit!(3), nil], ty!("ListInt"));
    let cons2 = dtor!("Cons", [lit!(3), cons1], ty!("ListInt"));
    let cons3 = dtor!("Cons", [lit!(0), cons2], ty!("ListInt"));
    let cons4 = dtor!("Cons", [lit!(2), cons3], ty!("ListInt"));

    let main = def!(
        "main",
        [bind!("a0", cns!())],
        call!("fmult", [cons4, covar!("a0")]),
        ["a0"]
    );

    let program = prog!([main, mult, fmult], [], [ty_list]);

    println!("{}\n", program.print_to_string(None));
    let program = program.focus();
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::shrink_prog(program);
    println!("{}", program.print_to_string(None))
}
