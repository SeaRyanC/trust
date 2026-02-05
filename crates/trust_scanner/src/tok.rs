use trust_common::SyntaxKind;

pub struct Tok {
    pub k: SyntaxKind,
    pub p0: u32,
    pub p1: u32,
    pub txt: Option<Box<str>>,
    pub num: Option<f64>,
    pub nl: bool,
}
