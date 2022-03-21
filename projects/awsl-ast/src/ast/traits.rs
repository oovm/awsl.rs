use super::*;

impl Default for ASTNode {
    fn default() -> Self {
        Self { kind: ASTKind::Null, range: PositionRange { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 0 } } }
    }
}
