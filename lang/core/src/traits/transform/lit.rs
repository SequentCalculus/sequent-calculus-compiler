use super::{Bind, Continuation, NamingTransformation, TransformState};
use crate::syntax::statement::Cut;
use crate::syntax::{
    term::{Cns, Literal, Mu, Term},
    Statement,
};
use std::rc::Rc;

impl NamingTransformation for Literal {
    type Target = Literal;
    fn transform(self, _: &mut TransformState) -> Self::Target {
        self
    }
}

impl Bind for Literal {
    ///bind(⌜n⌝)[k] = ⟨⌜n⌝ | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(Term::Literal(self)),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: new_var.clone(),
                    statement: Rc::new(k(new_var, state)),
                }
                .into(),
            ),
        }
        .into()
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{Bind, NamingTransformation};
    use crate::syntax::{
        statement::Cut,
        term::{Cns, Literal, Mu},
        Statement,
    };
    use std::rc::Rc;

    fn example_lit1() -> Literal {
        Literal { lit: 1 }
    }
    fn example_lit2() -> Literal {
        Literal { lit: 2 }
    }

    #[test]
    fn transform_lit1() {
        let result = example_lit1().transform(&mut Default::default());
        let expected = example_lit1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_lit2() {
        let result = example_lit2().transform(&mut Default::default());
        let expected = example_lit2();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_lit1() {
        let result =
            example_lit1().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn bind_lit2() {
        let result =
            example_lit2().bind(Box::new(|_, _| Statement::Done()), &mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 2 }.into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(Statement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
