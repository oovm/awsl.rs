use yggdrasil_shared::{Position, PositionRange};

use awsl_pest::{
    pest::iterators::{Pair, Pairs},
    Rule,
};

pub struct ParserConfig {
    pub tab_size: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self { tab_size: 4 }
    }
}

impl ParserConfig {
    pub fn get_position_root(&self, s: &Pairs<Rule>) -> PositionRange {
        let _ = s;
        PositionRange { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 0 } }
    }
    pub fn get_position(&self, s: &Pair<Rule>) -> PositionRange {
        let start = s.as_span().start_pos().line_col();
        let end = s.as_span().end_pos().line_col();
        PositionRange {
            start: Position { line: start.0 as u32 - 1, column: start.1 as u32 - 1 },
            end: Position { line: end.0 as u32 - 1, column: end.1 as u32 - 1 },
        }
    }
}
