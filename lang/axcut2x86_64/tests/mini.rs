use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::code::pretty;
use axcut2backend::coder::compile;
use axcut2x86_64::into_routine::into_x86_64_routine;
use axcut2x86_64::Backend;

use std::collections::HashSet;
use std::rc::Rc;

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
        context: Vec::new(),
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
        context: Vec::new(),
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
        ],
        body: j_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main, l, j],
        types: Vec::new(),
    };

    let (code, arg_num) = compile(program, &Backend);
    let assembler_code = into_x86_64_routine(&pretty(code), arg_num);

    //let mut file = File::create("tests/asm/mini.x86_64.asm")
    //    .expect("Cannot create file tests/asm/mini.x86_64.asm");
    //file.write_all(&mut assembler_code.as_bytes())
    //    .expect("Cannot write to file tests/asm/mini.x86_64.asm");
    let mut file = File::open("tests/asm/mini.x86_64.asm")
        .expect("Cannot open file tests/asm/mini.x86_64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/mini.x86_64.asm");

    assert_eq!(assembler_code, reference_code);
}
