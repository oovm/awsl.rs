use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::FromResidual,
    panic::Location,
};

pub type Result<T> = IResult<T, ValkyrieError>;

pub enum IResult<T, E> {
    Success(T),
    Failure(E),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieError {
    pub kind: ErrorKind,
    pub line: u32,
    pub column: u32,
    pub file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Internal(String),
}

impl Error for ValkyrieError {}

impl Display for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.kind)?;
        writeln!(f, "    at {}, {} in {}", self.line, self.column, self.file)
    }
}

impl<T> FromResidual<Option<Infallible>> for Result<T> {
    fn from_residual(_: Option<Infallible>) -> Self {
        let location = Location::caller();
        let line = location.line();
        let column = location.column();
        let file = location.file().to_string();
        let kind = ErrorKind::Internal(format!("Call . on None"));
        let error = ValkyrieError { kind, line, column, file };
        IResult::Failure(error)
    }
}
