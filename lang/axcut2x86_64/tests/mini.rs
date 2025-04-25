use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

#[test]
fn test_mini() {
    let main_body = Statement::Call(Call {
        label: "l".to_string(),
        args: vec![],
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let l_body = Statement::Literal(Literal {
        lit: 1,
        var: "x".to_string(),
        next: Rc::new(Statement::Literal(Literal {
            lit: 9,
            var: "y".to_string(),
            next: Rc::new(Statement::Call(Call {
                label: "j".to_string(),
                args: vec![],
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
        fst: "x".to_string(),
        op: BinOp::Sum,
        snd: "y".to_string(),
        var: "z".to_string(),
        next: Rc::new(Statement::PrintI64(PrintI64 {
            newline: true,
            var: "z".to_string(),
            next: Rc::new(Statement::Literal(Literal {
                lit: 0,
                var: "ret".to_string(),
                next: Rc::new(Statement::Return(Return {
                    var: "ret".to_string(),
                })),
                free_vars_next: None,
            })),
            free_vars_next: None,
        })),
        free_vars_next: None,
    });
    let j = Def {
        name: "j".to_string(),
        context: vec![
            ContextBinding {
                var: "y".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            },
            ContextBinding {
                var: "x".to_string(),
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
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("mini.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
