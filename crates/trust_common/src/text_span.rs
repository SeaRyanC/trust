//! Text span and range types for source location tracking
//!
//! These types are essential for LSP support, providing precise
//! character positions in source files.

use serde::{Deserialize, Serialize};

/// A position in source text, represented as a byte offset
pub type TextPos = u32;

/// A span in source text, represented as start and length
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextSpan {
    /// Starting position (byte offset)
    pub start: TextPos,
    /// Length in bytes
    pub length: TextPos,
}

impl TextSpan {
    pub fn new(start: TextPos, length: TextPos) -> Self {
        Self { start, length }
    }

    pub fn from_bounds(start: TextPos, end: TextPos) -> Self {
        Self {
            start,
            length: end.saturating_sub(start),
        }
    }

    pub fn end(&self) -> TextPos {
        self.start + self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn contains(&self, position: TextPos) -> bool {
        position >= self.start && position < self.end()
    }

    pub fn contains_span(&self, other: &TextSpan) -> bool {
        other.start >= self.start && other.end() <= self.end()
    }

    pub fn overlaps(&self, other: &TextSpan) -> bool {
        self.start < other.end() && other.start < self.end()
    }

    /// Combine two spans into a span that covers both
    pub fn union(&self, other: &TextSpan) -> TextSpan {
        let start = self.start.min(other.start);
        let end = self.end().max(other.end());
        TextSpan::from_bounds(start, end)
    }
}

impl Default for TextSpan {
    fn default() -> Self {
        Self { start: 0, length: 0 }
    }
}

/// A range in source text with start and end positions
/// This is the more common representation for editors/LSP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextRange {
    pub start: TextPos,
    pub end: TextPos,
}

impl TextRange {
    pub fn new(start: TextPos, end: TextPos) -> Self {
        Self { start, end }
    }

    pub fn to_span(&self) -> TextSpan {
        TextSpan::from_bounds(self.start, self.end)
    }

    pub fn length(&self) -> TextPos {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl From<TextSpan> for TextRange {
    fn from(span: TextSpan) -> Self {
        Self {
            start: span.start,
            end: span.end(),
        }
    }
}

impl From<TextRange> for TextSpan {
    fn from(range: TextRange) -> Self {
        Self::from_bounds(range.start, range.end)
    }
}

/// Line and column position (1-indexed, for human readability)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineAndColumn {
    /// 1-indexed line number
    pub line: u32,
    /// 1-indexed column number (in UTF-16 code units for LSP compatibility)
    pub column: u32,
}

impl LineAndColumn {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_span_contains() {
        let span = TextSpan::new(10, 5); // [10, 15)
        assert!(span.contains(10));
        assert!(span.contains(14));
        assert!(!span.contains(15));
        assert!(!span.contains(9));
    }

    #[test]
    fn test_text_span_union() {
        let span1 = TextSpan::new(10, 5); // [10, 15)
        let span2 = TextSpan::new(12, 8); // [12, 20)
        let union = span1.union(&span2);
        assert_eq!(union.start, 10);
        assert_eq!(union.end(), 20);
    }

    #[test]
    fn test_text_range_conversion() {
        let span = TextSpan::new(5, 10);
        let range: TextRange = span.into();
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 15);

        let back: TextSpan = range.into();
        assert_eq!(back, span);
    }
}
