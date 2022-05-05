use self::OperatorKind::*;
use super::*;
use awsl_error::AwslError;
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OperatorKind {
    Infix = 0,
    Prefix = 1,
    Suffix = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Operator {
    Add = 0,
}

#[derive(Debug)]
pub struct OperatorDisplay {
    pub name: &'static str,
    pub symbol: &'static str,
    pub kind: OperatorKind,
    pub precedence: u8,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_display().symbol, f)
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.as_display(), f)
    }
}

impl Operator {
    const fn as_display(&self) -> OperatorDisplay {
        match self {
            Operator::Add => OperatorDisplay { name: "add", symbol: "+", kind: Infix, precedence: 0 },
        }
    }
}
