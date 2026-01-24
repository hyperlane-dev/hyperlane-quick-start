#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JwtValidationError {
    Expired,
    InvalidSignature,
    InvalidIssuer,
    InvalidSubject,
    NotYetValid,
    Malformed,
    Other(String),
}
