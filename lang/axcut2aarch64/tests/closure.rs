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

use axcut_macros::{bind, create, invoke, substitute, sum, ty, ty_decl, xtor_sig};
#[test]
fn test_closure() {
    let ty_cont = ty_decl!("Cont", [xtor_sig!("Ret", [bind!("r")])],);
    let ty_func = ty_decl!(
        "Fun",
        [xtor_sig!(
            "apply",
            [bind!("x"), bind!("k", Chirality::Cns, ty!("Cont"))]
        )],
    );

    let main_body = Statement::Literal(Literal {
        lit: 9,
        var: "a".to_string(),
        next: Rc::new(Statement::Create(create!(
            "f",
            ty!("Fun"),
            [Clause {
                xtor: "apply".to_string(),
                context: vec![bind!("x"), bind!("k", Chirality::Cns, ty!("Cont"))].into(),
                body: Rc::new(Statement::Op(sum!(
                    "a",
                    "x",
                    "b",
                    substitute!(
                        [
                            (bind!("b"), "b"),
                            (bind!("k", Chirality::Cns, ty!("Cont")), "k")
                        ],
                        invoke!("k", "Ret", [], ty!("Cont")),
                    )
                ))),
            }],
            create!(
                "k",
                ty!("Cont"),
                [Clause {
                    xtor: "Ret".to_string(),
                    context: vec![bind!("r")].into(),
                    body: Rc::new(Statement::PrintI64(PrintI64 {
                        newline: true,
                        var: "r".to_string(),
                        next: Rc::new(Statement::Literal(Literal {
                            lit: 0,
                            var: "ret".to_string(),
                            next: Rc::new(Statement::Exit(Exit {
                                var: "ret".to_string(),
                            })),
                            free_vars_next: None,
                        })),
                        free_vars_next: None,
                    })),
                }],
                Literal {
                    lit: 1,
                    var: "y".to_string(),
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (bind!("y"), "y".to_string(),),
                            (bind!("k", Chirality::Cns, ty!("Cont")), "k".to_string(),),
                            (bind!("f", Chirality::Prd, ty!("Fun")), "f".to_string(),),
                        ],
                        next: Rc::new(Statement::Invoke(invoke!("f", "apply", [], ty!("Fun")),)),
                    })),
                    free_vars_next: None,
                },
                []
            ),
            [bind!("a")],
        ))),
        free_vars_next: None,
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_cont, ty_func],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("closure.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
