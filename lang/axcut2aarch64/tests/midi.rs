use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2aarch64::Backend;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2backend::coder::compile;
use goldenfile::Mint;
use printer::Print;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

use axcut_macros::{
    bind, call, clause, create, def, exit, ife, invoke, letin, lit, println_i64, substitute, sum,
    switch, ty, ty_decl, xtor_sig,
};

#[test]
fn test_midi() {
    let ty_list = ty_decl!(
        "List",
        [
            xtor_sig!("Nil", []),
            xtor_sig!(
                "Cons",
                [bind!("xs", Chirality::Prd, ty!("List")), bind!("x")]
            )
        ]
    );

    let ty_cont_list = ty_decl!(
        "ContList",
        [xtor_sig!(
            "Retl",
            [bind!("kl", Chirality::Prd, ty!("List"))]
        ),]
    );

    let ty_cont_int = ty_decl!("ContInt", [xtor_sig!("Reti", [bind!("ki")]),]);

    let main_body = create!(
        "t",
        ty!("ContInt"),
        [],
        [clause!(
            "Reti",
            [bind!("r")],
            println_i64!("r", lit!(0, "ret", exit!("ret")))
        )],
        create!(
            "k",
            ty!("ContList"),
            [bind!("t", Chirality::Cns, ty!("ContInt"))],
            [clause!(
                "Retl",
                [bind!("as", Chirality::Prd, ty!("List"))],
                substitute!(
                    [
                        (bind!("t", Chirality::Cns, ty!("ContInt")), "t"),
                        (bind!("as", Chirality::Prd, ty!("List")), "as"),
                    ],
                    call!("sum", [])
                )
            )],
            letin!(
                "zs",
                ty!("List"),
                "Nil",
                [],
                lit!(
                    3,
                    "n",
                    substitute!(
                        [
                            (bind!("k", Chirality::Cns, ty!("ContInt")), "k"),
                            (bind!("zs", Chirality::Prd, ty!("List")), "zs"),
                            (bind!("n"), "n",),
                        ],
                        call!("range", [])
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let range_body = Statement::IfC(ife!(
        "i",
        substitute!(
            [
                (bind!("xs", Chirality::Prd, ty!("List")), "xs"),
                (bind!("k", Chirality::Cns, ty!("ContList")), "k"),
            ],
            invoke!("k", "Retl", ty!("ContList"), [])
        ),
        substitute!(
            [
                (bind!("n"), "i"),
                (bind!("k", Chirality::Cns, ty!("ContList")), "k"),
                (bind!("xs", Chirality::Prd, ty!("List")), "xs"),
                (bind!("i"), "i"),
            ],
            letin!(
                "ys",
                ty!("List"),
                "Cons",
                [bind!("xs", Chirality::Prd, ty!("List")), bind!("i")],
                lit!(
                    -1,
                    "o",
                    sum!(
                        "n",
                        "o",
                        "j",
                        substitute!(
                            [
                                (bind!("k", Chirality::Cns, ty!("ContList")), "k"),
                                (bind!("ys", Chirality::Prd, ty!("List")), "ys"),
                                (bind!("j"), "j"),
                            ],
                            call!("range", [])
                        )
                    ),
                )
            )
        ),
    ));
    let range = def!(
        "range",
        [
            bind!("k", Chirality::Cns, ty!("ContList")),
            bind!("xs", Chirality::Prd, ty!("List")),
            bind!("i"),
        ],
        range_body
    );

    let sum_body = switch!(
        "xs",
        ty!("List"),
        [
            Clause {
                xtor: "Nil".to_string(),
                context: vec![].into(),
                body: Rc::new(Statement::Literal(Literal {
                    lit: 0,
                    var: "z".to_string(),
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (bind!("z"), "z".to_string()),
                            (bind!("k", Chirality::Cns, ty!("ContInt")), "k".to_string()),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: "k".to_string(),
                            tag: "Reti".to_string(),
                            ty: Ty::Decl("ContInt".to_string()),
                            args: vec![].into(),
                        })),
                    })),
                    free_vars_next: None,
                })),
            },
            Clause {
                xtor: "Cons".to_string(),
                context: vec![bind!("ys", Chirality::Prd, ty!("List")), bind!("y")].into(),
                body: Rc::new(Statement::Substitute(Substitute {
                    rearrange: vec![
                        (bind!("ys", Chirality::Prd, ty!("List")), "ys".to_string()),
                        (bind!("k", Chirality::Cns, ty!("ContInt")), "k".to_string()),
                        (bind!("y"), "y".to_string()),
                    ],
                    next: Rc::new(Statement::Create(Create {
                        var: "j".to_string(),
                        ty: Ty::Decl("ContInt".to_string()),
                        context: Some(
                            vec![bind!("k", Chirality::Cns, ty!("ContInt")), bind!("y")].into(),
                        ),
                        clauses: vec![Clause {
                            xtor: "Reti".to_string(),
                            context: vec![bind!("r")].into(),
                            body: Rc::new(Statement::Op(Op {
                                fst: "y".to_string(),
                                op: BinOp::Sum,
                                snd: "r".to_string(),
                                var: "s".to_string(),
                                next: Rc::new(Statement::Substitute(Substitute {
                                    rearrange: vec![
                                        (bind!("s"), "s".to_string()),
                                        (
                                            bind!("k", Chirality::Cns, ty!("ContInt")),
                                            "k".to_string(),
                                        ),
                                    ],
                                    next: Rc::new(Statement::Invoke(Invoke {
                                        var: "k".to_string(),
                                        tag: "Reti".to_string(),
                                        ty: Ty::Decl("ContInt".to_string()),
                                        args: vec![].into(),
                                    })),
                                })),
                                free_vars_next: None,
                            })),
                        }],
                        free_vars_clauses: None,
                        next: Rc::new(Statement::Substitute(Substitute {
                            rearrange: vec![
                                (bind!("j", Chirality::Cns, ty!("ContInt")), "j".to_string()),
                                (bind!("ys", Chirality::Prd, ty!("List")), "ys".to_string()),
                            ],
                            next: Rc::new(Statement::Call(Call {
                                label: "sum".to_string(),
                                args: vec![].into(),
                            })),
                        })),
                        free_vars_next: None,
                    })),
                })),
            }
        ]
    );
    let sum = Def {
        name: "sum".to_string(),
        context: vec![
            bind!("k", Chirality::Cns, ty!("ContList")),
            bind!("xs", Chirality::Prd, ty!("List")),
        ]
        .into(),
        body: sum_body.into(),
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main, range, sum],
        types: vec![ty_list, ty_cont_list, ty_cont_int],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("midi.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
