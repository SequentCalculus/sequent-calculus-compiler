use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Clause, Name, Statement},
};

impl<T> NamingTransformation for Clause<T> {
    ///N (Ki (xi,j ; αi,j ) ⇒ si ) = Ki (x i,j ; αi,j ) ⇒ N (si )
    fn transform(self, st: &mut TransformState) -> Clause<T> {
        Clause {
            xtor: self.xtor,
            vars: self.vars,
            covars: self.covars,
            rhs: self.rhs.transform(st),
        }
    }
}

impl<T> Bind for Clause<T> {
    ///bind(Ki (xi,j ; αi,j ) ⇒ si ) [k] = {K i (xi,j ; α i,j ) ⇒ N (si )
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
