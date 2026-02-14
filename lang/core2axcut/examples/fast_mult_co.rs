use core_lang::syntax::terms::*;

use printer::Print;

use core_macros::{
    bind, call, clause, cns, cocase, codata, covar, cut, def, dtor, dtor_sig, id, ife, lit, mu,
    prd, prod, prog, ty, var,
};

fn main() {
    let ty_list = codata!(
        id!("ListInt"),
        [
            dtor_sig!(id!("Nil"), []),
            dtor_sig!(
                id!("Cons"),
                [
                    bind!(id!("x"), prd!()),
                    bind!(id!("xs"), prd!(), ty!(id!("ListInt")))
                ]
            ),
        ]
    );
    let fmult = def!(
        id!("fmult"),
        [
            bind!(id!("l"), cns!(), ty!(id!("ListInt"))),
            bind!(id!("a"), cns!())
        ],
        cut!(
            mu!(
                id!("a", 1),
                call!(
                    id!("mult"),
                    [
                        covar!(id!("l"), ty!(id!("ListInt"))),
                        covar!(id!("a", 1)),
                        covar!(id!("a", 1))
                    ]
                )
            ),
            covar!(id!("a"))
        ),
        [id!("l"), id!("a"), id!("a", 1)]
    );

    let mult = def!(
        id!("mult"),
        [
            bind!(id!("l"), cns!(), ty!(id!("ListInt"))),
            bind!(id!("a", 2), cns!()),
            bind!(id!("a"), cns!()),
        ],
        cut!(
            cocase!(
                [
                    clause!(Prd, id!("Nil"), [], cut!(lit!(1), covar!(id!("a")))),
                    clause!(
                        Prd,
                        id!("Cons"),
                        [
                            bind!(id!("x"), prd!()),
                            bind!(id!("xs"), cns!(), ty!(id!("ListInt")))
                        ],
                        ife!(
                            var!(id!("x")),
                            cut!(lit!(0), covar!(id!("a", 2))),
                            cut!(
                                prod!(
                                    var!(id!("x")),
                                    mu!(
                                        id!("a", 1),
                                        call!(
                                            id!("mult"),
                                            [
                                                covar!(id!("xs"), ty!(id!("ListInt"))),
                                                covar!(id!("a", 2)),
                                                covar!(id!("a", 1))
                                            ]
                                        )
                                    )
                                ),
                                covar!(id!("a"))
                            )
                        )
                    ),
                ],
                ty!(id!("ListInt"))
            ),
            covar!(id!("l"), ty!(id!("ListInt"))),
            ty!(id!("ListInt"))
        ),
        [
            id!("l"),
            id!("a"),
            id!("a", 1),
            id!("a", 2),
            id!("x"),
            id!("xs")
        ]
    );

    let nil = dtor!(id!("Nil"), [], ty!(id!("ListInt")));
    let cons1 = dtor!(id!("Cons"), [lit!(3), nil], ty!(id!("ListInt")));
    let cons2 = dtor!(id!("Cons"), [lit!(3), cons1], ty!(id!("ListInt")));
    let cons3 = dtor!(id!("Cons"), [lit!(0), cons2], ty!(id!("ListInt")));
    let cons4 = dtor!(id!("Cons"), [lit!(2), cons3], ty!(id!("ListInt")));

    let main = def!(
        id!("main"),
        [bind!(id!("a"), cns!())],
        call!(id!("fmult"), [cons4, covar!(id!("a"))]),
        [id!("a")]
    );

    let program = prog!([main, mult, fmult], [], [ty_list]);

    println!("{}\n", program.print_to_string(None));
    let program = program.focus();
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::shrink_prog(program);
    println!("{}", program.print_to_string(None))
}
