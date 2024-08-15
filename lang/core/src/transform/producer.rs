use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Name, Producer, Statement},
};

impl NamingTransformation for Producer {
    type Target = Producer;
    fn transform(self: Producer, st: &mut TransformState) -> Producer {
        match self {
            Producer::Variable(var) => Producer::Variable(var),
            Producer::Literal(lit) => lit.transform(st).into(),
            Producer::Mu(mu) => mu.transform(st).into(),
            Producer::Constructor(cons) => cons.transform(st),
            Producer::Cocase(cocase) => cocase.transform(st).into(),
        }
    }
}

impl Bind for Producer {
    fn bind<F, K>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: FnOnce(Name) -> K,
        K: FnOnce(&mut TransformState) -> Statement,
    {
        match self {
            Producer::Variable(var) => k(var.var)(st),
            Producer::Literal(lit) => lit.bind(k, st),
            Producer::Mu(mu) => mu.bind(k, st),
            Producer::Constructor(cons) => cons.bind(k, st),
            Producer::Cocase(cocase) => cocase.bind(k, st),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{Clause, Cocase, Dtor, Literal, Mu, Producer, Statement, Variable},
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
    /* tests will only work once constructors are fully implemented tests will only work once
      fn example_cons() -> Constructor {
        Constructor {
            id: Ctor::Nil,
            producers: vec![],
            consumers: vec![],
        }
    }*/
    fn example_cocase() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: Dtor::Hd,
                    vars: vec![],
                    covars: vec!["a".to_owned()],
                    rhs: Rc::new(Statement::Done()),
                },
                Clause {
                    xtor: Dtor::Tl,
                    vars: vec![],
                    covars: vec!["a".to_owned()],
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

    /*    #[test]
        fn transform_cons() {
            let result = <Constructor as Into<Producer>>::into(example_cons())
                .transform(&mut Default::default());
            let expected = example_cons().transform(&mut Default::default()).into();
            assert_eq!(result, expected)
        }
    */
    #[test]
    fn transform_cocase() {
        let result =
            <Cocase as Into<Producer>>::into(example_cocase()).transform(&mut Default::default());
        let expected = example_cocase().transform(&mut Default::default()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_var() {
        let result = example_var().bind(|_| |_| Statement::Done(), &mut Default::default());
        let expected = Statement::Done();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit() {
        let result = <Literal as Into<Producer>>::into(example_lit())
            .bind(|_| |_| Statement::Done(), &mut Default::default());
        let expected = example_lit().bind(|_| |_| Statement::Done(), &mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_mu() {
        let result = <Mu as Into<Producer>>::into(example_mu())
            .bind(|_| |_| Statement::Done(), &mut Default::default());
        let expected = example_mu().bind(|_| |_| Statement::Done(), &mut Default::default());
        assert_eq!(result, expected)
    }

    /*    #[test]
    fn bind_cons() {
        let result = <Constructor as Into<Producer>>::into(example_cons())
            .bind(|_| |_| Statement::Done(), &mut Default::default());
        let expected = example_cons().bind(|_| |_| Statement::Done(), &mut Default::default());
        assert_eq!(result, expected)
    }*/

    #[test]
    fn bind_cocase() {
        let result = <Cocase as Into<Cocase>>::into(example_cocase())
            .bind(|_| |_| Statement::Done(), &mut Default::default());
        let expected = example_cocase().bind(|_| |_| Statement::Done(), &mut Default::default());
        assert_eq!(result, expected)
    }
}
