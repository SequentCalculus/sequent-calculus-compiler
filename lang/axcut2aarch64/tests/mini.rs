use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2aarch64::Backend;
use axcut2backend::coder::compile;
use printer::Print;

use std::collections::HashSet;
use std::rc::Rc;

use pretty_assertions::assert_eq;

use std::fs::File;
use std::io::prelude::*;

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
        case: Rc::new(Statement::Literal(Literal {
            lit: 9,
            var: "y".to_string(),
            case: Rc::new(Statement::Call(Call {
                label: "j".to_string(),
                args: vec![],
            })),
        })),
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
        case: Rc::new(Statement::Return(Return {
            var: "z".to_string(),
        })),
    });
    let j = Def {
        name: "j".to_string(),
        context: vec![
            ContextBinding {
                var: "y".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
            },
            ContextBinding {
                var: "x".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
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
    let assembler_code = into_aarch64_routine(assembly_prog);

    //let mut file = File::create("tests/asm/mini.aarch64.asm")
    //    .expect("Cannot create file tests/asm/mini.aarch64.asm");
    //file.write_all(&mut assembler_code.as_bytes())
    //    .expect("Cannot write to file tests/asm/mini.aarch64.asm");
    let mut file = File::open("tests/asm/mini.aarch64.asm")
        .expect("Cannot open file tests/asm/mini.aarch64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/mini.aarch64.asm");

    assert_eq!(assembler_code.print_to_string(None), reference_code);
}
