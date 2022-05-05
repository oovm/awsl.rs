mod operator_type;
mod symbol_type;

use super::*;

pub use self::operator_type::{Operator, OperatorKind};
pub use self::symbol_type::Symbol;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SymbolPath {
    full_path: Vec<Ranged<String>>,
}

impl SymbolPath {
    pub fn empty(capacity: usize) -> Self {
        Self { full_path: Vec::with_capacity(capacity) }
    }
    pub fn push(&mut self, symbol: String, range: PositionRange) {
        self.full_path.push(Ranged { kind: symbol, range });
    }
}
