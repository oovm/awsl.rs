mod methods;

use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Expression {
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    List(Box<List>),
    Symbol(Box<SymbolPath>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub prefixes: Vec<Operator>,
    pub suffixes: Vec<Operator>,
    pub expression: Expression,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub operator: Operator,
    pub lhs: Expression,
    pub rhs: Expression,
}

impl Expression {
    pub fn range(&self) -> PositionRange {
        todo!()
    }
}

impl UnaryExpression {
    pub fn range(&self) -> PositionRange {
        todo!()
    }
}

impl BinaryExpression {
    pub fn range(&self) -> PositionRange {
        todo!()
    }
}
