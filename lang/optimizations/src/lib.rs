pub trait Inline {
    type Target;
    fn inline(self) -> Self::Target;
}
