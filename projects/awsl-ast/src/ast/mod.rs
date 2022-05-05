use yggdrasil_shared::{Position, PositionRange};

use awsl_error::IResult;

use crate::{Result, Success};

pub use self::{
    control::{ControlKeyword, ForInLoop, IfElseChain},
    expression::Expression,
};

mod control;
mod expression;
mod traits;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ASTNode {
    pub kind: ASTKind,
    pub range: PositionRange,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    Root(Vec<ASTNode>),
    Statement(Vec<ASTNode>),
    Block(Vec<ASTNode>),
    ForInLoop(Box<ForInLoop>),
    Expression(Box<Expression>),
    Keywords(Keywords),
    Null,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Keywords {
    pub keyword: &'static str,
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
    pub fn for_in_loop(
        pattern: ASTNode,
        terms: ASTNode,
        block: ASTNode,
        guard: Option<ASTNode>,
        for_else: Option<ASTNode>,
        range: PositionRange,
    ) -> Result<Self> {
        Success(Self { kind: ASTKind::ForInLoop(box ForInLoop { pattern, terms, guard, block, for_else }), range })
    }
    pub fn expression(children: ASTNode, eos: bool, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::Expression(box Expression { nodes: vec![children], eos }), range })
    }
    pub fn keywords(keyword: &'static str, range: PositionRange) -> Result<Self> {
        Success(Self { kind: ASTKind::Keywords(Keywords { keyword }), range })
    }
}
