//! Source file and text handling
//!
//! Provides types for representing source files with efficient
//! line/column lookup for LSP support.

use crate::text_span::{LineAndColumn, TextPos, TextSpan};
use std::sync::Arc;

/// Immutable source text with efficient line lookup
#[derive(Debug, Clone)]
pub struct SourceText {
    /// The actual source code
    text: Arc<str>,
    /// Byte positions of line starts (cached for efficient lookup)
    line_starts: Arc<[TextPos]>,
}

impl SourceText {
    pub fn new(text: impl Into<Arc<str>>) -> Self {
        let text: Arc<str> = text.into();
        let line_starts = Self::compute_line_starts(&text);
        Self {
            text,
            line_starts: line_starts.into(),
        }
    }

    fn compute_line_starts(text: &str) -> Vec<TextPos> {
        let mut line_starts = vec![0];
        let mut pos = 0u32;

        for (i, c) in text.char_indices() {
            pos = i as u32;
            if c == '\n' {
                line_starts.push(pos + 1);
            } else if c == '\r' {
                // Handle \r\n as single newline
                if text[i + 1..].starts_with('\n') {
                    // Will be handled by \n
                } else {
                    line_starts.push(pos + 1);
                }
            }
        }

        line_starts
    }

    /// Get the full source text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the length in bytes
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Get a slice of the source text
    pub fn slice(&self, span: TextSpan) -> &str {
        let start = span.start as usize;
        let end = span.end() as usize;
        &self.text[start..end.min(self.text.len())]
    }

    /// Get line and column from a byte position
    pub fn line_and_column(&self, pos: TextPos) -> LineAndColumn {
        // Binary search for the line containing this position
        let line = match self.line_starts.binary_search(&pos) {
            Ok(exact) => exact,
            Err(insert_point) => insert_point.saturating_sub(1),
        };

        let line_start = self.line_starts[line];
        let column = pos - line_start;

        LineAndColumn {
            line: (line + 1) as u32,     // 1-indexed
            column: (column + 1) as u32, // 1-indexed
        }
    }

    /// Get byte position from line and column (1-indexed)
    pub fn position_from_line_and_column(&self, line: u32, column: u32) -> TextPos {
        if line == 0 || line as usize > self.line_starts.len() {
            return 0;
        }
        let line_start = self.line_starts[(line - 1) as usize];
        line_start + (column.saturating_sub(1))
    }

    /// Get number of lines
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Get the start position of a line (0-indexed)
    pub fn line_start(&self, line: usize) -> TextPos {
        self.line_starts.get(line).copied().unwrap_or(0)
    }

    /// Get the text of a specific line (0-indexed)
    pub fn line_text(&self, line: usize) -> &str {
        let start = self.line_start(line) as usize;
        let end = self
            .line_starts
            .get(line + 1)
            .map(|&e| e as usize)
            .unwrap_or(self.text.len());
        
        // Trim trailing newline characters
        let text = &self.text[start..end];
        text.trim_end_matches(['\r', '\n'])
    }
}

/// A source file with metadata
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// File path
    pub file_name: String,
    /// The source text
    pub text: SourceText,
    /// Language variant (e.g., .ts, .tsx, .js, .jsx)
    pub language_variant: LanguageVariant,
    /// Script kind
    pub script_kind: ScriptKind,
}

impl SourceFile {
    pub fn new(file_name: impl Into<String>, text: impl Into<Arc<str>>) -> Self {
        let file_name = file_name.into();
        let script_kind = ScriptKind::from_file_extension(&file_name);
        let language_variant = LanguageVariant::from_script_kind(script_kind);
        
        Self {
            file_name,
            text: SourceText::new(text),
            language_variant,
            script_kind,
        }
    }

    pub fn with_script_kind(mut self, kind: ScriptKind) -> Self {
        self.script_kind = kind;
        self.language_variant = LanguageVariant::from_script_kind(kind);
        self
    }
}

/// Language variant determining JSX support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVariant {
    Standard,
    JSX,
}

impl LanguageVariant {
    pub fn from_script_kind(kind: ScriptKind) -> Self {
        match kind {
            ScriptKind::TSX | ScriptKind::JSX => LanguageVariant::JSX,
            _ => LanguageVariant::Standard,
        }
    }
}

/// Script kind (file type)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind {
    Unknown,
    JS,
    JSX,
    TS,
    TSX,
    External,
    JSON,
    /// Deferred loading
    Deferred,
}

impl ScriptKind {
    pub fn from_file_extension(file_name: &str) -> Self {
        let lower = file_name.to_lowercase();
        if lower.ends_with(".ts") {
            ScriptKind::TS
        } else if lower.ends_with(".tsx") {
            ScriptKind::TSX
        } else if lower.ends_with(".js") || lower.ends_with(".mjs") || lower.ends_with(".cjs") {
            ScriptKind::JS
        } else if lower.ends_with(".jsx") {
            ScriptKind::JSX
        } else if lower.ends_with(".json") {
            ScriptKind::JSON
        } else {
            ScriptKind::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_and_column() {
        let text = SourceText::new("hello\nworld\ntest");
        
        // First line
        assert_eq!(text.line_and_column(0), LineAndColumn::new(1, 1));
        assert_eq!(text.line_and_column(4), LineAndColumn::new(1, 5));
        
        // Second line
        assert_eq!(text.line_and_column(6), LineAndColumn::new(2, 1));
        assert_eq!(text.line_and_column(10), LineAndColumn::new(2, 5));
        
        // Third line
        assert_eq!(text.line_and_column(12), LineAndColumn::new(3, 1));
    }

    #[test]
    fn test_line_text() {
        let text = SourceText::new("hello\nworld\ntest");
        
        assert_eq!(text.line_text(0), "hello");
        assert_eq!(text.line_text(1), "world");
        assert_eq!(text.line_text(2), "test");
    }

    #[test]
    fn test_script_kind_detection() {
        assert_eq!(ScriptKind::from_file_extension("test.ts"), ScriptKind::TS);
        assert_eq!(ScriptKind::from_file_extension("test.tsx"), ScriptKind::TSX);
        assert_eq!(ScriptKind::from_file_extension("test.js"), ScriptKind::JS);
        assert_eq!(ScriptKind::from_file_extension("test.jsx"), ScriptKind::JSX);
        assert_eq!(ScriptKind::from_file_extension("test.json"), ScriptKind::JSON);
    }
}
