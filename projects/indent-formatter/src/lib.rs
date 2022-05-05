mod formatter;
mod from_raw;
mod macros;

pub use self::formatter::IndentFormatter;

/// The main entry point for the program.
pub trait IndentDisplay {
    fn indent_fmt(&self, f: IndentFormatter) -> std::fmt::Result;
}
