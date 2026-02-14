use core_lang::syntax::terms::*;

use printer::Print;

use core_macros::{
    bind, call, case, clause, cns, covar, ctor, ctor_sig, cut, data, def, id, ife, lit, mu, prd,
    prod, prog, ty, var,
};

fn main() {
    let ty_list = data!(
        id!("ListInt"),
        [
            ctor_sig!(id!("Nil"), []),
            ctor_sig!(
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
            bind!(id!("l"), prd!(), ty!(id!("ListInt"))),
            bind!(id!("a"), cns!())
        ],
        cut!(
            mu!(
                id!("a", 1),
                call!(
                    id!("mult"),
                    [
                        var!(id!("l"), ty!(id!("ListInt"))),
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
            bind!(id!("l"), prd!(), ty!(id!("ListInt"))),
            bind!(id!("a", 2), cns!()),
            bind!(id!("a"), cns!()),
        ],
        cut!(
            var!(id!("l"), ty!(id!("ListInt"))),
            case!(
                [
                    clause!(Cns, id!("Nil"), [], cut!(lit!(1), covar!(id!("a")))),
                    clause!(
                        Cns,
                        id!("Cons"),
                        [
                            bind!(id!("x"), prd!()),
                            bind!(id!("xs"), prd!(), ty!(id!("ListInt")))
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
                                                var!(id!("xs"), ty!(id!("ListInt"))),
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

    let nil = ctor!(id!("Nil"), [], ty!(id!("ListInt")));
    let cons1 = ctor!(id!("Cons"), [lit!(3), nil], ty!(id!("ListInt")));
    let cons2 = ctor!(id!("Cons"), [lit!(3), cons1]);
    let cons3 = ctor!(id!("Cons"), [lit!(0), cons2]);
    let cons4 = ctor!(id!("Cons"), [lit!(2), cons3]);

    let main = def!(
        id!("main"),
        [bind!(id!("a"), cns!())],
        call!(id!("fmult"), [cons4, covar!(id!("a"))]),
        [id!("a")]
    );

    let program = prog!([main, mult, fmult], [ty_list], []);

    println!("{}\n", program.print_to_string(None));
    let program = program.focus();
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::shrink_prog(program);
    println!("{}", program.print_to_string(None))
}
