use crate::syntax::types::Ty;
use std::rc::Rc;

pub trait Typed {
    fn get_type(&self) -> Ty;
}

impl<T: Typed> Typed for Rc<T> {
    fn get_type(&self) -> Ty {
        self.as_ref().get_type()
    }
}
