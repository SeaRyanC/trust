//! Source map generation
pub struct SourceMap { mappings: Vec<Mapping> }
pub struct Mapping { gen_line: u32, gen_col: u32, src_line: u32, src_col: u32 }
impl SourceMap { pub fn new() -> Self { Self { mappings: Vec::new() } } }
