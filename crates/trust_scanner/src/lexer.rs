//! Lexer for TypeScript source code

use crate::tok::Tok;
use trust_common::SyntaxKind;

pub struct Lexer<'src> {
    bytes: &'src [u8],
    idx: usize,
    saw_newline: bool,
}

impl<'src> Lexer<'src> {
    pub fn create(source: &'src str) -> Self {
        Lexer {
            bytes: source.as_bytes(),
            idx: 0,
            saw_newline: false,
        }
    }

    pub fn produce_next(&mut self) -> Tok {
        self.consume_whitespace_and_comments();
        let nl_flag = self.saw_newline;
        self.saw_newline = false;

        if self.idx >= self.bytes.len() {
            return Tok {
                k: SyntaxKind::EndOfFile,
                p0: self.idx as u32,
                p1: self.idx as u32,
                txt: None,
                num: None,
                nl: nl_flag,
            };
        }

        let start_pos = self.idx;
        let current_byte = self.bytes[self.idx];

        let result_kind = match current_byte {
            b'{' => { self.idx += 1; SyntaxKind::OpenBrace }
            b'}' => { self.idx += 1; SyntaxKind::CloseBrace }
            b'(' => { self.idx += 1; SyntaxKind::OpenParen }
            b')' => { self.idx += 1; SyntaxKind::CloseParen }
            b'[' => { self.idx += 1; SyntaxKind::OpenBracket }
            b']' => { self.idx += 1; SyntaxKind::CloseBracket }
            b';' => { self.idx += 1; SyntaxKind::Semicolon }
            b',' => { self.idx += 1; SyntaxKind::Comma }
            b'~' => { self.idx += 1; SyntaxKind::Tilde }
            b'@' => { self.idx += 1; SyntaxKind::At }
            b'#' => { self.idx += 1; SyntaxKind::HashToken }
            b'`' => { self.idx += 1; SyntaxKind::BackTick }
            b':' => { self.idx += 1; SyntaxKind::Colon }

            b'.' => self.read_dot_or_spread(),
            b'+' => self.read_plus_variants(),
            b'-' => self.read_minus_variants(),
            b'*' => self.read_star_variants(),
            b'/' => self.read_slash_variants(),
            b'%' => self.read_percent_variants(),
            b'<' => self.read_less_than_variants(),
            b'>' => self.read_greater_than_variants(),
            b'=' => self.read_equals_variants(),
            b'!' => self.read_exclamation_variants(),
            b'&' => self.read_ampersand_variants(),
            b'|' => self.read_pipe_variants(),
            b'^' => self.read_caret_variants(),
            b'?' => self.read_question_variants(),

            b'"' | b'\'' => return self.read_string_literal(start_pos, nl_flag),
            b'0'..=b'9' => return self.read_numeric(start_pos, nl_flag),
            
            _ if is_id_start(current_byte) => return self.read_identifier(start_pos, nl_flag),
            
            _ => { self.idx += 1; SyntaxKind::Unknown }
        };

        Tok {
            k: result_kind,
            p0: start_pos as u32,
            p1: self.idx as u32,
            txt: None,
            num: None,
            nl: nl_flag,
        }
    }

    fn consume_whitespace_and_comments(&mut self) {
        while self.idx < self.bytes.len() {
            match self.bytes[self.idx] {
                b' ' | b'\t' | b'\r' => self.idx += 1,
                b'\n' => {
                    self.idx += 1;
                    self.saw_newline = true;
                }
                b'/' if self.peek_at(1) == Some(b'/') => self.skip_line_comment(),
                b'/' if self.peek_at(1) == Some(b'*') => self.skip_block_comment(),
                _ => break,
            }
        }
    }

    fn skip_line_comment(&mut self) {
        self.idx += 2;
        while self.idx < self.bytes.len() && self.bytes[self.idx] != b'\n' {
            self.idx += 1;
        }
    }

