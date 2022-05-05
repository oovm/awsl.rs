mod methods;

use super::*;
use crate::ast::symbols::Operator;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Expression {
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Symbol(Box<SymbolPath>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub prefixes: Vec<Operator>,
    pub suffixes: Vec<Operator>,
    pub expression: Expression,
    pub range: PositionRange,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub operator: Operator,
    pub lhs: Expression,
    pub rhs: Expression,
    pub range: PositionRange,
}
