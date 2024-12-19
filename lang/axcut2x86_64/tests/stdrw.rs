use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2x86_64::into_routine::into_x86_64_routine;
use axcut2x86_64::Backend;
use goldenfile::Mint;
use printer::Print;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

#[test]
fn test_stdrw() {
    let main_body = Statement::Literal(Literal {
        lit: 5,
        var: "c".to_string(),
        case: Rc::new(Statement::MMapAnonymousPage(MMapAnonymousPage {
            var: "m".to_string(),
            case: Rc::new(Statement::ReadStdin(ReadStdin {
                buffer: "m".to_string(),
                count: "c".to_string(),
                var: "r".to_string(),
                case: Rc::new(Statement::WriteStdout(WriteStdout {
                    buffer: "m".to_string(),
                    count: "r".to_string(),
                    var: "w".to_string(),
                    case: Rc::new(Statement::MUnmapPage(MUnmapPage {
                        var: "m".to_string(),
                        case: Rc::new(Statement::Return(Return {
                            var: "w".to_string(),
                        })),
                    })),
                })),
            })),
        })),
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("stdrw.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
