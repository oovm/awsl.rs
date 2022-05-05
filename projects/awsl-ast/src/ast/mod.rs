pub use self::containers::List;
pub use self::symbols::{Operator, OperatorKind, Symbol, SymbolPath};
pub use self::{
    control::{ControlKeyword, ForInLoop, IfElseChain},
    expression::Expression,
};
use crate::{Result, Success};
use awsl_error::IResult;
pub use serde_derive::{Deserialize, Serialize};
use std::collections::VecDeque;
use yggdrasil_shared::{Position, PositionRange};

mod containers;
mod control;
mod expression;
mod symbols;
mod traits;

pub type ASTNode = Ranged<ASTKind>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ranged<T> {
    pub kind: T,
    pub range: PositionRange,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ASTKind {
    Root(Vec<ASTNode>),
    Statement(Vec<ASTNode>),
    Block(Vec<ASTNode>),
    ForInLoop(Box<ForInLoop>),
    /// eos?
    Expression(Expression, bool),
    Keywords(Keywords),
    Symbol(SymbolPath),
    Null,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Keywords {
    pub keyword: String,
}

impl ASTNode {
    pub fn program(node: Vec<ASTNode>, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::Root(node), range })
    }
    pub fn statement(node: Vec<ASTNode>, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::Statement(node), range })
    }
    pub fn block(children: Vec<ASTNode>, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::Block(children), range })
    }
    pub fn for_in_loop(pattern: ASTNode, terms: ASTNode, block: ASTNode, guard: Option<ASTNode>, for_else: Option<ASTNode>, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::ForInLoop(box ForInLoop { pattern, terms, guard, block, for_else }), range })
    }
    pub fn expression(expression: Expression, eos: bool, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::Expression(expression, eos), range })
    }
    pub fn keywords(keyword: &'static str, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::Keywords(Keywords { keyword: keyword.to_string() }), range })
    }
}
