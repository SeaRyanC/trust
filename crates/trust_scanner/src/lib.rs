//! Trust Scanner - Lexical analysis for TypeScript
//!
//! Converts source code into tokens for parsing.

mod lexer;
mod tok;

pub use lexer::Lexer;
pub use tok::{Tok, TokData};
