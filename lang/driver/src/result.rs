#[derive(Debug, Clone)]
pub enum DriverError {
    ParseError(fun::parser::result::ParseError),
    TypeError(fun::typing::errors::Error),
}
