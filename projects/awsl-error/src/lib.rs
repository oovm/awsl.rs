#![feature(try_trait_v2)]

pub use crate::IResult::{Failure, Success};
pub use error::{ErrorKind, IResult, Result, ValkyrieError};

mod error;
