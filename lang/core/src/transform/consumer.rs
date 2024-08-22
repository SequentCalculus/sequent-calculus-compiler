use crate::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Consumer, Statement},
};
impl NamingTransformation for Consumer {
    type Target = Consumer;
    fn transform(self: Consumer, state: &mut TransformState) -> Consumer {
        match self {
            Consumer::Covariable(covar) => Consumer::Covariable(covar),
            Consumer::MuTilde(mutilde) => mutilde.transform(state).into(),
            Consumer::Case(case) => case.transform(state).into(),
            Consumer::Destructor(destructor) => destructor.transform(state),
        }
    }
}

impl Bind for Consumer {
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        match self {
            Consumer::Covariable(covar) => k(covar.covar, state),
            Consumer::MuTilde(mutilde) => mutilde.bind(k, state),
            Consumer::Case(case) => case.bind(k, state),
            Consumer::Destructor(destructor) => destructor.bind(k, state),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{
            Case, Clause, Consumer, Covariable, Ctor, Destructor, Dtor, Literal, MuTilde, Statement,
        },
    };
    use std::rc::Rc;

    fn example_covar() -> Consumer {
        Covariable {
            covar: "a".to_owned(),
        }
        .into()
    }

    fn example_mutilde() -> MuTilde {
        MuTilde {
            variable: "x".to_owned(),
            statement: Rc::new(Statement::Done()),
        }
    }

    fn example_case() -> Case {
        Case {
            cases: vec![
                Clause {
                    xtor: Ctor::Nil,
                    vars: vec![],
                    covars: vec!["a".to_owned()],
                    rhs: Rc::new(Statement::Done()),
                },
                Clause {
                    xtor: Ctor::Cons,
                    vars: vec!["x".to_owned(), "xs".to_owned()],
                    covars: vec!["a".to_owned()],
                    rhs: Rc::new(Statement::Done()),
                },
            ],
        }
    }

    fn example_dest() -> Destructor {
        Destructor {
            id: Dtor::Ap,
            producers: vec![Literal { lit: 1 }.into()],
            consumers: vec![Covariable {
                covar: "a".to_owned(),
            }
            .into()],
        }
    }

    #[test]
    fn transform_covar() {
        let result = example_covar().transform(&mut Default::default());
        let expected = example_covar();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_mutilde() {
        let result =
            <MuTilde as Into<Consumer>>::into(example_mutilde()).transform(&mut Default::default());
        let expected = example_mutilde().transform(&mut Default::default()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_case() {
        let result =
            <Case as Into<Consumer>>::into(example_case()).transform(&mut Default::default());
        let expected = example_case().transform(&mut Default::default()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_dest() {
        let result =
            <Destructor as Into<Consumer>>::into(example_dest()).transform(&mut Default::default());
        let expected = example_dest().transform(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_covar() {
        let result =
            example_covar().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Statement::Done();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_mutilde() {
        let result = <MuTilde as Into<Consumer>>::into(example_mutilde())
            .bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected =
            example_mutilde().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_case() {
        let result = <Case as Into<Consumer>>::into(example_case())
            .bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected =
            example_case().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_dest() {
        let result = <Destructor as Into<Consumer>>::into(example_dest())
            .bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected =
            example_dest().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        assert_eq!(result, expected)
    }
}
