//! This module defines the translation of a typechecked [Fun](fun) program into a
//! [Core](core_lang) program.

use crate::{
    declaration::{compile_ctor, compile_ctor_poly, compile_dtor, compile_dtor_poly},
    def::{compile_def, compile_def_poly, compile_main, compile_main_poly},
    types::compile_type_params,
};
use core_lang::syntax::names::Identifier;
use std::{collections::HashMap, rc::Rc};

use std::collections::VecDeque;

/// This function translates a typechecked [Fun](fun) program into a [Core](core_lang) program.
/// - `program` is the typechecked [Fun](fun) program.
pub fn compile_prog(prog: fun::syntax::program::CheckedProgram) -> core_lang::syntax::Prog {
    let mut data_types = Vec::new();
    let mut codata_types = Vec::new();

    for data in prog.data_types {
        data_types.push(core_lang::syntax::declaration::TypeDeclaration {
            dat: core_lang::syntax::declaration::Data,
            name: Identifier::new(data.name),
            xtors: data.ctors.into_iter().map(compile_ctor).collect(),
            type_params: vec![],
        });
    }
    for codata in prog.codata_types {
        codata_types.push(core_lang::syntax::declaration::TypeDeclaration {
            dat: core_lang::syntax::declaration::Codata,
            name: Identifier::new(codata.name),
            xtors: codata.dtors.into_iter().map(compile_dtor).collect(),
            type_params: vec![],
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
        max_id: 0,
    }
}

/// This function translates a typechecked [Fun](fun) program into a [Core](core_lang) program. Additionally, it replaces type parameters in the program with fresh core identifiers.
/// - `program` is the typechecked [Fun](fun) program.
pub fn compile_prog_poly(prog: fun::syntax::program::CheckedProgram) -> core_lang::syntax::Prog {
    let mut data_types = Vec::new();
    let mut codata_types = Vec::new();
    let mut max_id = 0;
    let mut global_type_param_subst = HashMap::new();

    for data in prog.data_types {
        let type_params = compile_type_params(&data.type_params, &mut max_id);
        let type_param_subst = build_type_param_subst(&data.type_params.bindings, &type_params);
        global_type_param_subst.extend(type_param_subst.clone());

        data_types.push(core_lang::syntax::declaration::TypeDeclaration {
            dat: core_lang::syntax::declaration::Data,
            name: Identifier::new(data.name),
            xtors: data
                .ctors
                .into_iter()
                .map(|ctor| compile_ctor_poly(ctor, Rc::new(type_param_subst.clone())))
                .collect(),
            type_params: type_params.clone(),
        });
    }
    for codata in prog.codata_types {
        let type_params = compile_type_params(&codata.type_params, &mut max_id);
        let type_param_subst = build_type_param_subst(&codata.type_params.bindings, &type_params);
        global_type_param_subst.extend(type_param_subst.clone());
        codata_types.push(core_lang::syntax::declaration::TypeDeclaration {
            dat: core_lang::syntax::declaration::Codata,
            name: Identifier::new(codata.name),
            xtors: codata
                .dtors
                .into_iter()
                .map(|dtor| compile_dtor_poly(dtor, Rc::new(type_param_subst.clone())))
                .collect(),
            type_params: type_params.clone(),
        });
    }

    let mut used_labels = prog.defs.iter().map(|def| def.name.clone()).collect();
    let mut defs_translated = VecDeque::new();
    for def in prog.defs {
        if def.name == "main" {
            for def_main in compile_main_poly(
                def,
                codata_types.as_slice(),
                &mut used_labels,
                Rc::new(global_type_param_subst.clone()),
            )
            .into_iter()
            .rev()
            {
                defs_translated.push_front(def_main);
            }
        } else {
            defs_translated.extend(compile_def_poly(
                def,
                codata_types.as_slice(),
                &mut used_labels,
                Rc::new(global_type_param_subst.clone()),
            ));
        }
    }

    core_lang::syntax::Prog {
        defs: defs_translated.into(),
        data_types,
        codata_types,
        max_id,
    }
}

fn build_type_param_subst(names: &[String], params: &[Identifier]) -> HashMap<String, Identifier> {
    names.iter().cloned().zip(params.iter().cloned()).collect()
}

#[cfg(test)]
mod compile_tests {
    use crate::{
        def::{compile_def, compile_main},
        program::compile_prog,
        program::compile_prog_poly,
    };
    use core_macros::{
        bind, cns, covar, ctor_sig, cut, data, def, exit, id, lit, mutilde, prd, tvar, ty, var,
    };
    use fun::syntax::context::TypeContext;
    use fun::syntax::{
        Chirality,
        declarations::{CtorSig, Data, Def},
        program::CheckedProgram,
        terms::{Lit, XVar},
        types::{Ty, TypeArgs},
        util::dummy_span,
    };
    use std::collections::HashSet;

    fn example_def1() -> Def {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        Def {
            span: dummy_span(),
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
            span: dummy_span(),
            name: "id".to_string(),
            context: ctx,
            body: XVar {
                span: dummy_span(),
                var: "x".to_owned(),
                ty: Some(Ty::mk_i64()),
                chi: Some(Chirality::Prd),
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

    fn example_list_poly() -> Data {
        let mut cons_ctx = fun::syntax::context::TypingContext::default();
        cons_ctx.add_var("x", Ty::mk_decl("A", TypeArgs::default()));
        cons_ctx.add_var(
            "xs",
            Ty::mk_decl(
                "List",
                TypeArgs::mk(vec![Ty::mk_decl("A", TypeArgs::default())]),
            ),
        );

        Data {
            span: None,
            name: "List".to_string(),
            type_params: TypeContext::mk(&["A"]),
            ctors: vec![
                CtorSig {
                    span: None,
                    name: "Nil".to_string(),
                    args: fun::syntax::context::TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_string(),
                    args: cons_ctx,
                },
            ],
        }
    }

    fn example_list_mono() -> Data {
        let mut cons_ctx = fun::syntax::context::TypingContext::default();
        cons_ctx.add_var("x", Ty::mk_i64());
        cons_ctx.add_var("xs", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));

        Data {
            span: None,
            name: "List[i64]".to_string(),
            type_params: TypeContext::default(),
            ctors: vec![
                CtorSig {
                    span: None,
                    name: "Nil".to_string(),
                    args: fun::syntax::context::TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_string(),
                    args: cons_ctx,
                },
            ],
        }
    }

    #[test]
    fn compile_def1() {
        let result = compile_main(
            example_def1(),
            &[],
            &mut HashSet::from(["main".to_string()]),
        );
        let expected = def!(
            id!("main"),
            [bind!(id!("a"), cns!())],
            cut!(lit!(1), mutilde!(id!("x0"), exit!(var!(id!("x0"))))),
        );

        assert_eq!(result[0].name, expected.name);
        assert_eq!(result[0].context, expected.context);
        assert_eq!(result[0].body, expected.body);
    }

    #[test]
    fn compile_def2() {
        let result = compile_def(example_def2(), &[], &mut HashSet::from(["id".to_string()]));
        let expected = def!(
            id!("id"),
            [bind!(id!("x"), prd!()), bind!(id!("a0"), cns!())],
            cut!(var!(id!("x")), covar!(id!("a0"))),
        );
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
        let expected1 = def!(
            id!("main"),
            [bind!(id!("a"), cns!())],
            cut!(lit!(1), mutilde!(id!("x0"), exit!(var!(id!("x0"))))),
        );
        let expected2 = def!(
            id!("id"),
            [bind!(id!("x"), prd!()), bind!(id!("a0"), cns!())],
            cut!(var!(id!("x")), covar!(id!("a0"))),
        );

        let def1 = &result.defs[0];
        let def2 = &result.defs[1];

        assert_eq!(def1, &expected1);
        assert_eq!(def2, &expected2);
    }

    #[test]
    fn compile_prog_poly_substitutes_type_params_in_xtor_args() {
        let checked = CheckedProgram {
            defs: vec![],
            data_types: vec![example_list_poly()],
            codata_types: vec![],
        };
        let result = compile_prog_poly(checked);
        assert_eq!(result.data_types.len(), 1);

        // Expected: data List[A_1] with Nil and Cons(x: prd A_1, xs: prd List[A_1])
        let expected = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))])),
                    ]
                )
            ],
            [id!("A", 1)]
        );

        assert_eq!(result.data_types[0], expected);
    }

    #[test]
    fn compile_prog_mono_keeps_monomorphic_types() {
        let checked = CheckedProgram {
            defs: vec![],
            data_types: vec![example_list_mono()],
            codata_types: vec![],
        };
        let result = compile_prog(checked);
        assert_eq!(result.data_types.len(), 1);

        // Expected: monomorphic List[i64]
        let expected = data!(
            id!("List[i64]"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!()),
                        bind!(id!("xs"), prd!(), ty!(id!("List[i64]"))),
                    ]
                )
            ],
            []
        );

        assert_eq!(result.data_types[0], expected);
    }
}
