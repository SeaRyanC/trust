//! Diagnostic types for error and warning reporting
//!
//! Provides types for reporting compiler messages to users, compatible
//! with LSP diagnostics.

use crate::text_span::TextSpan;
use serde::{Deserialize, Serialize};

/// Category of diagnostic message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticCategory {
    Warning,
    Error,
    Suggestion,
    Message,
}

impl DiagnosticCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticCategory::Warning => "warning",
            DiagnosticCategory::Error => "error",
            DiagnosticCategory::Suggestion => "suggestion",
            DiagnosticCategory::Message => "message",
        }
    }
}

/// A diagnostic message template
#[derive(Debug, Clone)]
pub struct DiagnosticMessage {
    pub code: u32,
    pub category: DiagnosticCategory,
    pub key: &'static str,
    pub message: &'static str,
}

impl DiagnosticMessage {
    pub const fn error(code: u32, key: &'static str, message: &'static str) -> Self {
        Self {
            code,
            category: DiagnosticCategory::Error,
            key,
            message,
        }
    }

    pub const fn warning(code: u32, key: &'static str, message: &'static str) -> Self {
        Self {
            code,
            category: DiagnosticCategory::Warning,
            key,
            message,
        }
    }
}

/// A diagnostic with location information
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// The file path where this diagnostic occurred
    pub file: Option<String>,
    /// The span in the source text
    pub span: Option<TextSpan>,
    /// The diagnostic message template
    pub message_text: String,
    /// Error code (e.g., 1002 for "Unterminated string literal")
    pub code: u32,
    /// Category (error, warning, etc.)
    pub category: DiagnosticCategory,
    /// Related information
    pub related_information: Vec<DiagnosticRelatedInformation>,
}

impl Diagnostic {
    pub fn new(message: &DiagnosticMessage, args: &[&str]) -> Self {
        let message_text = if args.is_empty() {
            message.message.to_string()
        } else {
            let mut text = message.message.to_string();
            for (i, arg) in args.iter().enumerate() {
                text = text.replace(&format!("{{{}}}", i), arg);
            }
            text
        };

        Self {
            file: None,
            span: None,
            message_text,
            code: message.code,
            category: message.category,
            related_information: Vec::new(),
        }
    }

    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }

    pub fn with_span(mut self, span: TextSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_location(mut self, file: String, span: TextSpan) -> Self {
        self.file = Some(file);
        self.span = Some(span);
        self
    }
}

/// Related diagnostic information
#[derive(Debug, Clone)]
pub struct DiagnosticRelatedInformation {
    pub file: String,
    pub span: TextSpan,
    pub message: String,
}

/// Collection of diagnostics
#[derive(Debug, Default)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.category == DiagnosticCategory::Error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.category == DiagnosticCategory::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.category == DiagnosticCategory::Warning)
    }

    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl IntoIterator for DiagnosticBag {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}

/// Common diagnostic messages used across the compiler
pub mod messages {
    use super::DiagnosticMessage;

    // Scanner errors (1000-1999)
    pub const UNTERMINATED_STRING_LITERAL: DiagnosticMessage =
        DiagnosticMessage::error(1002, "Unterminated_string_literal", "Unterminated string literal.");
    
    pub const UNTERMINATED_REGULAR_EXPRESSION_LITERAL: DiagnosticMessage =
        DiagnosticMessage::error(1003, "Unterminated_regular_expression_literal", "Unterminated regular expression literal.");
    
    pub const UNEXPECTED_TOKEN: DiagnosticMessage =
        DiagnosticMessage::error(1005, "Unexpected_token", "Unexpected token.");
    
    pub const INVALID_CHARACTER: DiagnosticMessage =
        DiagnosticMessage::error(1006, "Invalid_character", "Invalid character.");
    
    pub const DIGIT_EXPECTED: DiagnosticMessage =
        DiagnosticMessage::error(1124, "Digit_expected", "Digit expected.");
    
    pub const HEXADECIMAL_DIGIT_EXPECTED: DiagnosticMessage =
        DiagnosticMessage::error(1125, "Hexadecimal_digit_expected", "Hexadecimal digit expected.");
    
    pub const BINARY_DIGIT_EXPECTED: DiagnosticMessage =
        DiagnosticMessage::error(1177, "Binary_digit_expected", "Binary digit expected.");
    
    pub const OCTAL_DIGIT_EXPECTED: DiagnosticMessage =
        DiagnosticMessage::error(1178, "Octal_digit_expected", "Octal digit expected.");
    
    pub const UNTERMINATED_TEMPLATE_LITERAL: DiagnosticMessage =
        DiagnosticMessage::error(1160, "Unterminated_template_literal", "Unterminated template literal.");
    
    pub const INVALID_ESCAPE_SEQUENCE: DiagnosticMessage =
        DiagnosticMessage::error(1125, "Invalid_escape_sequence", "Invalid escape sequence.");

    // Parser errors (1000-1999 continued)
    pub const DECLARATION_OR_STATEMENT_EXPECTED: DiagnosticMessage =
        DiagnosticMessage::error(1128, "Declaration_or_statement_expected", "Declaration or statement expected.");
    
    pub const IDENTIFIER_EXPECTED: DiagnosticMessage =
        DiagnosticMessage::error(1003, "Identifier_expected", "Identifier expected.");
    
    pub const EXPRESSION_EXPECTED: DiagnosticMessage =
        DiagnosticMessage::error(1109, "Expression_expected", "Expression expected.");

    // Type errors (2000-2999)
    pub const TYPE_0_IS_NOT_ASSIGNABLE_TO_TYPE_1: DiagnosticMessage =
        DiagnosticMessage::error(2322, "Type_0_is_not_assignable_to_type_1", "Type '{0}' is not assignable to type '{1}'.");
    
    pub const CANNOT_FIND_NAME: DiagnosticMessage =
        DiagnosticMessage::error(2304, "Cannot_find_name_0", "Cannot find name '{0}'.");
    
    pub const PROPERTY_0_DOES_NOT_EXIST_ON_TYPE_1: DiagnosticMessage =
        DiagnosticMessage::error(2339, "Property_0_does_not_exist_on_type_1", "Property '{0}' does not exist on type '{1}'.");
}
