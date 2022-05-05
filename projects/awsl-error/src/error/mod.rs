use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::{ControlFlow, FromResidual, Range, Try},
    panic::Location,
};

use yggdrasil_shared::{Position, PositionRange};

mod from_3rd;

pub type Result<T> = IResult<T, AwslError>;

#[derive(Debug)]
pub enum IResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> IResult<T, E> {
    #[track_caller]
    pub fn unwrap(self) -> T
    where
        E: Error,
    {
        match self {
            IResult::Success(s) => s,
            IResult::Failure(e) => panic!("{}", e),
        }
    }
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
    Syntax(String),
}

impl Error for AwslError {}

impl Display for AwslError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.kind)?;
        writeln!(f, "    at {}, {} in {}", self.range.start.line, self.range.start.column, self.file)
    }
}

macro_rules! error_message {
    ($f:ident => $v:ident) => {
        pub fn $f<S>(message: S) -> Self
        where
            S: Into<String>,
        {
            let location = Location::caller();
            let start = Position { line: location.line(), column: location.column() };
            let end = Position { line: location.line(), column: location.column() };
            let file = location.file().to_string();
            AwslError { kind: AwslErrorKind::$v(message.into()), range: PositionRange { start, end }, file }
        }
    };
    ($($f:ident => $v:ident),*) => {
        impl AwslError {$(error_message!($f => $v);)*}
    };
}

error_message! {
    internal_error => Internal,
    syntax_error => Syntax
}

impl<T> FromResidual<Option<Infallible>> for Result<T> {
    fn from_residual(_: Option<Infallible>) -> Self {
        let location = Location::caller();
        let start = Position { line: location.line(), column: location.column() };
        let end = Position { line: location.line(), column: location.column() };
        let file = location.file().to_string();
        let kind = AwslErrorKind::Internal(format!("Can not call methods on None"));
        let error = AwslError { kind, range: Range { start, end }, file };
        IResult::Failure(error)
    }
}

impl<T, E> FromResidual<std::result::Result<Infallible, E>> for Result<T>
where
    E: Into<AwslError>,
{
    fn from_residual(residual: std::result::Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => IResult::Failure(e.into()),
        }
    }
}

impl<T> FromResidual<AwslError> for IResult<T, AwslError> {
    fn from_residual(residual: AwslError) -> Self {
        IResult::Failure(residual)
    }
}

impl<T, E> Try for IResult<T, E>
where
    E: Into<AwslError>,
    IResult<T, E>: FromResidual<AwslError>,
{
    type Output = T;
    type Residual = AwslError;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            IResult::Success(s) => ControlFlow::Continue(s),
            IResult::Failure(f) => ControlFlow::Break(f.into()),
        }
    }
}
