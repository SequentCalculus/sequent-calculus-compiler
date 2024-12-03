use crate::syntax::Ty;

pub trait Typed {
    fn get_type(&self) -> Ty;
}
