use super::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Symbol {
    symbol: String,
    range: PositionRange,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state)
    }
}

impl Symbol {
    pub fn new(symbol: String, range: PositionRange) -> Self {
        Self { symbol, range }
    }
}
