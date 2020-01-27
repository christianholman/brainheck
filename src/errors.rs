use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, UnmatchedBracketsError>;

#[derive(Debug, Clone)]
pub struct UnmatchedBracketsError;

impl fmt::Display for UnmatchedBracketsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "program contains unmatched brackets")
    }
}

impl error::Error for UnmatchedBracketsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
