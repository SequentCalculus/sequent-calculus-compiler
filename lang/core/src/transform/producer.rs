use crate::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Producer, Statement},
};

impl NamingTransformation for Producer {
    type Target = Producer;
    fn transform(self: Producer, state: &mut TransformState) -> Producer {
        match self {
            Producer::Variable(var) => Producer::Variable(var),
            Producer::Literal(lit) => lit.transform(state).into(),
            Producer::Mu(mu) => mu.transform(state).into(),
            Producer::Constructor(constructor) => constructor.transform(state),
            Producer::Cocase(cocase) => cocase.transform(state).into(),
        }
    }
}

impl Bind for Producer {
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        match self {
            Producer::Variable(var) => k(var.var, state),
            Producer::Literal(lit) => lit.bind(k, state),
            Producer::Mu(mu) => mu.bind(k, state),
            Producer::Constructor(constructor) => constructor.bind(k, state),
            Producer::Cocase(cocase) => cocase.bind(k, state),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{
            context::ContextBinding, types::Ty, Clause, Cocase, Constructor, Literal, Mu, Producer,
            Statement, Variable,
        },
    };
    use std::rc::Rc;

    fn example_var() -> Producer {
        Variable {
            var: "x".to_owned(),
        }
        .into()
    }
    fn example_lit() -> Literal {
        Literal { lit: 1 }.into()
    }
    fn example_mu() -> Mu {
        Mu {
            covariable: "a".to_owned(),
            statement: Rc::new(Statement::Done()),
        }
    }

    fn example_cons() -> Constructor {
        Constructor {
            id: "Nil".to_owned(),
            args: vec![],
        }
    }

    fn example_cocase() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: "Hd".to_owned(),
                    context: vec![ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    }],
                    rhs: Rc::new(Statement::Done()),
                },
                Clause {
                    xtor: "Tl".to_owned(),
                    context: vec![ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    }],
                    rhs: Rc::new(Statement::Done()),
                },
            ],
        }
        .into()
    }

    #[test]
    fn transform_var() {
        let result = example_var().transform(&mut Default::default());
        let expected = example_var();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_lit() {
        let result =
            <Literal as Into<Producer>>::into(example_lit()).transform(&mut Default::default());
        let expected = example_lit().transform(&mut Default::default()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_mu() {
        let result = <Mu as Into<Producer>>::into(example_mu()).transform(&mut Default::default());
        let expected = example_mu().transform(&mut Default::default()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_cons() {
        let result = <Constructor as Into<Producer>>::into(example_cons())
            .transform(&mut Default::default());
        let expected = example_cons().transform(&mut Default::default()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_cocase() {
        let result =
            <Cocase as Into<Producer>>::into(example_cocase()).transform(&mut Default::default());
        let expected = example_cocase().transform(&mut Default::default()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_var() {
        let result =
            example_var().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Statement::Done();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit() {
        let result = <Literal as Into<Producer>>::into(example_lit())
            .bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected =
            example_lit().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_mu() {
        let result = <Mu as Into<Producer>>::into(example_mu())
            .bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected =
            example_mu().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_cons() {
        let result = <Constructor as Into<Producer>>::into(example_cons())
            .bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected =
            example_cons().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_cocase() {
        let result = <Cocase as Into<Producer>>::into(example_cocase())
            .bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected =
            example_cocase().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        assert_eq!(result, expected)
    }
}
