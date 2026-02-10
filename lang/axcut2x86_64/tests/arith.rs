use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::io::prelude::*;

use axcut_macros::{def, exit, lit, println_i64, prod, prog, sub, sum};

#[test]
fn test_arith() {
    let main_body = lit!(
        1,
        "a",
        lit!(
            3,
            "b",
            sub!(
                "a",
                "b",
                "c",
                lit!(
                    8,
                    "d",
                    lit!(
                        -1,
                        "e",
                        prod!(
                            "e",
                            "d",
                            "f",
                            sum!(
                                "f",
                                "c",
                                "g",
                                lit!(
                                    -6,
                                    "h",
                                    prod!(
                                        "h",
                                        "g",
                                        "i",
                                        println_i64!("i", lit!(0, "ret", exit!("ret")))
                                    )
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
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("arith.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
