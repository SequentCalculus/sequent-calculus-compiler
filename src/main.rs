pub mod compiler;
pub mod core;
pub mod fun;
pub mod grammar;
use compiler::compile_prog;
use core::eval::eval_main;
use core::focusing::Focus;
use core::simplify::Simplify;
use core::syntax::Statement;
use fun::syntax::{Def, Term};
use fun::types::{infer_types, Error, Ty};
use grammar::fun::TermParser;

use std::env;
use std::fmt::Display;

fn main() {
    let arg: String = env::args().next().unwrap();
    dispatch(arg);
}

fn dispatch(arg: String) {
    let parser: TermParser = TermParser::new();
    let parsed: Term = parser.parse(&arg).unwrap();
    let ex_prog: fun::syntax::Prog<()> = fun::syntax::Prog {
        prog_defs: vec![Def {
            name: String::from("main"),
            args: vec![],
            cont: vec![],
            body: parsed,
            ret_ty: (),
        }],
    };

    let m_prog_typed: Result<fun::syntax::Prog<Ty>, Error> = infer_types(ex_prog);
    if m_prog_typed.is_err() {
        return;
    }
    let prog_typed: fun::syntax::Prog<Ty> =
        m_prog_typed.unwrap_or(fun::syntax::Prog { prog_defs: vec![] });
    format_result(&prog_typed, "Type Checking");

    let prog_compiled: core::syntax::Prog<Ty> = compile_prog(prog_typed);
    format_result(&prog_compiled, "Compilation");

    let prog_focused: core::syntax::Prog<Ty> = Focus::focus(prog_compiled);
    format_result(&prog_focused, "Focusing");

    let prog_simplified: core::syntax::Prog<Ty> = Simplify::simplify(prog_focused);
    format_result(&prog_simplified, "Simplification");

    let m_eval_res: Option<Vec<Statement>> = eval_main(prog_simplified);
    if m_eval_res.is_none() {
        println!("No definition main found in program.");
        return;
    }
    let eval_res: Vec<Statement> = m_eval_res.unwrap_or_default();
    format_result(&format_trace(&eval_res), "Evaluation");
}

fn format_result<T: Display>(res: &T, step: &str) {
    println!("--------- Result of {} --------", step);
    println!("{}", res);
}

fn format_trace(tr: &[core::syntax::Statement]) -> String {
    let mut out_str: String = "".to_owned();
    for (i, st) in tr.iter().enumerate() {
        out_str.push_str(&format!("{}: {}", i, st));
    }
    out_str
}
