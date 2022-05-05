use self::OperatorKind::*;
use super::*;
use awsl_error::AwslError;
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::str::FromStr;

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

#[derive(Debug, Clone, Eq)]
pub struct Operator {
    pub name: &'static str,
    pub symbol: &'static str,
    pub kind: OperatorKind,
    pub precedence: u8,
}

impl Operator {
    pub const ADD: Self = Self::new("add", "+", Infix, 8);

    pub const fn new(name: &'static str, symbol: &'static str, kind: OperatorKind, precedence: u8) -> Self {
        Self { name, kind, precedence, symbol }
    }
}

impl FromStr for Operator {
    type Err = AwslError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let out = match s {
            "add" => Self::ADD,
            _ => panic!(),
        };
        Ok(out)
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Operator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

pub struct OperatorVisitor;

impl<'de> Visitor<'de> for OperatorVisitor {
    type Value = Operator;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("FF")
    }
    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: Error,
    {
        Operator::from_str(v).map_err(|e| E::custom(e))
    }
}

impl Serialize for Operator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.name)
    }
}

impl<'de> Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(OperatorVisitor)
    }
}
