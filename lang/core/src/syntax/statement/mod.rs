use super::{
    term::{Cns, Prd, Term},
    Covar, Var,
};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt};

pub mod cut;
pub mod fun;
pub mod ifz;
pub mod op;

pub use cut::*;
pub use fun::*;
pub use ifz::*;
pub use op::*;

// Statement
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
    IfZ(IfZ),
    Fun(Fun),
    Done(),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Cut(c) => c.fmt(f),
            Statement::Op(op) => op.fmt(f),
            Statement::IfZ(i) => i.fmt(f),
            Statement::Fun(fun) => fun.fmt(f),
            Statement::Done() => write!(f, "Done"),
        }
    }
}

impl FreeV for Statement {
    fn free_vars(self: &Statement) -> HashSet<Var> {
        match self {
            Statement::Cut(c) => c.free_vars(),
            Statement::Op(op) => op.free_vars(),
            Statement::IfZ(i) => i.free_vars(),
            Statement::Fun(f) => f.free_vars(),
            Statement::Done() => HashSet::new(),
        }
    }
    fn free_covars(self: &Statement) -> HashSet<Covar> {
        match self {
            Statement::Cut(c) => c.free_covars(),
            Statement::Op(op) => op.free_covars(),
            Statement::IfZ(i) => i.free_covars(),
            Statement::Fun(f) => f.free_covars(),
            Statement::Done() => HashSet::new(),
        }
    }
}

impl Subst for Statement {
    type Target = Statement;
    fn subst_sim(
        self: &Statement,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Statement {
        match self {
            Statement::Cut(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Statement::Op(o) => o.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfZ(i) => i.subst_sim(prod_subst, cons_subst).into(),
            Statement::Fun(f) => f.subst_sim(prod_subst, cons_subst).into(),
            Statement::Done() => Statement::Done(),
        }
    }
}

#[cfg(test)]
mod statement_tests {
    use crate::{
        syntax::{
            substitution::SubstitutionBinding,
            term::{Cns, Prd, Term, XVar},
            BinOp, Covar, Statement, Var,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

    use super::{Cut, Fun, IfZ, Op};

    fn example_cut() -> Statement {
        Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_op() -> Statement {
        Op {
            fst: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_ifz() -> Statement {
        IfZ {
            ifc: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            elsec: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_fun() -> Statement {
        Fun {
            name: "main".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
            }
            .into(),
            "x".to_owned(),
        )]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
            }
            .into(),
            "a".to_owned(),
        )]
    }

    #[test]
    fn display_cut() {
        let result = format!("{}", example_cut());
        let expected = "<x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_op() {
        let result = format!("{}", example_op());
        let expected = "*(x, x; 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_ifz() {
        let result = format!("{}", example_ifz());
        let expected = "IfZ(x; <x | 'a>, <x | 'a>)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fun() {
        let result = format!("{}", example_fun());
        let expected = "main(x, 'a)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_done() {
        let result = format!("{}", Statement::Done());
        let expected = "Done".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cut() {
        let result = example_cut().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_op() {
        let result = example_op().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_ifz() {
        let result = example_ifz().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_fun() {
        let result = example_fun().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_done() {
        let result = Statement::Done().free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_cut() {
        let result = example_cut().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_op() {
        let result = example_op().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_ifz() {
        let result = example_ifz().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_fun() {
        let result = example_fun().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_done() {
        let result = Statement::Done().free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cut() {
        let result = example_cut().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                }
                .into(),
            ),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "b".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn subst_op() {
        let result = example_op().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Op {
            fst: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                }
                .into(),
            ),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "b".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_ifz() {
        let result = example_ifz().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = IfZ {
            ifc: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                }
                .into(),
            ),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "y".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "b".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            elsec: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "y".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "b".to_owned(),
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
    fn subst_fun() {
        let result = example_fun().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Fun {
            name: "main".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "y".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "b".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_done() {
        let result = Statement::Done().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Statement::Done();
        assert_eq!(result, expected)
    }
}
