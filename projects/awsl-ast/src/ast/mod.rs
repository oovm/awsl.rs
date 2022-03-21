mod traits;
use awsl_error::IResult;
use yggdrasil_shared::{Position, PositionRange};

use crate::{Result, Success};

pub use self::control::{ControlKeyword, ForInLoop, IfElseChain};

mod control;

pub struct ASTNode {
    pub kind: ASTKind,
    pub range: PositionRange,
}

pub enum ASTKind {
    Root(Vec<ASTNode>),
    Statement(Vec<ASTNode>),
    Block(Vec<ASTNode>),
    ForInLoop(Box<ForInLoop>),
    Expression(Vec<ASTNode>),
    Null,
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
        Success(Self { kind: ASTKind::Expression(Box::new(children), eos), range })
    }
}
