use crate::syntax::CodataDeclaration;

/// This trait provides a method for deciding whether a given term is a (co)value.
pub trait IsCoValue {
    /// This method returns whether the given term is a (co)value.
    fn is_co_value(&self, codata_types: &[CodataDeclaration]) -> bool;
}

impl<T: IsCoValue> IsCoValue for Vec<T> {
    fn is_co_value(&self, codata_types: &[CodataDeclaration]) -> bool {
        self.iter().all(|element| element.is_co_value(codata_types))
    }
}

/// This trait provides a method for deciding whether a given term is a value.
pub trait IsValue {
    /// This method returns whether the given term is a value.
    fn is_value(&self, codata_types: &[CodataDeclaration]) -> bool;
}

/// This trait provides a method for deciding whether a given term is a covalue.
pub trait IsCovalue {
    /// This method returns whether the given term is a covalue.
    fn is_covalue(&self, codata_types: &[CodataDeclaration]) -> bool;
}
