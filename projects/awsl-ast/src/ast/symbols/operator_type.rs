use super::*;
use std::mem::transmute;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum OperatorKind {
    Infix = 0,
    Prefix = 1,
    Suffix = 2,
}

impl From<u8> for OperatorKind {
    fn from(n: u8) -> Self {
        unsafe { transmute::<u8, Self>(n) }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Operator {
    pub kind: OperatorKind,
    pub precedence: u8,
    pub symbol: &'static str,
}

impl Operator {
    pub const ADD: Self = Self::new(0, 8, "+");

    pub const fn new(kind: u8, precedence: u8, symbol: &'static str) -> Self {
        Self { kind: kind.into(), precedence, symbol }
    }
}
