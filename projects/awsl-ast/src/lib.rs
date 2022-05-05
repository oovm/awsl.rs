#![feature(once_cell)]
#![feature(box_syntax)]

extern crate core;

pub use ast::{ASTKind, ASTNode};
pub use awsl_error::{Failure, Result, Success};
pub use parser::ParserConfig;

pub mod ast;
mod parser;
mod traits;
mod value;
