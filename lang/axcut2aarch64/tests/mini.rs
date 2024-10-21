use axcut::syntax::*;
use axcut2aarch64::code::pretty;
use axcut2aarch64::coder::compile;
use axcut2aarch64::into_routine::into_aarch64_routine;

use std::rc::Rc;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_mini() {
    let main_body = Statement::Call(Call {
        label: "l".to_string(),
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new(),
        body: main_body,
    };

    let l_body = Statement::Literal(Literal {
        lit: 1,
        var: "x".to_string(),
        case: Rc::new(Statement::Literal(Literal {
            lit: 9,
            var: "y".to_string(),
            case: Rc::new(Statement::Call(Call {
                label: "j".to_string(),
            })),
        })),
    });
    let l = Def {
        name: "l".to_string(),
        context: Vec::new(),
        body: l_body,
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
                pol: Polarity::Ext,
                ty: Ty::Int,
            },
            ContextBinding {
                var: "x".to_string(),
                pol: Polarity::Ext,
                ty: Ty::Int,
            },
        ],
        body: j_body,
    };

    let program = Prog {
        defs: vec![main, l, j],
        types: Vec::new(),
    };

    let (code, arg_num) = compile(program);
    let assembler_code = into_aarch64_routine("mini", &pretty(code), arg_num);

    let mut file = File::open("tests/asm/mini.aarch64.asm")
        .expect("Cannot open file tests/asm/mini.aarch64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/mini.aarch64.asm");

    assert_eq!(assembler_code, reference_code);
}
