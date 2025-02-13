use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core_lang::syntax::term::Cns;

impl CompileWithCont for fun::syntax::terms::Call {
    /// ```text
    /// 〚f(t_1, ...; a_1, ...) 〛_{c} = f(〚t_1〛, ...; a_1, ..., c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let mut new_args = compile_subst(self.args, state);
        new_args.add_cons(cont);
        core_lang::syntax::statement::Call {
            name: self.name,
            args: new_args,
            ty: compile_ty(
                self.ret_ty
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

    use crate::definition::CompileWithCont;
    use std::collections::HashMap;

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
                        ctors: HashMap::new(),
                        dtors: HashMap::new(),
                        funs,
                        types: HashMap::new(),
                        ctor_templates: HashMap::new(),
                        dtor_templates: HashMap::new(),
                        type_templates: HashMap::new(),
                    }
                },
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let mut subst = core_lang::syntax::substitution::Substitution::default();
        subst.add_prod(core_lang::syntax::term::Literal::new(3));
        subst.add_cons(core_lang::syntax::term::XVar::covar(
            "a0",
            core_lang::syntax::types::Ty::I64,
        ));
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::Call {
                name: "fac".to_owned(),
                args: subst,
                ty: core_lang::syntax::types::Ty::I64,
            },
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
