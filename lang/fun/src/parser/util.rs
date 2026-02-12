//! This module contains helper functions for generating [`miette`] errors.

/// Create a miette::SourceSpan from left and right byte offsets (exclusive).
pub fn span(l: usize, r: usize) -> miette::SourceSpan {
    (l..r).into()
}

/// This trait provides a method for converting things to [`miette`] equivalents.
pub trait ToMiette {
    type Target;

    fn to_miette(self) -> Self::Target;
}

/// This trait provides a method for converting things from [`miette`] equivalents.
pub trait FromMiette {
    type Target;

    #[allow(clippy::wrong_self_convention)]
    fn from_miette(self) -> Self::Target;
}

impl ToMiette for miette::SourceSpan {
    type Target = miette::SourceSpan;

    fn to_miette(self) -> Self::Target {
        self
    }
}

impl<T: ToMiette> ToMiette for Option<T> {
    type Target = Option<T::Target>;

    fn to_miette(self) -> Self::Target {
        self.map(ToMiette::to_miette)
    }
}
