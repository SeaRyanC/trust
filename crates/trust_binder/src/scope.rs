//! Scope management
use std::collections::HashMap;
use super::symbols::SymbolId;

pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub bindings: HashMap<String, SymbolId>,
}
