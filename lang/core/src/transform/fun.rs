use super::super::{
    naming_transformation::{bind_many, NamingTransformation, TransformState},
    syntax::{Covariable, Fun, Statement, Variable},
};

impl NamingTransformation for Fun {
    type Target = Statement;
    ///N(f(p_i; c_j)) = bind(p_i)[λas.bind(c_j)[λbs.f(as; bs)]]
    fn transform(self, state: &mut TransformState) -> Statement {
        bind_many(
            self.producers.into(),
            Box::new(|vars, state: &mut TransformState| {
                bind_many(
                    self.consumers.into(),
                    Box::new(|covars, _: &mut TransformState| {
                        Fun {
                            name: self.name,
                            producers: vars
                                .into_iter()
                                .map(|var| Variable { var }.into())
                                .collect(),
                            consumers: covars
                                .into_iter()
                                .map(|covar| Covariable { covar }.into())
                                .collect(),
                        }
                        .into()
                    }),
                    state,
                )
            }),
            state,
        )
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{Covariable, Fun, Variable},
    };

    fn example_fun1() -> Fun {
        Fun {
            name: "main".to_owned(),
            producers: vec![],
            consumers: vec![],
        }
    }
    fn example_fun2() -> Fun {
        Fun {
            name: "fun".to_owned(),
            producers: vec![Variable {
                var: "x".to_owned(),
            }
            .into()],
            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
    }

    #[test]
    fn transform_fun1() {
        let result = example_fun1().transform(&mut Default::default());
        let expected = Fun {
            name: "main".to_owned(),
            producers: vec![],
            consumers: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun2() {
        let result = example_fun2().transform(&mut Default::default());
        let expected = Fun {
            name: "fun".to_owned(),
            producers: vec![Variable {
                var: "x".to_owned(),
            }
            .into()],
            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
        .into();
        assert_eq!(result, expected)
    }
}
