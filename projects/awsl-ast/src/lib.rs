#![feature(once_cell)]
#![feature(box_syntax)]

pub use ast::{ASTKind, ASTNode};
pub use awsl_error::{Failure, Result, Success};
pub use parser::ParserConfig;

mod ast;
mod parser;
