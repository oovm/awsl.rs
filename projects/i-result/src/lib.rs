#![feature(try_trait_v2)]

mod error;

pub use crate::IResult::{Failure, Success};
pub use error::{AwslErrorKind, IResult, Result, AwslError};

