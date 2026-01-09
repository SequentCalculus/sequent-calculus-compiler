//! This module defines the translation of a typechecked [Fun](fun) program into a
//! [Core](core_lang) program.

use crate::{
    declaration::{compile_ctor, compile_dtor},
    def::{compile_def, compile_main},
};

use std::collections::VecDeque;

/// This function translates a typechecked [Fun](fun) program into a [Core](core_lang) program.
/// - `program` is the typechecked [Fun](fun) program.
pub fn compile_prog(prog: fun::syntax::program::CheckedProgram) -> core_lang::syntax::Prog {
    let mut data_types = Vec::new();
    let mut codata_types = Vec::new();

    for data in prog.data_types {
        data_types.push(core_lang::syntax::declaration::TypeDeclaration {
            dat: core_lang::syntax::declaration::Data,
            name: data.name,
            xtors: data.ctors.into_iter().map(compile_ctor).collect(),
        });
    }
    for codata in prog.codata_types {
        codata_types.push(core_lang::syntax::declaration::TypeDeclaration {
            dat: core_lang::syntax::declaration::Codata,
            name: codata.name,
            xtors: codata.dtors.into_iter().map(compile_dtor).collect(),
        });
    }

    let mut used_labels = prog.defs.iter().map(|def| def.name.clone()).collect();
    let mut defs_translated = VecDeque::new();
    for def in prog.defs {
        if def.name == "main" {
            for def_main in compile_main(def, codata_types.as_slice(), &mut used_labels)
                .into_iter()
                .rev()
            {
                defs_translated.push_front(def_main);
            }
        } else {
            defs_translated.extend(compile_def(def, codata_types.as_slice(), &mut used_labels));
        }
    }

    core_lang::syntax::Prog {
        defs: defs_translated.into(),
        data_types,
        codata_types,
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::{
        def::{compile_def, compile_main},
        program::compile_prog,
    };
    use codespan::Span;
    use fun::syntax::{
        context::{Chirality::Prd, TypeContext},
        declarations::Def,
        program::CheckedProgram,
        terms::{Lit, XVar},
        types::Ty,
    };
    use std::collections::HashSet;

    fn example_def1() -> Def {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        Def {
            span: Span::default(),
            name: "main".to_string(),
            context: ctx,
            body: Lit::mk(1).into(),
            ret_ty: Ty::mk_i64(),
        }
    }
    fn example_def2() -> Def {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        Def {
            span: Span::default(),
            name: "id".to_string(),
            context: ctx,
            body: XVar {
                span: Span::default(),
                var: "x".to_owned(),
                ty: Some(Ty::mk_i64()),
                chi: Some(Prd),
            }
            .into(),
            ret_ty: Ty::mk_i64(),
        }
    }

    fn example_prog1() -> CheckedProgram {
        CheckedProgram {
            defs: vec![],
            data_types: vec![],
            codata_types: vec![],
        }
    }

    fn example_prog2() -> CheckedProgram {
        CheckedProgram {
            defs: vec![example_def1().into(), example_def2().into()],
            data_types: vec![],
            codata_types: vec![],
        }
    }

    #[test]
    fn compile_def1() {
        let result = compile_main(
            example_def1(),
            &[],
            &mut HashSet::from(["main".to_string()]),
        );
        let mut ctx = core_lang::syntax::TypingContext::default();
        ctx.add_covar("a", core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::Def {
            name: "main".to_string(),
            context: ctx,
            body: core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::Literal::new(1),
                core_lang::syntax::terms::Mu::tilde_mu(
                    "x0",
                    core_lang::syntax::Statement::Exit(core_lang::syntax::statements::Exit::exit(
                        core_lang::syntax::terms::XVar::var(
                            "x0",
                            core_lang::syntax::types::Ty::I64,
                        ),
                        core_lang::syntax::types::Ty::I64,
                    )),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["a".to_string(), "x0".to_string()]),
        };
        assert_eq!(result[0].name, expected.name);
        assert_eq!(result[0].context, expected.context);
        assert_eq!(result[0].body, expected.body);
    }

    #[test]
    fn compile_def2() {
        let result = compile_def(example_def2(), &[], &mut HashSet::from(["id".to_string()]));
        let mut ctx = core_lang::syntax::TypingContext::default();
        ctx.add_var("x", core_lang::syntax::types::Ty::I64);
        ctx.add_covar("a0", core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::Def {
            name: "id".to_owned(),
            context: ctx,
            body: core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["x".to_string(), "a0".to_string()]),
        };
        assert_eq!(result[0].name, expected.name);
        assert_eq!(result[0].context, expected.context);
        assert_eq!(result[0].body, expected.body);
    }

    #[test]
    fn compile_prog1() {
        let result = compile_prog(example_prog1());
        assert!(result.defs.is_empty());
        assert!(result.data_types.is_empty());
        assert!(result.codata_types.is_empty());
    }

    #[test]
    fn compile_prog2() {
        let result = compile_prog(example_prog2());
        assert_eq!(result.defs.len(), 2);
        let mut ctx = core_lang::syntax::TypingContext::default();
        ctx.add_covar("a", core_lang::syntax::types::Ty::I64);
        let expected1 = core_lang::syntax::Def {
            name: "main".to_owned(),
            context: ctx,
            body: core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::Literal::new(1),
                core_lang::syntax::terms::Mu::tilde_mu(
                    "x0",
                    core_lang::syntax::Statement::Exit(core_lang::syntax::statements::Exit::exit(
                        core_lang::syntax::terms::XVar::var(
                            "x0",
                            core_lang::syntax::types::Ty::I64,
                        ),
                        core_lang::syntax::types::Ty::I64,
                    )),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["a".to_string(), "x0".to_string()]),
        };
        let mut ctx = core_lang::syntax::TypingContext::default();
        ctx.add_var("x", core_lang::syntax::types::Ty::I64);
        ctx.add_covar("a0", core_lang::syntax::types::Ty::I64);
        let expected2 = core_lang::syntax::Def {
            name: "id".to_owned(),
            context: ctx,
            body: core_lang::syntax::statements::Cut::new(
                core_lang::syntax::terms::XVar::var("x", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::terms::XVar::covar("a0", core_lang::syntax::types::Ty::I64),
                core_lang::syntax::types::Ty::I64,
            )
            .into(),
            used_vars: HashSet::from(["x".to_string(), "a0".to_string()]),
        };

        let def1 = &result.defs[0];
        let def2 = &result.defs[1];

        assert_eq!(def1, &expected1);
        assert_eq!(def2, &expected2);
    }
}
