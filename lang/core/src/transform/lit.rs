use crate::syntax::statement::Cut;

use super::super::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{Literal, MuTilde, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Literal {
    type Target = Literal;
    ///N(n) = n
    fn transform(self, _state: &mut TransformState) -> Literal {
        self
    }
}

impl Bind for Literal {
    ///bind(⌜n⌝)[k] = ⟨⌜n⌝ | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(self.into()),
            consumer: Rc::new(
                MuTilde {
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
    use crate::{
        naming_transformation::{Bind, NamingTransformation},
        syntax::{statement::Cut, Literal, MuTilde, Statement},
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
                MuTilde {
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
                MuTilde {
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
