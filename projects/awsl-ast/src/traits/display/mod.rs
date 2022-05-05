use crate::{ASTKind, ASTNode};
use indentation::{IndentDisplay, IndentFormatter};
use std::fmt::{Display, Formatter};

impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.indent_fmt(IndentFormatter::from(f))
    }
}

impl Display for ASTKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.indent_fmt(IndentFormatter::from(f))
    }
}

impl IndentDisplay for ASTKind {
    fn indent_fmt(&self, f: IndentFormatter) -> std::fmt::Result {
        match self {
            ASTKind::Root(_) => {}
            ASTKind::Statement(_) => {}
            ASTKind::Block(_) => {}
            ASTKind::ForInLoop(_) => {}
            ASTKind::Expression(_) => {}
            ASTKind::Keywords(_) => {}
            ASTKind::Null => {}
            ASTKind::Symbol(_) => {}
        }
        todo!()
    }
}
