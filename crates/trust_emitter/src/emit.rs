//! JavaScript emission
pub struct Emitter { out: String }
impl Emitter {
    pub fn new() -> Self { Self { out: String::new() } }
    pub fn finish(self) -> String { self.out }
}
