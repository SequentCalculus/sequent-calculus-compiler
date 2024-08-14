use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Covar, Covariable, Op, Statement, Var, Variable},
};
use std::rc::Rc;

impl NamingTransformation for Op {
    type Target = Statement;
    ///N (⊙(p 1, p 2 ; c)) = bind(p 1) [λa1 .bind(p 2 ) [λa2 .bind(c) [λb. ⊙ (a1 , a 2; b)]]]
    fn transform(self, st: &mut TransformState) -> Statement {
        let cont = |a1: Var| {
            |st: &mut TransformState| {
                Rc::unwrap_or_clone(self.snd).bind(
                    |a2: Var| {
                        |st: &mut TransformState| {
                            Rc::unwrap_or_clone(self.continuation).bind(
                                |b: Covar| {
                                    |_: &mut TransformState| {
                                        Op {
                                            fst: Rc::new(Variable { var: a1 }.into()),
                                            op: self.op,
                                            snd: Rc::new(Variable { var: a2 }.into()),
                                            continuation: Rc::new(Covariable { covar: b }.into()),
                                        }
                                        .into()
                                    }
                                },
                                st,
                            )
                        }
                    },
                    st,
                )
            }
        };
        Rc::unwrap_or_clone(self.fst).bind(cont, st)
    }
}
