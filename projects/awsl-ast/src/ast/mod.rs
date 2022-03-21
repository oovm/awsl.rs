use std::ops::Range;
use yggdrasil_shared::{Position, PositionRange};

pub struct ASTNode {
    pub kind: ASTKind,
    pub span: PositionRange,
}

impl Default for ASTNode {
    fn default() -> Self {
        Self {
            kind: ASTKind::Null,
            span: PositionRange { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 0 } }
        }
    }
}

pub enum ASTKind {
    Null,
}
