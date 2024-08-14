use crate::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::Clause,
};

impl<T> NamingTransformation for Clause<T> {
    type Target = Clause<T>;
    ///N (Ki (xi,j ; αi,j ) ⇒ si ) = Ki (x i,j ; αi,j ) ⇒ N (si )
    fn transform(self, st: &mut TransformState) -> Clause<T> {
        Clause {
            xtor: self.xtor,
            vars: self.vars,
            covars: self.covars,
            rhs: self.rhs.transform(st),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{Clause, Covariable, Ctor, Cut, Dtor, Variable},
    };
    use std::rc::Rc;

    fn example_clause1() -> Clause<Ctor> {
        Clause {
            xtor: Ctor::Tup,
            vars: vec!["x".to_owned(), "y".to_owned()],
            covars: vec!["a".to_owned()],
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
            vars: vec!["x".to_owned()],
            covars: vec!["a".to_owned()],
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
