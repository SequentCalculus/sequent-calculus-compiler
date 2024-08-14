use crate::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::Clause,
};

impl<T> NamingTransformation for Clause<T> {
    type Target = Clause<T>;
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
