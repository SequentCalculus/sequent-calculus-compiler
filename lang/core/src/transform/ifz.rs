use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{IfZ, Statement, Variable},
};
use std::rc::Rc;

impl NamingTransformation for IfZ {
    type Target = Statement;
    ///N (ifz(p, s1 , s2 )) = bind(p) [λa.ifz(a, N (s 1), N (s 2 ))]
    fn transform(self, st: &mut TransformState) -> Statement {
        let then_trans = self.thenc.transform(st);
        let else_trans = self.elsec.transform(st);
        let cont = |a| {
            |_: &mut TransformState| {
                IfZ {
                    ifc: Rc::new(Variable { var: a }.into()),
                    thenc: then_trans,
                    elsec: else_trans,
                }
                .into()
            }
        };

        Rc::unwrap_or_clone(self.ifc).bind(cont, st)
    }
}
