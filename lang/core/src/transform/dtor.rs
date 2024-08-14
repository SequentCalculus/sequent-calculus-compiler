use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{
        Consumer, Covar, Covariable, Cut, Destructor, MuTilde, Name, Statement, Var, Variable,
    },
};
use std::rc::Rc;

impl NamingTransformation for Destructor {
    type Target = Consumer;
    ///N (D (pi ; c j )) =  μx  ̃ .bind(pi ) [λas.bind(c j ) [λbs.⟨x | D (as; bs)⟩]]
    fn transform(self, st: &mut TransformState) -> Consumer {
        let new_v = st.fresh_var();
        let new_st = Bind::bind_many(self.producers, |ns: Vec<Var>| {
            |_: &mut TransformState| {
                Bind::bind_many(self.consumers, |bs: Vec<Covar>| {
                    |_: &mut TransformState| {
                        Cut {
                            producer: Rc::new(Variable { var: new_v.clone() }.into()),
                            consumer: Rc::new(
                                Destructor {
                                    id: self.id,
                                    producers: ns
                                        .into_iter()
                                        .map(|n| Variable { var: n }.into())
                                        .collect(),
                                    consumers: bs
                                        .into_iter()
                                        .map(|b| Covariable { covar: b }.into())
                                        .collect(),
                                }
                                .into(),
                            ),
                        }
                        .into()
                    }
                })
            }
        });
        MuTilde {
            variable: new_v,
            statement: Rc::new(new_st),
        }
        .into()
    }
}

impl Bind for Destructor {
    ///bind(D (pi ; c j )) [k] =  bind(p i ) [λas.bind(c j ) [λbs.⟨μα .k (α) | D (as; bs)⟩]]
    fn bind<F, K>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: FnOnce(Name) -> K,
        K: FnOnce(&mut TransformState) -> Statement,
    {
        todo!("not impleneted")
    }
}
