mod formatter;
mod macros;
mod from_raw;

pub use self::formatter::{IndentFormatter};

/// The main entry point for the program.
pub trait IndentDisplay {
    fn indent_fmt(&self, f: IndentFormatter) -> std::fmt::Result;
}