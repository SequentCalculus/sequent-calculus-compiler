use super::super::{
    naming_transformation::{bind_many, NameBind, NamingTransformation, TransformState},
    syntax::{substitution::SubstitutionBinding, Covariable, Fun, Statement, Variable},
};

impl NamingTransformation for Fun {
    type Target = Statement;
    ///N(f(p_i; c_j)) = bind(p_i)[λas.bind(c_j)[λbs.f(as; bs)]]
    fn transform(self, state: &mut TransformState) -> Statement {
        bind_many(
            self.args.into(),
            Box::new(|args, _: &mut TransformState| {
                Fun {
                    name: self.name,
                    //same problem as with constructors
                    args: args
                        .into_iter()
                        .map(|var| match var {
                            NameBind::Var(v) => {
                                SubstitutionBinding::ProducerBinding(Variable { var: v }.into())
                            }
                            NameBind::Covar(cv) => SubstitutionBinding::ConsumerBinding(
                                Covariable { covar: cv }.into(),
                            ),
                        })
                        .collect(),
                }
                .into()
            }),
            state,
        )
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::NamingTransformation,
        syntax::{substitution::SubstitutionBinding, Covariable, Fun, Variable},
    };

    fn example_fun1() -> Fun {
        Fun {
            name: "main".to_owned(),
            args: vec![],
        }
    }
    fn example_fun2() -> Fun {
        Fun {
            name: "fun".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    Covariable {
                        covar: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
    }

    #[test]
    fn transform_fun1() {
        let result = example_fun1().transform(&mut Default::default());
        let expected = Fun {
            name: "main".to_owned(),
            args: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun2() {
        let result = example_fun2().transform(&mut Default::default());
        let expected = Fun {
            name: "fun".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    Variable {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    Covariable {
                        covar: "a".to_owned(),
                    }
                    .into(),
                ),
            ],
        }
        .into();
        assert_eq!(result, expected)
    }
}
