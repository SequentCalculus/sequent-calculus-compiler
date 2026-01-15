use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

#[test]
fn test_mini() {
    let main_body = Statement::Call(Call {
        label: "l".to_string(),
        args: vec![].into(),
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let l_body = Statement::Literal(Literal {
        lit: 1,
        var: Var {
            name: "x".to_string(),
            id: 0,
        },
        next: Rc::new(Statement::Literal(Literal {
            lit: 9,
            var: Var {
                name: "y".to_string(),
                id: 0,
            },
            next: Rc::new(Statement::Call(Call {
                label: "j".to_string(),
                args: vec![].into(),
            })),
            free_vars_next: None,
        })),
        free_vars_next: None,
    });
    let l = Def {
        name: "l".to_string(),
        context: Vec::new().into(),
        body: l_body,
        used_vars: HashSet::new(),
    };

    let j_body = Statement::Op(Op {
        fst: Var {
            name: "x".to_string(),
            id: 0,
        },
        op: BinOp::Sum,
        snd: Var {
            name: "y".to_string(),
            id: 0,
        },
        var: Var {
            name: "z".to_string(),
            id: 0,
        },
        next: Rc::new(Statement::Exit(Exit {
            var: Var {
                name: "z".to_string(),
                id: 0,
            },
        })),
        free_vars_next: None,
    });
    let j = Def {
        name: "j".to_string(),
        context: vec![
            ContextBinding {
                var: Var {
                    name: "y".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            },
            ContextBinding {
                var: Var {
                    name: "x".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            },
        ]
        .into(),
        body: j_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main, l, j],
        types: Vec::new(),
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("mini.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
