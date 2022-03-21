#![feature(once_cell)]

pub use ast::{ASTKind, ASTNode};
pub use awsl_error::{Failure, Result, Success};
pub use parser::AwslParser;

mod parser;
mod ast;

