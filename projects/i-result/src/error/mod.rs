use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::{ControlFlow, FromResidual, Range, Try},
    panic::Location,
};
use yggdrasil_shared::Position;

#[derive(Debug)]
pub enum IResult<T, E> {
    Success(T),
    Failure(E),
}

#[derive(Debug)]
pub enum TResult<T, E> {
    Failure(E),
    Success(T),
}

impl<T, E> IResult<T, E> where E: Error {
    pub fn unwrap(self) -> T {
        match self {
            IResult::Success(s) => { s }
            IResult::Failure(e) => { panic!("{:?}", e) }
        }
    }
}
