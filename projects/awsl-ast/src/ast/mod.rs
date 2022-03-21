use yggdrasil_shared::{Position, PositionRange};

pub struct ASTNode {
    pub kind: ASTKind,
    pub range: PositionRange,
}

impl Default for ASTNode {
    fn default() -> Self {
        Self { kind: ASTKind::Null, range: PositionRange { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 0 } } }
    }
}

pub enum ASTKind {
    Root(Vec<ASTNode>),
    Statement(Vec<ASTNode>),
    Null,
}

impl ASTNode {
    pub fn program(node: Vec<ASTNode>, span: PositionRange) -> Self {
        Self { kind: ASTKind::Root(node), range: span }
    }
    pub fn statement(node: Vec<ASTNode>, span: PositionRange) -> Self {
        Self { kind: ASTKind::Statement(node), range: span }
    }
}
