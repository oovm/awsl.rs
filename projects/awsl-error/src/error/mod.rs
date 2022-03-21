use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::FromResidual,
    panic::Location,
};
use std::ops::Range;

use yggdrasil_shared::{OffsetRange, Position, PositionRange};

pub type Result<T> = IResult<T, AwslError>;

pub enum IResult<T, E> {
    Success(T),
    Failure(E),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwslError {
    pub kind: AwslErrorKind,
    pub range: PositionRange,
    pub file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AwslErrorKind {
    Internal(String),
}

impl Error for AwslError {}

impl Display for AwslError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.kind)?;
        writeln!(f, "    at {}, {} in {}", self.range.start.line, self.range.start.column, self.file)
    }
}

impl<T> FromResidual<Option<Infallible>> for Result<T> {
    fn from_residual(_: Option<Infallible>) -> Self {
        let location = Location::caller();
        let start = Position {
            line: location.line(),
            column: location.column(),
        };
        let end = Position {
            line: location.line(),
            column: location.column(),
        };
        let file = location.file().to_string();
        let kind = AwslErrorKind::Internal(format!("Can not call methods on None"));
        let error = AwslError { kind, range: Range { start, end }, file };
        IResult::Failure(error)
    }
}
