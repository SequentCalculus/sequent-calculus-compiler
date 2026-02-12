use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::io::prelude::*;

use axcut_macros::{def, exit, lit, prod, prog, sub, sum};

#[test]
fn test_arith() {
    let main_body = lit!(
        1,
        ("a", 0),
        lit!(
            3,
            ("b", 0),
            sub!(
                ("a", 0),
                ("b", 0),
                ("c", 0),
                lit!(
                    8,
                    ("d", 0),
                    lit!(
                        -1,
                        ("e", 0),
                        prod!(
                            ("e", 0),
                            ("d", 0),
                            ("f", 0),
                            sum!(
                                ("f", 0),
                                ("c", 0),
                                ("g", 0),
                                lit!(
                                    -6,
                                    ("h", 0),
                                    prod!(("h", 0), ("g", 0), ("i", 0), exit!(("i", 0)))
                                )
                            )
                        )
                    )
                )
            )
        )
    );
    let main = def!("main", [], main_body);

    let program = prog!([main], []);

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("arith.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
