use crate::syntax::{names::Name, types::Ty};
use std::{collections::HashMap, rc::Rc};

pub trait SubstType {
    fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Self;
}

impl<T> SubstType for Rc<T>
where
    T: SubstType + Clone,
{
    fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).subst_ty(mappings))
    }
}

impl<T> SubstType for Option<T>
where
    T: SubstType,
{
    fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Self {
        self.map(|t| t.subst_ty(mappings))
    }
}

impl<T> SubstType for Vec<T>
where
    T: SubstType,
{
    fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Self {
        self.into_iter().map(|t| t.subst_ty(mappings)).collect()
    }
}
