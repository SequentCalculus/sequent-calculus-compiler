use crate::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::Clause,
};

impl<T> NamingTransformation for Clause<T> {
    type Target = Clause<T>;
    ///N(K_i(x_{i,j}; a_{i,j}) => s_i ) = K_i (x_{i,j}; a_{i,j} ) => N(s_i)
    fn transform(self, state: &mut TransformState) -> Clause<T> {
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
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{
            context::ContextBinding, types::Ty, Clause, Covariable, Ctor, Cut, Dtor, Variable,
        },
    };
    use std::rc::Rc;

    fn example_clause1() -> Clause<Ctor> {
        Clause {
            xtor: Ctor::Tup,
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
                        Variable {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Covariable {
                            covar: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }
    fn example_clause2() -> Clause<Dtor> {
        Clause {
            xtor: Dtor::Ap,
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
                        Variable {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        Covariable {
                            covar: "a".to_owned(),
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
