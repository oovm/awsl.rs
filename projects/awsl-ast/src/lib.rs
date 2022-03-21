#![feature(once_cell)]

pub use ast::{ASTKind, ASTNode};
pub use awsl_error::{Failure, Result, Success};
pub use parser::ParserConfig;

mod parser;
mod ast;

