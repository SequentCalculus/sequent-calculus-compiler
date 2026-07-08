//! This module defines the translation for the call of a top-level function.

use std::{collections::HashMap, rc::Rc};

use crate::{
    arguments::compile_subst,
    compile::{Compile, CompileState},
    types::compile_ty_poly,
};
use core_lang::syntax::{names::Identifier, terms::Cns};

impl Compile for fun::syntax::terms::Call {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚f(t_1, ..., a_1, ...) 〛_{c} = f(〚t_1〛, ..., a_1, ..., c)
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
        type_params: Rc<HashMap<String, Identifier>>,
    ) -> core_lang::syntax::Statement {
        let mut args = compile_subst(self.args, state, type_params.clone());
        args.entries.push(cont.into());
        core_lang::syntax::statements::Call {
            name: Identifier::new(self.name),
            type_args: core_lang::syntax::types::TypeArgs {
                args: self
                    .type_args
                    .args
                    .iter()
                    .map(|arg| compile_ty_poly(arg, type_params.clone()))
                    .collect::<Vec<_>>(),
            },
            args,
            ty: compile_ty_poly(
                &self
                    .ret_ty
                    .expect("Types should be annotated before translation"),
                type_params,
            ),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_macros::{call, covar, id, lit, mu, tvar, ty};
    use fun::{
        parse_term,
        syntax::context::TypingContext,
        typing::{check::Check, symbol_table::SymbolTable},
    };
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        rc::Rc,
    };

    #[test]
    fn compile_fac() {
        let term = parse_term!("fac(3)");
        let mut ctx = TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut {
                    let mut funs = HashMap::new();
                    funs.insert("fac".to_owned(), (ctx, fun::syntax::types::Ty::mk_i64()));

                    SymbolTable {
                        ctors: HashMap::default(),
                        dtors: HashMap::default(),
                        defs: funs,
                        types: HashMap::default(),
                        ctor_templates: HashMap::default(),
                        dtor_templates: HashMap::default(),
                        type_templates: HashMap::default(),
                    }
                },
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::from(["fac".to_string()]),
            current_label: "fac",
            lifted_statements: &mut VecDeque::default(),
        };
        let result = term_typed.compile(&mut state, ty!("int"), Rc::default());

        let expected = mu!(id!("a0"), call!(id!("fac"), [lit!(3), covar!(id!("a0"))])).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_poly_call_concrete() {
        let term = parse_term!("id[i64](42)");
        let mut ctx = TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());

        let mut defs = HashMap::new();
        defs.insert("id".to_owned(), (ctx, fun::syntax::types::Ty::mk_i64()));

        let term_typed = term
            .check(
                &mut SymbolTable {
                    ctors: HashMap::default(),
                    dtors: HashMap::default(),
                    defs,
                    types: HashMap::default(),
                    ctor_templates: HashMap::default(),
                    dtor_templates: HashMap::default(),
                    type_templates: HashMap::default(),
                },
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::from(["id".to_string()]),
            current_label: "main",
            lifted_statements: &mut VecDeque::default(),
        };

        let continuation = core_lang::syntax::terms::XVar::covar(id!("a0"), ty!("int")).into();
        let result = match term_typed {
            fun::syntax::terms::Term::Call(call_node) => {
                call_node.compile_with_cont(continuation, &mut state, Rc::default())
            }
            _ => panic!("Expected a Call node after parsing"),
        };

        if let core_lang::syntax::Statement::Call(compiled_call) = result {
            assert_eq!(compiled_call.name, id!("id"));
            assert_eq!(compiled_call.type_args.args.len(), 1);
            assert_eq!(compiled_call.type_args.args[0], ty!("int"));
        } else {
            panic!("Expected a core Call statement");
        }
    }

    #[test]
    fn compile_poly_call_with_context_substitution() {
        // simulate a polymorphic call where the type parameter "A" is substituted with a concrete type (e.g., i64) during compilation.
        let term = parse_term!("id[i64](42)");
        let mut ctx = TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());

        let mut defs = HashMap::new();
        defs.insert("id".to_owned(), (ctx, fun::syntax::types::Ty::mk_i64()));

        let term_typed = term
            .check(
                &mut SymbolTable {
                    ctors: HashMap::default(),
                    dtors: HashMap::default(),
                    defs,
                    types: HashMap::default(),
                    ctor_templates: HashMap::default(),
                    dtor_templates: HashMap::default(),
                    type_templates: HashMap::default(),
                },
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut call_node = match term_typed {
            fun::syntax::terms::Term::Call(c) => c,
            _ => panic!("Expected a Call node after parsing"),
        };

        // Set the type arguments and return type for the call node to simulate a polymorphic call with a concrete type substitution.
        let generic_ty =
            fun::syntax::types::Ty::mk_decl("A", fun::syntax::types::TypeArgs::default());
        call_node.type_args = fun::syntax::types::TypeArgs::mk(vec![generic_ty.clone()]);
        call_node.ret_ty = Some(generic_ty);

        let mut state = CompileState {
            used_vars: HashSet::from(["x".to_string()]),
            codata_types: &[],
            used_labels: &mut HashSet::from(["id".to_string()]),
            current_label: "foo",
            lifted_statements: &mut VecDeque::default(),
        };

        // Simulate the type parameter substitution that would normally occur in compile_type_params.
        let mut type_params_subst = HashMap::new();
        let fresh_target_id = id!("A", 42);
        type_params_subst.insert("A".to_string(), fresh_target_id.clone());
        let type_params_rc = Rc::new(type_params_subst);

        let continuation =
            core_lang::syntax::terms::XVar::covar(id!("a0"), tvar!(id!("A", 42))).into();

        let result = call_node.compile_with_cont(continuation, &mut state, type_params_rc);
        if let core_lang::syntax::Statement::Call(compiled_call) = result {
            assert_eq!(compiled_call.type_args.args.len(), 1);
            assert_eq!(compiled_call.type_args.args[0], tvar!(id!("A", 42)));
        } else {
            panic!("Expected a core Call statement");
        }
    }
}