    fn skip_block_comment(&mut self) {
        self.idx += 2;
        while self.idx + 1 < self.bytes.len() {
            if self.bytes[self.idx] == b'\n' {
                self.saw_newline = true;
            }
            if self.bytes[self.idx] == b'*' && self.bytes[self.idx + 1] == b'/' {
                self.idx += 2;
                return;
            }
            self.idx += 1;
        }
        self.idx = self.bytes.len();
    }

    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.idx + offset).copied()
    }

    fn read_dot_or_spread(&mut self) -> SyntaxKind {
        if self.peek_at(1) == Some(b'.') && self.peek_at(2) == Some(b'.') {
            self.idx += 3;
            SyntaxKind::DotDotDot
        } else {
            self.idx += 1;
            SyntaxKind::Dot
        }
    }

    fn read_plus_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        match self.peek_at(0) {
            Some(b'+') => { self.idx += 1; SyntaxKind::PlusPlus }
            Some(b'=') => { self.idx += 1; SyntaxKind::PlusEquals }
            _ => SyntaxKind::Plus
        }
    }

    fn read_minus_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        match self.peek_at(0) {
            Some(b'-') => { self.idx += 1; SyntaxKind::MinusMinus }
            Some(b'=') => { self.idx += 1; SyntaxKind::MinusEquals }
            _ => SyntaxKind::Minus
        }
    }

    fn read_star_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        if self.peek_at(0) == Some(b'*') {
            self.idx += 1;
            if self.peek_at(0) == Some(b'=') {
                self.idx += 1;
                SyntaxKind::AsteriskAsteriskEquals
            } else {
                SyntaxKind::AsteriskAsterisk
            }
        } else if self.peek_at(0) == Some(b'=') {
            self.idx += 1;
            SyntaxKind::AsteriskEquals
        } else {
            SyntaxKind::Asterisk
        }
    }

    fn read_slash_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        if self.peek_at(0) == Some(b'=') {
            self.idx += 1;
            SyntaxKind::SlashEquals
        } else {
            SyntaxKind::Slash
        }
    }

    fn read_percent_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        if self.peek_at(0) == Some(b'=') {
            self.idx += 1;
            SyntaxKind::PercentEquals
        } else {
            SyntaxKind::Percent
        }
    }

    fn read_less_than_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        match self.peek_at(0) {
            Some(b'<') => {
                self.idx += 1;
                if self.peek_at(0) == Some(b'=') {
                    self.idx += 1;
                    SyntaxKind::LessThanLessThanEquals
                } else {
                    SyntaxKind::LessThanLessThan
                }
            }
            Some(b'=') => { self.idx += 1; SyntaxKind::LessThanEquals }
            Some(b'/') => { self.idx += 1; SyntaxKind::LessThanSlash }
            _ => SyntaxKind::LessThan
        }
    }

    fn read_greater_than_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        if self.peek_at(0) == Some(b'>') {
            self.idx += 1;
            if self.peek_at(0) == Some(b'>') {
                self.idx += 1;
                if self.peek_at(0) == Some(b'=') {
                    self.idx += 1;
                    SyntaxKind::GreaterThanGreaterThanGreaterThanEquals
                } else {
                    SyntaxKind::GreaterThanGreaterThanGreaterThan
                }
            } else if self.peek_at(0) == Some(b'=') {
                self.idx += 1;
                SyntaxKind::GreaterThanGreaterThanEquals
            } else {
                SyntaxKind::GreaterThanGreaterThan
            }
        } else if self.peek_at(0) == Some(b'=') {
            self.idx += 1;
            SyntaxKind::GreaterThanEquals
        } else {
            SyntaxKind::GreaterThan
        }
    }

    fn read_equals_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        if self.peek_at(0) == Some(b'=') {
            self.idx += 1;
            if self.peek_at(0) == Some(b'=') {
                self.idx += 1;
                SyntaxKind::EqualsEqualsEquals
            } else {
                SyntaxKind::EqualsEquals
            }
        } else if self.peek_at(0) == Some(b'>') {
            self.idx += 1;
            SyntaxKind::EqualsGreaterThan
        } else {
            SyntaxKind::Equals
        }
    }

    fn read_exclamation_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        if self.peek_at(0) == Some(b'=') {
            self.idx += 1;
            if self.peek_at(0) == Some(b'=') {
                self.idx += 1;
                SyntaxKind::ExclamationEqualsEquals
            } else {
                SyntaxKind::ExclamationEquals
            }
        } else {
            SyntaxKind::Exclamation
        }
    }

    fn read_ampersand_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        match self.peek_at(0) {
            Some(b'&') => {
                self.idx += 1;
                if self.peek_at(0) == Some(b'=') {
                    self.idx += 1;
                    SyntaxKind::AmpersandAmpersandEquals
                } else {
                    SyntaxKind::AmpersandAmpersand
                }
            }
            Some(b'=') => { self.idx += 1; SyntaxKind::AmpersandEquals }
            _ => SyntaxKind::Ampersand
        }
    }

    fn read_pipe_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        match self.peek_at(0) {
            Some(b'|') => {
                self.idx += 1;
                if self.peek_at(0) == Some(b'=') {
                    self.idx += 1;
                    SyntaxKind::BarBarEquals
                } else {
                    SyntaxKind::BarBar
                }
            }
            Some(b'=') => { self.idx += 1; SyntaxKind::BarEquals }
            _ => SyntaxKind::Bar
        }
    }

    fn read_caret_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        if self.peek_at(0) == Some(b'=') {
            self.idx += 1;
            SyntaxKind::CaretEquals
        } else {
            SyntaxKind::Caret
        }
    }

    fn read_question_variants(&mut self) -> SyntaxKind {
        self.idx += 1;
        match self.peek_at(0) {
            Some(b'?') => {
                self.idx += 1;
                if self.peek_at(0) == Some(b'=') {
                    self.idx += 1;
                    SyntaxKind::QuestionQuestionEquals
                } else {
                    SyntaxKind::QuestionQuestion
                }
            }
            Some(b'.') => {
                if !matches!(self.peek_at(1), Some(b'0'..=b'9')) {
                    self.idx += 1;
                    SyntaxKind::QuestionDot
                } else {
                    SyntaxKind::Question
                }
            }
            _ => SyntaxKind::Question
        }
    }

    fn read_string_literal(&mut self, start_pos: usize, nl_flag: bool) -> Tok {
        let quote_char = self.bytes[self.idx];
        self.idx += 1;
        let content_start = self.idx;
        
        while self.idx < self.bytes.len() {
            let ch = self.bytes[self.idx];
            if ch == quote_char {
                let content = String::from_utf8_lossy(&self.bytes[content_start..self.idx]).into_owned();
                self.idx += 1;
                return Tok {
                    k: SyntaxKind::StringLiteral,
                    p0: start_pos as u32,
                    p1: self.idx as u32,
                    txt: Some(content.into_boxed_str()),
                    num: None,
                    nl: nl_flag,
                };
            }
            if ch == b'\\' && self.idx + 1 < self.bytes.len() {
                self.idx += 2;
            } else {
                self.idx += 1;
            }
        }

        let content = String::from_utf8_lossy(&self.bytes[content_start..self.idx]).into_owned();
        Tok {
            k: SyntaxKind::StringLiteral,
            p0: start_pos as u32,
            p1: self.idx as u32,
            txt: Some(content.into_boxed_str()),
            num: None,
            nl: nl_flag,
        }
    }

    fn read_numeric(&mut self, start_pos: usize, nl_flag: bool) -> Tok {
        while self.idx < self.bytes.len() {
            match self.bytes[self.idx] {
                b'0'..=b'9' | b'.' | b'e' | b'E' | b'x' | b'X' | b'b' | b'B' | b'o' | b'O' 
                | b'a'..=b'f' | b'A'..=b'F' | b'_' | b'+' | b'-' => self.idx += 1,
                b'n' => {
                    self.idx += 1;
                    let text = String::from_utf8_lossy(&self.bytes[start_pos..self.idx - 1]).into_owned();
                    return Tok {
                        k: SyntaxKind::BigIntLiteral,
                        p0: start_pos as u32,
                        p1: self.idx as u32,
                        txt: Some(text.into_boxed_str()),
                        num: None,
                        nl: nl_flag,
                    };
                }
                _ => break,
            }
        }

        let text = String::from_utf8_lossy(&self.bytes[start_pos..self.idx]);
        let parsed_value = text.replace('_', "").parse::<f64>().unwrap_or(f64::NAN);

        Tok {
            k: SyntaxKind::NumericLiteral,
            p0: start_pos as u32,
            p1: self.idx as u32,
            txt: None,
            num: Some(parsed_value),
            nl: nl_flag,
        }
    }

    fn read_identifier(&mut self, start_pos: usize, nl_flag: bool) -> Tok {
        while self.idx < self.bytes.len() && is_id_continue(self.bytes[self.idx]) {
            self.idx += 1;
        }

        let text = String::from_utf8_lossy(&self.bytes[start_pos..self.idx]).into_owned();
        let kind = keyword_from_str(&text).unwrap_or(SyntaxKind::Identifier);

        Tok {
            k: kind,
            p0: start_pos as u32,
            p1: self.idx as u32,
            txt: Some(text.into_boxed_str()),
            num: None,
            nl: nl_flag,
        }
    }
}

