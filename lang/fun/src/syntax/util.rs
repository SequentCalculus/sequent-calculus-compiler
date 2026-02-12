/// Create an empty span
///
/// For use in test cases only
pub fn dummy_span() -> miette::SourceSpan {
    miette::SourceSpan::from(0..0)
}
