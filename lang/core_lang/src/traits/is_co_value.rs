use crate::syntax::{CodataDeclaration, TypeParam};

/// This trait provides a method for deciding whether a given term is a (co)value.
pub trait IsCoValue {
    /// This method returns whether the given term is a (co)value.
    /// - `type_params` is the ambient list of declaration-site type parameters currently in
    ///   scope, needed to resolve the polarity of a term typed at a `Ty::Var`.
    fn is_co_value(&self, codata_types: &[CodataDeclaration], type_params: &[TypeParam]) -> bool;
}

impl<T: IsCoValue> IsCoValue for Vec<T> {
    fn is_co_value(&self, codata_types: &[CodataDeclaration], type_params: &[TypeParam]) -> bool {
        self.iter()
            .all(|element| element.is_co_value(codata_types, type_params))
    }
}

/// This trait provides a method for deciding whether a given term is a value.
pub trait IsValue {
    /// This method returns whether the given term is a value.
    /// - `type_params` is the ambient list of declaration-site type parameters currently in
    ///   scope, needed to resolve the polarity of a term typed at a `Ty::Var`.
    fn is_value(&self, codata_types: &[CodataDeclaration], type_params: &[TypeParam]) -> bool;
}

/// This trait provides a method for deciding whether a given term is a covalue.
pub trait IsCovalue {
    /// This method returns whether the given term is a covalue.
    /// - `type_params` is the ambient list of declaration-site type parameters currently in
    ///   scope, needed to resolve the polarity of a term typed at a `Ty::Var`.
    fn is_covalue(&self, codata_types: &[CodataDeclaration], type_params: &[TypeParam]) -> bool;
}
