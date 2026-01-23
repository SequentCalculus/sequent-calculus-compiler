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
    bind, call, clause, create, def, exit, letin, lit, println_i64, substitute, ty, ty_decl,
    xtor_sig,
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

    let range_body = Statement::IfC(IfC {
        sort: ifc::IfSort::Equal,
        fst: "i".to_string(),
        snd: None,
        thenc: Rc::new(Statement::Substitute(Substitute {
            rearrange: vec![
                (bind!("xs", Chirality::Prd, ty!("List")), "xs".to_string()),
                (bind!("k", Chirality::Cns, ty!("ContList")), "k".to_string()),
            ],
            next: Rc::new(Statement::Invoke(Invoke {
                var: "k".to_string(),
                tag: "Retl".to_string(),
                ty: Ty::Decl("ContList".to_string()),
                args: vec![].into(),
            })),
        })),
        elsec: Rc::new(Statement::Substitute(Substitute {
            rearrange: vec![
                (bind!("n"), "i".to_string()),
                (bind!("k", Chirality::Cns, ty!("ContList")), "k".to_string()),
                (bind!("xs", Chirality::Prd, ty!("List")), "xs".to_string()),
                (bind!("i"), "i".to_string()),
            ],
            next: Rc::new(Statement::Let(Let {
                var: "ys".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Cons".to_string(),
                args: vec![bind!("xs", Chirality::Prd, ty!("List")), bind!("i")].into(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: -1,
                    var: "o".to_string(),
                    next: Rc::new(Statement::Op(Op {
                        fst: "n".to_string(),
                        op: BinOp::Sum,
                        snd: "o".to_string(),
                        var: "j".to_string(),
                        next: Rc::new(Statement::Substitute(Substitute {
                            rearrange: vec![
                                (bind!("k", Chirality::Cns, ty!("ContList")), "k".to_string()),
                                (bind!("ys", Chirality::Prd, ty!("List")), "ys".to_string()),
                                (bind!("j"), "j".to_string()),
                            ],
                            next: Rc::new(Statement::Call(Call {
                                label: "range".to_string(),
                                args: vec![].into(),
                            })),
                        })),
                        free_vars_next: None,
                    })),
                    free_vars_next: None,
                })),
                free_vars_next: None,
            })),
        })),
    });
    let range = Def {
        name: "range".to_string(),
        context: vec![
            bind!("k", Chirality::Cns, ty!("ContList")),
            bind!("xs", Chirality::Prd, ty!("List")),
            bind!("i"),
        ]
        .into(),
        body: range_body,
        used_vars: HashSet::new(),
    };

    let sum_body = Statement::Switch(Switch {
        var: "xs".to_string(),
        ty: Ty::Decl("List".to_string()),
        clauses: vec![
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
            },
        ],
        free_vars_clauses: None,
    });
    let sum = Def {
        name: "sum".to_string(),
        context: vec![
            bind!("k", Chirality::Cns, ty!("ContList")),
            bind!("xs", Chirality::Prd, ty!("List")),
        ]
        .into(),
        body: sum_body,
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
