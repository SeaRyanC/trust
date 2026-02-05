//! Symbol definitions
use std::sync::Arc;

pub type SymbolId = u32;

pub struct Symbol {
    pub id: SymbolId,
    pub name: Arc<str>,
    pub flags: SymbolFlags,
}

#[derive(Default)]
pub struct SymbolFlags(u32);
impl SymbolFlags {
    pub const NONE: u32 = 0;
    pub const VARIABLE: u32 = 1;
    pub const FUNCTION: u32 = 2;
    pub const CLASS: u32 = 4;
}
