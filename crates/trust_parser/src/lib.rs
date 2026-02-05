//! Trust Parser - TypeScript AST construction
//!
//! Transforms token stream into abstract syntax tree.

pub mod ast;
pub mod parse;

pub use ast::*;
pub use parse::Parser;
