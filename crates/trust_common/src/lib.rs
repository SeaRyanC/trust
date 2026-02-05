//! Trust Common - Shared types and utilities for the TypeScript compiler
//!
//! This crate provides common types used across all phases of compilation:
//! - Source positions and spans for LSP support
//! - Diagnostic types for error reporting
//! - Syntax kinds enumeration matching TypeScript

pub mod diagnostics;
pub mod source;
pub mod syntax_kind;
pub mod text_span;

pub use diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticMessage};
pub use source::{SourceFile, SourceText};
pub use syntax_kind::SyntaxKind;
pub use text_span::{TextRange, TextSpan};
