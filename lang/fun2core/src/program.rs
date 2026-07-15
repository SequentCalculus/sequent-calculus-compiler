//! This module defines the translation of a typechecked [Fun](fun) program into a
//! [Core](core_lang) program.

use crate::{
    declaration::{compile_ctor, compile_dtor},
    def::{compile_def, compile_main},
    types::compile_type_params,
};
use core_lang::syntax::names::Identifier;
use std::{collections::HashMap, rc::Rc};

use std::collections::VecDeque;

/// This function translates a typechecked [Fun](fun) program into a [Core](core_lang) program. Additionally, it replaces type parameters in the program with fresh core identifiers.
/// - `program` is the typechecked [Fun](fun) program.
pub fn compile_prog(prog: fun::syntax::program::CheckedProgram) -> core_lang::syntax::Prog {
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
                .map(|ctor| compile_ctor(ctor, Rc::new(type_param_subst.clone()), &mut max_id))
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
                .map(|dtor| compile_dtor(dtor, Rc::new(type_param_subst.clone()), &mut max_id))
                .collect(),
            type_params: type_params.clone(),
        });
    }

    let mut used_labels = prog.defs.iter().map(|def| def.name.clone()).collect();
    let mut defs_translated = VecDeque::new();
    for def in prog.defs {
        if def.name == "main" {
            for def_main in compile_main(
                def,
                codata_types.as_slice(),
                &mut used_labels,
                Rc::new(global_type_param_subst.clone()),
                &mut max_id,
            )
            .into_iter()
            .rev()
            {
                defs_translated.push_front(def_main);
            }
        } else {
            let type_params = compile_type_params(&def.type_params, &mut max_id);
            let type_param_subst = build_type_param_subst(&def.type_params.bindings, &type_params);

            let mut local_subst = global_type_param_subst.clone();
            local_subst.extend(type_param_subst);

            defs_translated.extend(compile_def(
                def,
                codata_types.as_slice(),
                &mut used_labels,
                Rc::new(local_subst),
                type_params,
                &mut max_id,
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

pub fn build_type_param_subst(
    names: &[String],
    params: &[Identifier],
) -> HashMap<String, Identifier> {
    names.iter().cloned().zip(params.iter().cloned()).collect()
}

#[cfg(test)]
mod compile_tests {
    use crate::def::{compile_def, compile_main};
    use crate::program::compile_prog;
    use core_lang::syntax::Identifier;
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

    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    fn example_def1() -> Def {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        Def {
            span: dummy_span(),
            name: "main".to_string(),
            type_params: TypeContext::default(),
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
            type_params: TypeContext::default(),
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
                    type_params: TypeContext::default(),
                    args: fun::syntax::context::TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_string(),
                    type_params: TypeContext::default(),
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
                    type_params: TypeContext::default(),
                    args: fun::syntax::context::TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_string(),
                    type_params: TypeContext::default(),
                    args: cons_ctx,
                },
            ],
        }
    }

    fn example_def_poly() -> Def {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("A", TypeArgs::default()));
        Def {
            span: dummy_span(),
            name: "id_poly".to_string(),
            type_params: TypeContext::mk(&["A"]),
            context: ctx,
            body: XVar {
                span: dummy_span(),
                var: "x".to_owned(),
                ty: Some(Ty::mk_decl("A", TypeArgs::default())),
                chi: Some(Chirality::Prd),
            }
            .into(),
            ret_ty: Ty::mk_decl("A", TypeArgs::default()),
        }
    }

    fn example_box() -> Data {
        Data {
            span: None,
            name: "Box".to_owned(),
            type_params: TypeContext::default(),
            ctors: vec![CtorSig {
                span: None,
                name: "Pack".to_owned(),
                type_params: TypeContext::mk(&["A"]),
                args: {
                    let mut ctx = fun::syntax::context::TypingContext::default();
                    ctx.add_var("x", Ty::mk_decl("A", TypeArgs::default()));
                    ctx
                },
            }],
        }
    }

    #[test]
    fn compile_def1() {
        let result = compile_main(
            example_def1(),
            &[],
            &mut HashSet::from(["main".to_string()]),
            Rc::new(HashMap::new()),
            &mut 0,
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
        let result = compile_def(
            example_def2(),
            &[],
            &mut HashSet::from(["id".to_string()]),
            Rc::new(HashMap::new()),
            vec![],
            &mut 0,
        );
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
        let result = compile_prog(checked);
        assert_eq!(result.data_types.len(), 1);

        // Expected: data List[A_1] with Nil and Cons(x: prd A_1, xs: prd List[A_1])
        let expected = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
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
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!()),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), vec![ty!("int")])),
                    ]
                )
            ],
            []
        );

        assert_eq!(result.data_types[0], expected);
    }

    #[test]
    fn compile_poly_def() {
        let fresh_param = id!("A", 1);
        let mut subst = HashMap::new();
        subst.insert("A".to_string(), fresh_param.clone());

        let result = compile_def(
            example_def_poly(),
            &[],
            &mut HashSet::from(["id_poly".to_string()]),
            Rc::new(subst),
            vec![fresh_param.clone()],
            &mut 0,
        );

        assert_eq!(result.len(), 1);
        let compiled = &result[0];
        compiled.context.bindings.iter().for_each(|binding| {
            if binding.var.name == "x" {
                assert_eq!(binding.ty, tvar!(id!("A", 1)));
            }
        });

        assert_eq!(compiled.name, Identifier::new("id_poly".to_string()));
        assert_eq!(compiled.type_params, vec![fresh_param]);

        assert_eq!(compiled.context.bindings.len(), 2);
    }

    #[test]
    fn compile_prog_poly_def() {
        let checked = CheckedProgram {
            defs: vec![example_def_poly().into()],
            data_types: vec![],
            codata_types: vec![],
        };

        let result = compile_prog(checked);
        assert_eq!(result.defs.len(), 1);

        let compiled_def = &result.defs[0];
        compiled_def.context.bindings.iter().for_each(|binding| {
            if binding.var.name == "x" {
                assert_eq!(binding.ty, tvar!(id!("A", 1)));
            }
        });

        assert_eq!(compiled_def.name, Identifier::new("id_poly".to_string()));
        assert_eq!(compiled_def.type_params.len(), 1);

        assert!(result.max_id > 0);
    }

    #[test]
    fn compile_prog_box() {
        let checked = CheckedProgram {
            defs: vec![],
            data_types: vec![example_box()],
            codata_types: vec![],
        };

        let expected = data!(
            id!("Box"),
            [ctor_sig!(
                id!("Pack"),
                [id!("A", 1)],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            []
        );

        let result = compile_prog(checked);
        assert_eq!(result.data_types.len(), 1);

        let compiled_box = &result.data_types[0];
        assert_eq!(compiled_box, &expected);
    }
}
