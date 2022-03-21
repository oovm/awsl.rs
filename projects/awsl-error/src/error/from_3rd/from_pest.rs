use std::{fmt::Display, ops::Range};

use pest::error::{Error, LineColLocation};
use yggdrasil_shared::Position;

use crate::{AwslError, AwslErrorKind};

impl<R> From<Error<R>> for AwslError
where
    Error<R>: Display,
{
    fn from(e: Error<R>) -> Self {
        let range = match e.line_col {
            LineColLocation::Pos(s) => {
                let start = Position { line: s.0 as u32, column: s.1 as u32 };
                let end = Position { line: s.0 as u32, column: s.1 as u32 };
                Range { start, end }
            }
            LineColLocation::Span(s, e) => {
                let start = Position { line: s.0 as u32, column: s.1 as u32 };
                let end = Position { line: e.0 as u32, column: e.1 as u32 };
                Range { start, end }
            }
        };
        // let kind = e.message();
        // let file = e.path.unwrap_or("<UNKNOWN>".to_string());
        let kind = AwslErrorKind::Syntax(e.to_string());
        Self { kind, range, file: "<UNKNOWN>".to_string() }
    }
}
