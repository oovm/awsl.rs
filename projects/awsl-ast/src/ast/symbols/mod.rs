mod operator_type;
mod symbol_type;

use super::*;
use std::ops::Range;

pub use self::operator_type::{Operator, OperatorKind};
pub use self::symbol_type::Symbol;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SymbolPath {
    full_path: Vec<Ranged<String>>,
}

impl From<SymbolPath> for Expression {
    fn from(s: SymbolPath) -> Self {
        Expression::Symbol(box s)
    }
}

impl SymbolPath {
    pub fn empty(capacity: usize) -> Self {
        Self { full_path: Vec::with_capacity(capacity) }
    }
    pub fn push(&mut self, symbol: String, range: PositionRange) {
        self.full_path.push(Ranged { kind: symbol, range });
    }
    pub fn range(&self) -> PositionRange {
        if self.full_path.is_empty() {
            panic!("not a valid symbol path!");
        }
        unsafe {
            Range {
                // safe when non empty
                start: self.full_path.first().unwrap_unchecked().range.start,
                end: self.full_path.last().unwrap_unchecked().range.end,
            }
        }
    }
}
