use crate::syntax::statement::Fun;

use super::super::{
    naming_transformation::{bind_many, NamingTransformation, TransformState},
    syntax::Statement,
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
                    args: args.into_iter().collect(),
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
        syntax::{statement::Fun, substitution::SubstitutionBinding, Covariable, Variable},
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
