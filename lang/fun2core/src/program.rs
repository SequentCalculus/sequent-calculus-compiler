//! This module defines the translation of a typechecked [Fun](fun) program into a
//! [Core](core_lang) program.

use crate::{
    compile::CompileState,
    declaration::{compile_ctor, compile_dtor},
    def::{compile_def, compile_main},
    types::compile_type_params,
};
use core_lang::syntax::{
    names::Identifier,
    type_params::{ParamPolarity, TypeParam},
};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

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
        let type_param_subst = build_type_param_subst(&data.type_params.names(), &type_params);
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
        let type_param_subst = build_type_param_subst(&codata.type_params.names(), &type_params);
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

    let mut used_labels: HashSet<String> = prog.defs.iter().map(|def| def.name.clone()).collect();
    let mut state = CompileState {
        used_vars: HashSet::new(),
        codata_types: &codata_types,
        data_types: &data_types,
        used_labels: &mut used_labels,
        current_label: "",
        lifted_statements: &mut VecDeque::new(),
        max_id: &mut max_id,
    };

    let mut defs_translated = VecDeque::new();
    for def in prog.defs {
        if def.name == "main" {
            for def_main in compile_main(def, &mut state, Rc::new(global_type_param_subst.clone()))
                .into_iter()
                .rev()
            {
                defs_translated.push_front(def_main);
            }
        } else {
            let type_params = compile_type_params(&def.type_params, state.max_id);
            let type_param_subst = build_type_param_subst(&def.type_params.names(), &type_params);

            let mut local_subst = global_type_param_subst.clone();
            local_subst.extend(type_param_subst);

            defs_translated.extend(compile_def(
                def,
                &mut state,
                Rc::new(local_subst),
                type_params,
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
    params: &[TypeParam],
) -> HashMap<String, (Identifier, ParamPolarity)> {
    names
        .iter()
        .cloned()
        .zip(params.iter().map(|p| (p.id.clone(), p.polarity)))
        .collect()
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::CompileState;
    use crate::def::{compile_def, compile_main};
    use crate::program::compile_prog;
    use core_lang::syntax::Identifier;
    use core_lang::syntax::type_params::{ParamPolarity, TypeParam};
    use core_macros::{
        bind, cns, covar, ctor_sig, cut, data, def, exit, id, lit, mutilde, prd, tparam, tvar, ty,
        var,
    };
    use fun::syntax::declarations::Polarity;
    use fun::syntax::type_params::TypeParams;
    use fun::syntax::{
        Chirality,
        declarations::{CtorSig, Data, Def},
        program::CheckedProgram,
        terms::{Lit, XVar},
        types::{Ty, TypeArgs},
        util::dummy_span,
    };

    use std::collections::{HashMap, HashSet, VecDeque};
    use std::rc::Rc;

    fn example_def1() -> Def {
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        Def {
            span: dummy_span(),
            name: "main".to_string(),
            type_params: TypeParams::default(),
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
            type_params: TypeParams::default(),
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
            type_params: TypeParams::mk(&[("A", Polarity::Data)]),
            ctors: vec![
                CtorSig {
                    span: None,
                    name: "Nil".to_string(),
                    type_params: TypeParams::default(),
                    args: fun::syntax::context::TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_string(),
                    type_params: TypeParams::default(),
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
            type_params: TypeParams::default(),
            ctors: vec![
                CtorSig {
                    span: None,
                    name: "Nil".to_string(),
                    type_params: TypeParams::default(),
                    args: fun::syntax::context::TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_string(),
                    type_params: TypeParams::default(),
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
            type_params: TypeParams::mk(&[("A", Polarity::Data)]),
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
            type_params: TypeParams::default(),
            ctors: vec![CtorSig {
                span: None,
                name: "Pack".to_owned(),
                type_params: TypeParams::mk(&[("A", Polarity::Data)]),
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
        let mut state = CompileState {
            used_vars: HashSet::new(),
            codata_types: &[],
            data_types: &[],
            used_labels: &mut HashSet::from(["main".to_string()]),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };

        let result = compile_main(example_def1(), &mut state, Rc::new(HashMap::new()));
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
        let mut state = CompileState {
            used_vars: HashSet::new(),
            codata_types: &[],
            data_types: &[],
            used_labels: &mut HashSet::from(["id".to_string()]),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };
        let result = compile_def(example_def2(), &mut state, Rc::new(HashMap::new()), vec![]);
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
                ctor_sig!(id!("Nil", 2), [], []),
                ctor_sig!(
                    id!("Cons", 3),
                    [],
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))])),
                    ]
                )
            ],
            [tparam!(id!("A", 1), "+")]
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
                ctor_sig!(id!("Nil", 1), [], []),
                ctor_sig!(
                    id!("Cons", 2),
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
        subst.insert("A".to_string(), (fresh_param.clone(), ParamPolarity::Data));

        let mut state = CompileState {
            used_vars: HashSet::new(),
            codata_types: &[],
            data_types: &[],
            used_labels: &mut HashSet::from(["id_poly".to_string()]),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };

        let result = compile_def(
            example_def_poly(),
            &mut state,
            Rc::new(subst),
            vec![TypeParam {
                id: fresh_param.clone(),
                polarity: ParamPolarity::Data,
            }],
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
                id!("Pack", 2),
                [tparam!(id!("A", 1), "+")],
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
