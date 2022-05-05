use url::Position;
use yggdrasil_shared::PositionRange;

pub struct ASTNode {
    pub kind: ASTKind,
    pub span: PositionRange,
}

pub struct ASTKind {}
