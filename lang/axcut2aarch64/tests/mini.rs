use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2aarch64::Backend;
use axcut2backend::coder::compile;
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
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mint = Mint::new("tests/asm");
    let mut mint = mint;
    let mut file = mint.new_goldenfile("mini.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
