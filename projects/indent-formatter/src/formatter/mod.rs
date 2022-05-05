use std::fmt::Formatter;
use std::ops::{AddAssign, Deref, SubAssign};
use crate::ASTNode;

pub struct IndentFormatter<'i> {
    raw: Formatter<'i>,
    pub indent: usize,
}

impl<'i> From<Formatter<'i>> for IndentFormatter<'i> {
    fn from(f: Formatter<'i>) -> Self {
        Self {
            raw: f,
            indent: 0,
        }
    }
}

impl<'i> Deref for IndentFormatter<'i> {
    type Target = Formatter<'i>;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<'i> AddAssign<usize> for IndentFormatter<'i> {
    fn add_assign(&mut self, rhs: usize) {
        self.indent += rhs;
    }
}

impl<'i> SubAssign<usize> for IndentFormatter<'i> {
    fn sub_assign(&mut self, rhs: usize) {
        self.indent = self.indent.saturating_sub(rhs)
    }
}

pub trait PrettyPrint {
    fn pretty_fmt(&self, f: IndentFormatter) -> std::fmt::Result;
}

impl PrettyPrint for ASTNode {
    fn pretty_fmt(&self, f: IndentFormatter) -> std::fmt::Result {
        todo!()
    }
}