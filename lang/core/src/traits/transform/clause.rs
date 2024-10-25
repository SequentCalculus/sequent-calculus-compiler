use super::{NamingTransformation, TransformState};
use crate::syntax::Clause;

impl NamingTransformation for Clause {
    type Target = Clause;
    ///N(K_i(x_{i,j}; a_{i,j}) => s_i ) = K_i (x_{i,j}; a_{i,j} ) => N(s_i)
    fn transform(self, state: &mut TransformState) -> Clause {
        state.add_context(&self.context);
        Clause {
            xtor: self.xtor,
            context: self.context,
            rhs: self.rhs.transform(state),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::NamingTransformation;
    use crate::syntax::{
        context::ContextBinding,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Clause,
    };
    use std::rc::Rc;

    fn example_clause1() -> Clause {
        Clause {
            xtor: "Tup".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::VarBinding {
                    var: "y".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            rhs: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
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
    }
    fn example_clause2() -> Clause {
        Clause {
            xtor: "Ap".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            rhs: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
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
    }

    #[test]
    fn transform_clause1() {
        let result = example_clause1().transform(&mut Default::default());
        let expected = example_clause1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_clause2() {
        let result = example_clause2().transform(&mut Default::default());
        let expected = example_clause2();
        assert_eq!(result, expected)
    }
}