fn is_id_start(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'$')
}

fn is_id_continue(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'$')
}

fn keyword_from_str(s: &str) -> Option<SyntaxKind> {
    Some(match s {
        "break" => SyntaxKind::BreakKeyword,
        "case" => SyntaxKind::CaseKeyword,
        "catch" => SyntaxKind::CatchKeyword,
        "class" => SyntaxKind::ClassKeyword,
        "const" => SyntaxKind::ConstKeyword,
        "continue" => SyntaxKind::ContinueKeyword,
        "debugger" => SyntaxKind::DebuggerKeyword,
        "default" => SyntaxKind::DefaultKeyword,
        "delete" => SyntaxKind::DeleteKeyword,
        "do" => SyntaxKind::DoKeyword,
        "else" => SyntaxKind::ElseKeyword,
        "enum" => SyntaxKind::EnumKeyword,
        "export" => SyntaxKind::ExportKeyword,
        "extends" => SyntaxKind::ExtendsKeyword,
        "false" => SyntaxKind::FalseKeyword,
        "finally" => SyntaxKind::FinallyKeyword,
        "for" => SyntaxKind::ForKeyword,
        "function" => SyntaxKind::FunctionKeyword,
        "if" => SyntaxKind::IfKeyword,
        "import" => SyntaxKind::ImportKeyword,
        "in" => SyntaxKind::InKeyword,
        "instanceof" => SyntaxKind::InstanceOfKeyword,
        "new" => SyntaxKind::NewKeyword,
        "null" => SyntaxKind::NullKeyword,
        "return" => SyntaxKind::ReturnKeyword,
        "super" => SyntaxKind::SuperKeyword,
        "switch" => SyntaxKind::SwitchKeyword,
        "this" => SyntaxKind::ThisKeyword,
        "throw" => SyntaxKind::ThrowKeyword,
        "true" => SyntaxKind::TrueKeyword,
        "try" => SyntaxKind::TryKeyword,
        "typeof" => SyntaxKind::TypeOfKeyword,
        "var" => SyntaxKind::VarKeyword,
        "void" => SyntaxKind::VoidKeyword,
        "while" => SyntaxKind::WhileKeyword,
        "with" => SyntaxKind::WithKeyword,
        "implements" => SyntaxKind::ImplementsKeyword,
        "interface" => SyntaxKind::InterfaceKeyword,
        "let" => SyntaxKind::LetKeyword,
        "package" => SyntaxKind::PackageKeyword,
        "private" => SyntaxKind::PrivateKeyword,
        "protected" => SyntaxKind::ProtectedKeyword,
        "public" => SyntaxKind::PublicKeyword,
        "static" => SyntaxKind::StaticKeyword,
        "yield" => SyntaxKind::YieldKeyword,
        "abstract" => SyntaxKind::AbstractKeyword,
        "as" => SyntaxKind::AsKeyword,
        "async" => SyntaxKind::AsyncKeyword,
        "await" => SyntaxKind::AwaitKeyword,
        "any" => SyntaxKind::AnyKeyword,
        "boolean" => SyntaxKind::BooleanKeyword,
        "constructor" => SyntaxKind::ConstructorKeyword,
        "declare" => SyntaxKind::DeclareKeyword,
        "get" => SyntaxKind::GetKeyword,
        "infer" => SyntaxKind::InferKeyword,
        "is" => SyntaxKind::IsKeyword,
        "keyof" => SyntaxKind::KeyOfKeyword,
        "module" => SyntaxKind::ModuleKeyword,
        "namespace" => SyntaxKind::NamespaceKeyword,
        "never" => SyntaxKind::NeverKeyword,
        "readonly" => SyntaxKind::ReadonlyKeyword,
        "require" => SyntaxKind::RequireKeyword,
        "number" => SyntaxKind::NumberKeyword,
        "object" => SyntaxKind::ObjectKeyword,
        "set" => SyntaxKind::SetKeyword,
        "string" => SyntaxKind::StringKeyword,
        "symbol" => SyntaxKind::SymbolKeyword,
        "type" => SyntaxKind::TypeKeyword,
        "undefined" => SyntaxKind::UndefinedKeyword,
        "unique" => SyntaxKind::UniqueKeyword,
        "unknown" => SyntaxKind::UnknownKeyword,
        "from" => SyntaxKind::FromKeyword,
        "global" => SyntaxKind::GlobalKeyword,
        "bigint" => SyntaxKind::BigIntKeyword,
        "override" => SyntaxKind::OverrideKeyword,
        "of" => SyntaxKind::OfKeyword,
        "satisfies" => SyntaxKind::SatisfiesKeyword,
        "asserts" => SyntaxKind::AssertsKeyword,
        "assert" => SyntaxKind::AssertKeyword,
        "out" => SyntaxKind::OutKeyword,
        "using" => SyntaxKind::UsingKeyword,
        "accessor" => SyntaxKind::AccessorKeyword,
        "intrinsic" => SyntaxKind::IntrinsicKeyword,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_punctuation() {
        let mut lex = Lexer::create("{ } ( ) [ ]");
        assert_eq!(lex.produce_next().k, SyntaxKind::OpenBrace);
        assert_eq!(lex.produce_next().k, SyntaxKind::CloseBrace);
        assert_eq!(lex.produce_next().k, SyntaxKind::OpenParen);
        assert_eq!(lex.produce_next().k, SyntaxKind::CloseParen);
        assert_eq!(lex.produce_next().k, SyntaxKind::OpenBracket);
        assert_eq!(lex.produce_next().k, SyntaxKind::CloseBracket);
        assert_eq!(lex.produce_next().k, SyntaxKind::EndOfFile);
    }

    #[test]
    fn lex_keywords() {
        let mut lex = Lexer::create("const let var function class interface");
        assert_eq!(lex.produce_next().k, SyntaxKind::ConstKeyword);
        assert_eq!(lex.produce_next().k, SyntaxKind::LetKeyword);
        assert_eq!(lex.produce_next().k, SyntaxKind::VarKeyword);
        assert_eq!(lex.produce_next().k, SyntaxKind::FunctionKeyword);
        assert_eq!(lex.produce_next().k, SyntaxKind::ClassKeyword);
        assert_eq!(lex.produce_next().k, SyntaxKind::InterfaceKeyword);
    }

    #[test]
    fn lex_operators() {
        let mut lex = Lexer::create("+ ++ += === !==");
        assert_eq!(lex.produce_next().k, SyntaxKind::Plus);
        assert_eq!(lex.produce_next().k, SyntaxKind::PlusPlus);
        assert_eq!(lex.produce_next().k, SyntaxKind::PlusEquals);
        assert_eq!(lex.produce_next().k, SyntaxKind::EqualsEqualsEquals);
        assert_eq!(lex.produce_next().k, SyntaxKind::ExclamationEqualsEquals);
    }

    #[test]
    fn lex_string() {
        let mut lex = Lexer::create("\"hello world\"");
        let tok = lex.produce_next();
        assert_eq!(tok.k, SyntaxKind::StringLiteral);
        assert_eq!(tok.txt.as_deref(), Some("hello world"));
    }

    #[test]
    fn lex_number() {
        let mut lex = Lexer::create("42 3.14");
        let tok1 = lex.produce_next();
        assert_eq!(tok1.k, SyntaxKind::NumericLiteral);
        assert_eq!(tok1.num, Some(42.0));
        
        let tok2 = lex.produce_next();
        assert_eq!(tok2.k, SyntaxKind::NumericLiteral);
        assert_eq!(tok2.num, Some(3.14));
    }

    #[test]
    fn lex_identifier() {
        let mut lex = Lexer::create("myVar _private $jquery");
        let tok1 = lex.produce_next();
        assert_eq!(tok1.k, SyntaxKind::Identifier);
        assert_eq!(tok1.txt.as_deref(), Some("myVar"));
        
        let tok2 = lex.produce_next();
        assert_eq!(tok2.k, SyntaxKind::Identifier);
        assert_eq!(tok2.txt.as_deref(), Some("_private"));
        
        let tok3 = lex.produce_next();
        assert_eq!(tok3.k, SyntaxKind::Identifier);
        assert_eq!(tok3.txt.as_deref(), Some("$jquery"));
    }

    #[test]
    fn lex_comments() {
        let mut lex = Lexer::create("a // comment\nb /* block */ c");
        assert_eq!(lex.produce_next().txt.as_deref(), Some("a"));
        let b_tok = lex.produce_next();
        assert_eq!(b_tok.txt.as_deref(), Some("b"));
        assert!(b_tok.nl);
        assert_eq!(lex.produce_next().txt.as_deref(), Some("c"));
    }
}
