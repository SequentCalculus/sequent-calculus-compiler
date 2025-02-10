use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_ty,
};
use core_lang::syntax::term::Cns;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Let {
    /// ```text
    /// 〚let x := t_1; t_2 〛_{c} = <〚t_1 〛| μ~x.〚t_2 〛_{c}> if t_1: codata {...}
    /// 〚let x := t_1; t_2 〛_{c} = 〚t_1 〛_{μ~x.〚t_2 〛_{c}}
    /// ```
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core_lang::syntax::Statement {
        let ty = compile_ty(self.var_ty);
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core_lang::syntax::term::Mu {
            prdcns: Cns,
            variable: self.variable,
            ty: ty.clone(),
            statement: Rc::new(self.in_term.compile_with_cont(cont, state)),
        }
        .into();

        if ty.is_codata(state.codata_types) {
            // <〚t_1 〛| new_cont>
            core_lang::syntax::statement::Cut {
                producer: Rc::new(self.bound_term.compile_opt(state, ty.clone())),
                ty,
                consumer: Rc::new(new_cont),
            }
            .into()
        } else {
            // 〚t_1 〛_{new_cont}
            self.bound_term.compile_with_cont(new_cont, state)
        }
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{parse_term, test_common::symbol_table_list, typing::check::Check};

    use crate::definition::CompileWithCont;

    #[test]
    fn compile_let1() {
        let term = parse_term!("let x : i64 = 1; x * x");
        let term_typed = term
            .check(
                &mut Default::default(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();
        let result =
            term_typed.compile_opt(&mut Default::default(), core_lang::syntax::types::Ty::I64);
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::Literal::new(1),
                core_lang::syntax::term::Mu::tilde_mu(
                    "x",
                    core_lang::syntax::statement::Op::prod(
                        core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::I64),
                        core_lang::syntax::term::XVar::var("x", core_lang::syntax::types::Ty::I64),
                        core_lang::syntax::term::XVar::covar(
                            "a0",
                            core_lang::syntax::types::Ty::I64,
                        ),
                    ),
                    core_lang::syntax::types::Ty::I64,
                ),
                core_lang::syntax::types::Ty::I64,
            ),
            core_lang::syntax::types::Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_let2() {
        let term = parse_term!("let x : List[i64] = Cons(x,Nil); x");
        let mut ctx = fun::syntax::context::TypingContext::default();
        ctx.add_var("x", fun::syntax::types::Ty::mk_i64());
        let term_typed = term
            .check(
                &mut symbol_table_list(),
                &ctx,
                &fun::syntax::types::Ty::mk_decl(
                    "List",
                    fun::syntax::types::TypeArgs::mk(vec![fun::syntax::types::Ty::mk_i64()]),
                ),
            )
            .unwrap();
        let result = term_typed.compile_opt(
            &mut Default::default(),
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        );
        let mut subst = core_lang::syntax::substitution::Substitution::default();
        subst.add_prod(core_lang::syntax::term::XVar::var(
            "x",
            core_lang::syntax::types::Ty::I64,
        ));
        subst.add_prod(core_lang::syntax::term::Xtor::ctor(
            "Nil",
            core_lang::syntax::substitution::Substitution::default(),
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        ));
        let mut subst = core_lang::syntax::substitution::Substitution::default();
        subst.add_prod(core_lang::syntax::term::XVar::var(
            "x",
            core_lang::syntax::types::Ty::I64,
        ));
        subst.add_prod(core_lang::syntax::term::Xtor::ctor(
            "Nil",
            core_lang::syntax::substitution::Substitution::default(),
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        ));
        let expected = core_lang::syntax::term::Mu::mu(
            "a0",
            core_lang::syntax::statement::Cut::new(
                core_lang::syntax::term::Xtor::ctor(
                    "Cons",
                    subst,
                    core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
                ),
                core_lang::syntax::term::Mu::tilde_mu(
                    "x",
                    core_lang::syntax::statement::Cut::new(
                        core_lang::syntax::term::XVar::var(
                            "x",
                            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
                        ),
                        core_lang::syntax::term::XVar::covar(
                            "a0",
                            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
                        ),
                        core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
                    ),
                    core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
                ),
                core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
            ),
            core_lang::syntax::types::Ty::Decl("List[i64]".to_owned()),
        )
        .into();
        assert_eq!(result, expected)
    }
}
