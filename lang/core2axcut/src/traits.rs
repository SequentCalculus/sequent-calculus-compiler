use core::syntax::declaration::FsTypeDeclaration;
use core::syntax::Var;

use std::collections::HashSet;
use std::rc::Rc;

/// This assumes all variable bindings to be unique and maintains this invariant.
pub trait Shrinking {
    type Target;
    fn shrink(self, used_vars: &mut HashSet<Var>, types: &[FsTypeDeclaration]) -> Self::Target;
}

impl<T: Shrinking + Clone> Shrinking for Rc<T> {
    type Target = Rc<T::Target>;
    fn shrink(self, used_vars: &mut HashSet<Var>, types: &[FsTypeDeclaration]) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).shrink(used_vars, types))
    }
}

impl<T: Shrinking> Shrinking for Vec<T> {
    type Target = Vec<T::Target>;
    fn shrink(self, used_vars: &mut HashSet<Var>, types: &[FsTypeDeclaration]) -> Self::Target {
        self.into_iter()
            .map(|element| element.shrink(used_vars, types))
            .collect()
    }
}
