use std::{
    fmt::{Arguments, Formatter, Write},
    ops::{AddAssign, SubAssign},
};

/// A formatter that indents lines.
pub struct IndentFormatter<'a, 'i> {
    /// The formatter to use for the actual output.
    pub(crate) raw: &'i mut Formatter<'a>,
    /// The current indentation level.
    pub indent_level: usize,
    /// The number of spaces to use for each indentation level.
    pub indent_chars: &'i str,
}

impl AddAssign<usize> for IndentFormatter<'_, '_> {
    fn add_assign(&mut self, rhs: usize) {
        self.indent_level = self.indent_level.saturating_add(rhs);
    }
}

impl SubAssign<usize> for IndentFormatter<'_, '_> {
    fn sub_assign(&mut self, rhs: usize) {
        self.indent_level = self.indent_level.saturating_sub(rhs)
    }
}

impl Write for IndentFormatter<'_, '_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.raw.write_str(s)
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.raw.write_char(c)
    }
    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> std::fmt::Result {
        self.raw.write_fmt(args)
    }
}

impl<'a, 'i> IndentFormatter<'a, 'i> {
    /// Creates a new formatter with the given indentation level and number of spaces per indentation level.
    pub fn new(f: &'i mut Formatter<'a>, indent: &'i str) -> Self {
        Self { raw: f, indent_level: 0, indent_chars: indent }
    }
    /// Write the given string with the current indentation level.
    pub fn write_indent(&mut self) -> std::fmt::Result {
        for _ in 0..self.indent_level {
            self.raw.write_str(self.indent_chars)?;
        }
        Ok(())
    }
    /// Write the given string with the current indentation level.
    pub fn write_lines(&mut self, s: &str) -> std::fmt::Result {
        for line in s.lines() {
            self.write_indent()?;
            self.raw.write_str(line)?;
        }
        Ok(())
    }
}
