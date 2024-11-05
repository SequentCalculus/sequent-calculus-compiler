use std::rc::Rc;

use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_subst,
};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};
use fun::syntax::substitution::subst_covars;

impl CompileWithCont for fun::syntax::terms::Constructor {
    /// ```text
    /// 〚K(t_1, ...) 〛_{c} = ⟨K( 〚t_1〛, ...) | c⟩
    /// 〚K(t_1, ...) 〛 = K( 〚t_1〛, ...)
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::term::Term<Prd> {
        state.covars.extend(subst_covars(&self.args));
        let ty_name = state.lookup_data(&self.id).unwrap().name;
        core::syntax::term::Xtor {
            prdcns: Prd,
            id: self.id,
            args: compile_subst(self.args, state),
            ty: Ty::Decl(ty_name),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let ty_name = state.lookup_data(&self.id).unwrap().name;
        core::syntax::statement::Cut {
            producer: Rc::new(self.compile_opt(state)),
            ty: Ty::Decl(ty_name),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{
        parse_term,
        typing::{
            check::terms::Check,
            symbol_table::{Polarity, SymbolTable},
        },
    };

    use crate::definition::{CompileState, CompileWithCont};
    use core::syntax::{term::Prd, types::Ty};

    #[test]
    fn compile_cons() {
        let term = parse_term!("Cons(1,Nil)");
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table.ctors.insert("Nil".to_owned(), vec![]);
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            vec![
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                },
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: fun::syntax::types::Ty::mk_decl("ListInt"),
                },
            ],
        );
        let term_typed = term
            .check(
                &symbol_table,
                &vec![],
                &fun::syntax::types::Ty::mk_decl("ListInt"),
            )
            .unwrap();
        let mut state = CompileState::default();
        state
            .data_decls
            .push(core::syntax::declaration::DataDeclaration {
                dat: core::syntax::declaration::Data,
                name: "ListInt".to_owned(),
                xtors: vec![
                    core::syntax::declaration::XtorSig {
                        xtor: core::syntax::declaration::Data,
                        name: "Nil".to_owned(),
                        args: vec![],
                    },
                    core::syntax::declaration::XtorSig {
                        xtor: core::syntax::declaration::Data,
                        name: "Cons".to_owned(),
                        args: vec![
                            core::syntax::context::ContextBinding::VarBinding {
                                var: "x".to_owned(),
                                ty: Ty::Int(),
                            },
                            core::syntax::context::ContextBinding::VarBinding {
                                var: "xs".to_owned(),
                                ty: Ty::Decl("ListInt".to_owned()),
                            },
                        ],
                    },
                ],
            });
        let result = term_typed.compile_opt(&mut state);
        let expected = core::syntax::term::Xtor {
            prdcns: Prd,
            id: "Cons".to_owned(),
            args: vec![
                core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                    prd: core::syntax::term::Literal { lit: 1 }.into(),
                    ty: Ty::Int(),
                },
                core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                    prd: core::syntax::term::Xtor {
                        prdcns: Prd,
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: Ty::Decl("ListInt".to_owned()),
                    }
                    .into(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
            ],
            ty: Ty::Decl("ListInt".to_owned()),
        }
        .into();
        assert_eq!(result, expected)
    }
}
