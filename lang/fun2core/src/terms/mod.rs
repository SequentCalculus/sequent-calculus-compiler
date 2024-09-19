use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

pub mod case;
pub mod cocase;
pub mod constructor;
pub mod destructor;
pub mod fun_call;
pub mod goto;
pub mod idents;
pub mod ifz;
pub mod label;
pub mod let_exp;
pub mod op;
pub mod paren;

impl CompileWithCont for fun::syntax::terms::Term {
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        match self {
            fun::syntax::terms::Term::Var(v) => core::syntax::Variable { var: v.var }.into(),
            fun::syntax::terms::Term::Lit(n) => core::syntax::Literal { lit: n.val }.into(),
            fun::syntax::terms::Term::Op(op) => op.compile_opt(state),
            fun::syntax::terms::Term::IfZ(ifz) => ifz.compile_opt(state),
            fun::syntax::terms::Term::Let(lt) => lt.compile_opt(state),
            fun::syntax::terms::Term::Fun(fun) => fun.compile_opt(state),
            fun::syntax::terms::Term::Constructor(cons) => cons.compile_opt(state),
            fun::syntax::terms::Term::Destructor(dest) => dest.compile_opt(state),
            fun::syntax::terms::Term::Case(case) => case.compile_opt(state),
            fun::syntax::terms::Term::Cocase(cocase) => cocase.compile_opt(state),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_opt(state),
            fun::syntax::terms::Term::Label(label) => label.compile_opt(state),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_opt(state),
        }
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        match self {
            fun::syntax::terms::Term::Var(v) => {
                let new_var: core::syntax::Producer = core::syntax::Variable { var: v.var }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_var),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::terms::Term::Lit(n) => {
                let new_lit: core::syntax::Producer = core::syntax::Literal { lit: n.val }.into();
                core::syntax::Cut {
                    producer: Rc::new(new_lit),
                    consumer: Rc::new(cont),
                }
                .into()
            }
            fun::syntax::terms::Term::Op(op) => op.compile_with_cont(cont, state),
            fun::syntax::terms::Term::IfZ(ifz) => ifz.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Let(lt) => lt.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Fun(fun) => fun.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Constructor(cons) => cons.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Destructor(dest) => dest.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Case(case) => case.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Cocase(cocase) => cocase.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Goto(goto) => goto.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Label(label) => label.compile_with_cont(cont, state),
            fun::syntax::terms::Term::Paren(paren) => paren.compile_with_cont(cont, state),
        }
    }
}

#[cfg(test)]
mod compile_tests {

    use crate::definition::CompileWithCont;
    use fun::syntax::terms::{
        Case, Clause, Cocase, Constructor, Destructor, Fun, Goto, IfZ, Label, Let, Lit, Op, Paren,
        Term, Var,
    };
    use fun::syntax::{BinOp, Ctor, Dtor};
    use std::rc::Rc;

    fn example_var() -> Term {
        Var {
            var: "x".to_owned(),
        }
        .into()
    }

    fn example_lit() -> Term {
        Lit { val: 1 }.into()
    }

    fn example_op() -> Op {
        Op {
            fst: Rc::new(Lit { val: 1 }.into()),
            op: BinOp::Sum,
            snd: Rc::new(Lit { val: 2 }.into()),
        }
    }

    fn example_ifz() -> IfZ {
        IfZ {
            ifc: Rc::new(Lit { val: 0 }.into()),
            thenc: Rc::new(
                Var {
                    var: "x".to_owned(),
                }
                .into(),
            ),
            elsec: Rc::new(
                Var {
                    var: "y".to_owned(),
                }
                .into(),
            ),
        }
    }

    fn example_let() -> Let {
        Let {
            variable: "x".to_owned(),
            bound_term: Rc::new(Lit { val: 1 }.into()),
            in_term: Rc::new(
                Var {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    fn example_fun() -> Fun {
        Fun {
            name: "mult".to_owned(),
            args: vec![Var {
                var: "x".to_owned(),
            }
            .into()],
        }
    }

    fn example_ctor() -> Constructor {
        Constructor {
            id: Ctor::Nil,
            args: vec![],
        }
    }

    fn example_dtor() -> Destructor {
        Destructor {
            id: Dtor::Hd,
            args: vec![],
            destructee: Rc::new(
                Var {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    fn example_case() -> Case {
        Case {
            cases: vec![Clause {
                xtor: Ctor::Nil,
                context: vec![],
                rhs: Lit { val: 1 }.into(),
            }],
            destructee: Rc::new(
                Var {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    fn example_cocase() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: Dtor::Fst,
                    context: vec![],
                    rhs: Lit { val: 1 }.into(),
                },
                Clause {
                    xtor: Dtor::Snd,
                    context: vec![],
                    rhs: Lit { val: 2 }.into(),
                },
            ],
        }
    }

    fn example_goto() -> Goto {
        Goto {
            term: Rc::new(Lit { val: 1 }.into()),
            target: "a".to_owned(),
        }
    }

    fn example_label() -> Label {
        Label {
            label: "a".to_owned(),
            term: Rc::new(Lit { val: 1 }.into()),
        }
    }
    fn example_paren() -> Paren {
        Paren {
            inner: Rc::new(Lit { val: 1 }.into()),
        }
    }

    #[test]
    fn compile_var() {
        let result = example_var().compile_opt(&mut Default::default());
        let expected = core::syntax::Variable {
            var: "x".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_lit() {
        let result = example_lit().compile_opt(&mut Default::default());
        let expected = core::syntax::Literal { lit: 1 }.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_op() {
        let result = <Op as Into<Term>>::into(example_op()).compile_opt(&mut Default::default());
        let expected = example_op().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ifz() {
        let result = <IfZ as Into<Term>>::into(example_ifz()).compile_opt(&mut Default::default());
        let expected = example_ifz().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_let() {
        let result = <Let as Into<Term>>::into(example_let()).compile_opt(&mut Default::default());
        let expected = example_let().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_let() {
        let result = <Let as Into<Term>>::into(example_let()).compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = example_let().compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_fun() {
        let result = <Fun as Into<Term>>::into(example_fun()).compile_opt(&mut Default::default());
        let expected = example_fun().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_ctor() {
        let result =
            <Constructor as Into<Term>>::into(example_ctor()).compile_opt(&mut Default::default());
        let expected = example_ctor().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_dtor() {
        let result =
            <Destructor as Into<Term>>::into(example_dtor()).compile_opt(&mut Default::default());
        let expected = example_dtor().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_dtor() {
        let result = <Destructor as Into<Term>>::into(example_dtor()).compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = example_dtor().compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_case() {
        let result =
            <Case as Into<Term>>::into(example_case()).compile_opt(&mut Default::default());
        let expected = example_case().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_cocase() {
        let result =
            <Cocase as Into<Term>>::into(example_cocase()).compile_opt(&mut Default::default());
        let expected = example_cocase().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_goto() {
        let result =
            <Goto as Into<Term>>::into(example_goto()).compile_opt(&mut Default::default());
        let expected = example_goto().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_label() {
        let result =
            <Label as Into<Term>>::into(example_label()).compile_opt(&mut Default::default());
        let expected = example_label().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_label() {
        let result = <Label as Into<Term>>::into(example_label()).compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = example_label().compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_paren() {
        let result =
            <Paren as Into<Term>>::into(example_paren()).compile_opt(&mut Default::default());
        let expected = example_paren().compile_opt(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_paren() {
        let result = <Paren as Into<Term>>::into(example_paren()).compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = example_paren().compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        assert_eq!(result, expected)
    }
}
