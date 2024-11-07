use crate::definition::{CompileState, CompileWithCont};
use core::syntax::term::Cns;
impl CompileWithCont for fun::syntax::terms::Goto {
    /// ```text
    /// 〚goto(t; a) 〛_{c} = 〚t〛_{a}
    /// ```
    fn compile_with_cont(
        self,
        _: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let ty = state.covars.get(&self.target).unwrap().clone();
        self.term.compile_with_cont(
            core::syntax::term::XVar {
                prdcns: Cns,
                var: self.target,
                ty,
            }
            .into(),
            state,
        )
    }
}

#[cfg(test)]
mod compile_tests {

    use fun::{
        parse_term,
        typing::{check::terms::Check, symbol_table::SymbolTable},
    };

    use crate::definition::CompileWithCont;
    use core::syntax::{
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_goto1() {
        let term = parse_term!("goto(1; 'a)");
        let result = term.compile_opt(&mut Default::default(), Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_goto2() {
        let term = parse_term!("label 'a { ifz(x, goto(0;'a), x * 2) }");
        let term_typed = term
            .check(
                &SymbolTable::default(),
                &vec![fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                }],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result = term_typed.compile_opt(&mut Default::default(), Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::IfZ {
                    ifc: Rc::new(
                        core::syntax::term::XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                    thenc: Rc::new(
                        core::syntax::statement::Cut {
                            producer: Rc::new(core::syntax::term::Literal { lit: 0 }.into()),
                            ty: Ty::Int(),
                            consumer: Rc::new(
                                core::syntax::term::XVar {
                                    prdcns: Cns,
                                    var: "a".to_owned(),
                                    ty: Ty::Int(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                    elsec: Rc::new(
                        core::syntax::statement::Op {
                            fst: Rc::new(
                                core::syntax::term::XVar {
                                    prdcns: Prd,
                                    var: "x".to_owned(),
                                    ty: Ty::Int(),
                                }
                                .into(),
                            ),
                            op: core::syntax::BinOp::Prod,
                            snd: Rc::new(core::syntax::term::Literal { lit: 2 }.into()),
                            continuation: Rc::new(
                                core::syntax::term::XVar {
                                    prdcns: Cns,
                                    var: "a".to_owned(),
                                    ty: Ty::Int(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
