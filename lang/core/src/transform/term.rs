use crate::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::term::{PrdCns, Term},
};

impl<T: PrdCns> NamingTransformation for Term<T> {
    type Target = Term<T>;

    fn transform(self, st: &mut TransformState) -> Self::Target {
        match self {
            Term::XVar(var) => todo!(),
            Term::Literal(lit) => todo!(),
            Term::Mu(mu) => todo!(),
            Term::Xtor(xtor) => todo!(),
            Term::XCase(xcase) => todo!(),
        }
    }
}
