//! This module defines the translation for the call of a top-level function.

use crate::{
    arguments::compile_subst,
    compile::{Compile, CompileState},
    types::compile_ty,
};
use core_lang::syntax::terms::Cns;

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
    ) -> core_lang::syntax::Statement {
        let mut new_args = compile_subst(self.args, state);
        new_args.add_cons(cont);
        core_lang::syntax::statements::Call {
            name: self.name,
            args: new_args,
            ty: compile_ty(
                &self
                    .ret_ty
                    .expect("Types should be annotated before translation"),
            ),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{
        parse_term,
        syntax::context::TypingContext,
        typing::{check::Check, symbol_table::SymbolTable},
    };

    use crate::compile::{Compile, CompileState};
    use std::collections::{HashMap, HashSet, VecDeque};

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
                        def_templates: HashMap::default(),
                        instantiated_defs: Vec::new(),
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
        let result = term_typed.compile(&mut state, core_lang::syntax::types::Ty::I64);

        let mut arguments = core_lang::syntax::arguments::Arguments::default();
        arguments.add_prod(core_lang::syntax::terms::Literal::new(3));
        arguments.add_cons(core_lang::syntax::terms::XVar::covar(
            "a0",
            core_lang::syntax::types::Ty::I64,
        ));
        let expected = core_lang::syntax::terms::Mu::mu(
            "a0",
            core_lang::syntax::statements::Call {
                name: "fac".to_owned(),
                args: arguments,
                ty: core_lang::syntax::types::Ty::I64,
            },
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
