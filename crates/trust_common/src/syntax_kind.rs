//! Syntax kinds for tokens and AST nodes
//!
//! This module defines all the different kinds of syntax elements
//! that can appear in TypeScript source code.

use serde::{Deserialize, Serialize};

/// Represents the kind of a syntax element (token or node)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum SyntaxKind {
    // Special markers
    EndOfFile = 0,
    Unknown = 1,

    // Trivia (whitespace, comments)
    WhitespaceTrivia = 2,
    NewlineTrivia = 3,
    SingleLineComment = 4,
    MultiLineComment = 5,

    // Literals
    NumericLiteral = 10,
    BigIntLiteral = 11,
    StringLiteral = 12,
    RegexLiteral = 13,
    NoSubstitutionTemplate = 14,
    TemplateHead = 15,
    TemplateMiddle = 16,
    TemplateTail = 17,

    // Punctuation
    OpenBrace = 20,        // {
    CloseBrace = 21,       // }
    OpenParen = 22,        // (
    CloseParen = 23,       // )
    OpenBracket = 24,      // [
    CloseBracket = 25,     // ]
    Dot = 26,              // .
    DotDotDot = 27,        // ...
    Semicolon = 28,        // ;
    Comma = 29,            // ,
    QuestionDot = 30,      // ?.
    LessThan = 31,         // <
    LessThanSlash = 32,    // </
    GreaterThan = 33,      // >
    LessThanEquals = 34,   // <=
    GreaterThanEquals = 35,// >=
    EqualsEquals = 36,     // ==
    ExclamationEquals = 37,// !=
    EqualsEqualsEquals = 38,   // ===
    ExclamationEqualsEquals = 39, // !==
    EqualsGreaterThan = 40,// =>
    Plus = 41,             // +
    Minus = 42,            // -
    Asterisk = 43,         // *
    AsteriskAsterisk = 44, // **
    Slash = 45,            // /
    Percent = 46,          // %
    PlusPlus = 47,         // ++
    MinusMinus = 48,       // --
    LessThanLessThan = 49, // <<
    GreaterThanGreaterThan = 50, // >>
    GreaterThanGreaterThanGreaterThan = 51, // >>>
    Ampersand = 52,        // &
    Bar = 53,              // |
    Caret = 54,            // ^
    Exclamation = 55,      // !
    Tilde = 56,            // ~
    AmpersandAmpersand = 57,   // &&
    BarBar = 58,           // ||
    Question = 59,         // ?
    Colon = 60,            // :
    At = 61,               // @
    QuestionQuestion = 62, // ??
    BackTick = 63,         // `
    HashToken = 64,        // #

    // Assignments
    Equals = 70,           // =
    PlusEquals = 71,       // +=
    MinusEquals = 72,      // -=
    AsteriskEquals = 73,   // *=
    AsteriskAsteriskEquals = 74, // **=
    SlashEquals = 75,      // /=
    PercentEquals = 76,    // %=
    LessThanLessThanEquals = 77, // <<=
    GreaterThanGreaterThanEquals = 78, // >>=
    GreaterThanGreaterThanGreaterThanEquals = 79, // >>>=
    AmpersandEquals = 80,  // &=
    BarEquals = 81,        // |=
    CaretEquals = 82,      // ^=
    BarBarEquals = 83,     // ||=
    AmpersandAmpersandEquals = 84, // &&=
    QuestionQuestionEquals = 85, // ??=

    // Identifiers and keywords
    Identifier = 90,

    // Reserved words
    BreakKeyword = 100,
    CaseKeyword = 101,
    CatchKeyword = 102,
    ClassKeyword = 103,
    ConstKeyword = 104,
    ContinueKeyword = 105,
    DebuggerKeyword = 106,
    DefaultKeyword = 107,
    DeleteKeyword = 108,
    DoKeyword = 109,
    ElseKeyword = 110,
    EnumKeyword = 111,
    ExportKeyword = 112,
    ExtendsKeyword = 113,
    FalseKeyword = 114,
    FinallyKeyword = 115,
    ForKeyword = 116,
    FunctionKeyword = 117,
    IfKeyword = 118,
    ImportKeyword = 119,
    InKeyword = 120,
    InstanceOfKeyword = 121,
    NewKeyword = 122,
    NullKeyword = 123,
    ReturnKeyword = 124,
    SuperKeyword = 125,
    SwitchKeyword = 126,
    ThisKeyword = 127,
    ThrowKeyword = 128,
    TrueKeyword = 129,
    TryKeyword = 130,
    TypeOfKeyword = 131,
    VarKeyword = 132,
    VoidKeyword = 133,
    WhileKeyword = 134,
    WithKeyword = 135,

    // Strict mode reserved words
    ImplementsKeyword = 140,
    InterfaceKeyword = 141,
    LetKeyword = 142,
    PackageKeyword = 143,
    PrivateKeyword = 144,
    ProtectedKeyword = 145,
    PublicKeyword = 146,
    StaticKeyword = 147,
    YieldKeyword = 148,

    // Contextual keywords
    AbstractKeyword = 150,
    AccessorKeyword = 151,
    AsKeyword = 152,
    AssertsKeyword = 153,
    AssertKeyword = 154,
    AnyKeyword = 155,
    AsyncKeyword = 156,
    AwaitKeyword = 157,
    BooleanKeyword = 158,
    ConstructorKeyword = 159,
    DeclareKeyword = 160,
    GetKeyword = 161,
    InferKeyword = 162,
    IntrinsicKeyword = 163,
    IsKeyword = 164,
    KeyOfKeyword = 165,
    ModuleKeyword = 166,
    NamespaceKeyword = 167,
    NeverKeyword = 168,
    OutKeyword = 169,
    ReadonlyKeyword = 170,
    RequireKeyword = 171,
    NumberKeyword = 172,
    ObjectKeyword = 173,
    SatisfiesKeyword = 174,
    SetKeyword = 175,
    StringKeyword = 176,
    SymbolKeyword = 177,
    TypeKeyword = 178,
    UndefinedKeyword = 179,
    UniqueKeyword = 180,
    UnknownKeyword = 181,
    UsingKeyword = 182,
    FromKeyword = 183,
    GlobalKeyword = 184,
    BigIntKeyword = 185,
    OverrideKeyword = 186,
    OfKeyword = 187,

    // JSX
    JsxText = 200,
    JsxTextAllWhiteSpaces = 201,

    // Nodes - Names
    QualifiedName = 210,
    ComputedPropertyName = 211,

    // Nodes - Signature elements
    TypeParameter = 220,
    Parameter = 221,
    Decorator = 222,

    // Nodes - TypeScript type members
    PropertySignature = 230,
    PropertyDeclaration = 231,
    MethodSignature = 232,
    MethodDeclaration = 233,
    ClassStaticBlockDeclaration = 234,
    Constructor = 235,
    GetAccessor = 236,
    SetAccessor = 237,
    CallSignature = 238,
    ConstructSignature = 239,
    IndexSignature = 240,

    // Nodes - Type nodes
    TypePredicate = 250,
    TypeReference = 251,
    FunctionType = 252,
    ConstructorType = 253,
    TypeQuery = 254,
    TypeLiteral = 255,
    ArrayType = 256,
    TupleType = 257,
    OptionalType = 258,
    RestType = 259,
    UnionType = 260,
    IntersectionType = 261,
    ConditionalType = 262,
    InferType = 263,
    ParenthesizedType = 264,
    ThisType = 265,
    TypeOperator = 266,
    IndexedAccessType = 267,
    MappedType = 268,
    LiteralType = 269,
    NamedTupleMember = 270,
    TemplateLiteralType = 271,
    TemplateLiteralTypeSpan = 272,
    ImportType = 273,

    // Nodes - Binding patterns
    ObjectBindingPattern = 280,
    ArrayBindingPattern = 281,
    BindingElement = 282,

    // Nodes - Expressions
    ArrayLiteralExpression = 290,
    ObjectLiteralExpression = 291,
    PropertyAccessExpression = 292,
    ElementAccessExpression = 293,
    CallExpression = 294,
    NewExpression = 295,
    TaggedTemplateExpression = 296,
    TypeAssertionExpression = 297,
    ParenthesizedExpression = 298,
    FunctionExpression = 299,
    ArrowFunction = 300,
    DeleteExpression = 301,
    TypeOfExpression = 302,
    VoidExpression = 303,
    AwaitExpression = 304,
    PrefixUnaryExpression = 305,
    PostfixUnaryExpression = 306,
    BinaryExpression = 307,
    ConditionalExpression = 308,
    TemplateExpression = 309,
    YieldExpression = 310,
    SpreadElement = 311,
    ClassExpression = 312,
    OmittedExpression = 313,
    ExpressionWithTypeArguments = 314,
    AsExpression = 315,
    NonNullExpression = 316,
    MetaProperty = 317,
    SyntheticExpression = 318,
    SatisfiesExpression = 319,

    // Nodes - Misc
    TemplateSpan = 330,
    SemicolonClassElement = 331,

    // Nodes - Element
    Block = 340,
    EmptyStatement = 341,
    VariableStatement = 342,
    ExpressionStatement = 343,
    IfStatement = 344,
    DoStatement = 345,
    WhileStatement = 346,
    ForStatement = 347,
    ForInStatement = 348,
    ForOfStatement = 349,
    ContinueStatement = 350,
    BreakStatement = 351,
    ReturnStatement = 352,
    WithStatement = 353,
    SwitchStatement = 354,
    LabeledStatement = 355,
    ThrowStatement = 356,
    TryStatement = 357,
    DebuggerStatement = 358,
    VariableDeclaration = 359,
    VariableDeclarationList = 360,
    FunctionDeclaration = 361,
    ClassDeclaration = 362,
    InterfaceDeclaration = 363,
    TypeAliasDeclaration = 364,
    EnumDeclaration = 365,
    ModuleDeclaration = 366,
    ModuleBlock = 367,
    CaseBlock = 368,
    NamespaceExportDeclaration = 369,
    ImportEqualsDeclaration = 370,
    ImportDeclaration = 371,
    ImportClause = 372,
    NamespaceImport = 373,
    NamedImports = 374,
    ImportSpecifier = 375,
    ExportAssignment = 376,
    ExportDeclaration = 377,
    NamedExports = 378,
    NamespaceExport = 379,
    ExportSpecifier = 380,
    MissingDeclaration = 381,

    // Module references
    ExternalModuleReference = 390,

    // JSX nodes
    JsxElement = 400,
    JsxSelfClosingElement = 401,
    JsxOpeningElement = 402,
    JsxClosingElement = 403,
    JsxFragment = 404,
    JsxOpeningFragment = 405,
    JsxClosingFragment = 406,
    JsxAttribute = 407,
    JsxAttributes = 408,
    JsxSpreadAttribute = 409,
    JsxExpression = 410,
    JsxNamespacedName = 411,

    // Clauses
    CaseClause = 420,
    DefaultClause = 421,
    HeritageClause = 422,
    CatchClause = 423,
    AssertClause = 424,
    AssertEntry = 425,
    ImportTypeAssertionContainer = 426,

    // Property assignments
    PropertyAssignment = 430,
    ShorthandPropertyAssignment = 431,
    SpreadAssignment = 432,

    // Enum member
    EnumMember = 440,

    // Source file and bundles
    SourceFile = 450,
    Bundle = 451,

    // Synthetic nodes
    SyntheticReferenceExpression = 460,

    // JSDoc nodes
    JSDocTypeExpression = 500,
    JSDocNameReference = 501,
    JSDocMemberName = 502,
    JSDocAllType = 503,
    JSDocUnknownType = 504,
    JSDocNullableType = 505,
    JSDocNonNullableType = 506,
    JSDocOptionalType = 507,
    JSDocFunctionType = 508,
    JSDocVariadicType = 509,
    JSDocNamepathType = 510,
    JSDocComment = 511,
    JSDocText = 512,
    JSDocTypeLiteral = 513,
    JSDocSignature = 514,
    JSDocLink = 515,
    JSDocLinkCode = 516,
    JSDocLinkPlain = 517,
    JSDocTag = 518,
    JSDocAugmentsTag = 519,
    JSDocImplementsTag = 520,
    JSDocAuthorTag = 521,
    JSDocDeprecatedTag = 522,
    JSDocClassTag = 523,
    JSDocPublicTag = 524,
    JSDocPrivateTag = 525,
    JSDocProtectedTag = 526,
    JSDocReadonlyTag = 527,
    JSDocOverrideTag = 528,
    JSDocCallbackTag = 529,
    JSDocOverloadTag = 530,
    JSDocEnumTag = 531,
    JSDocParameterTag = 532,
    JSDocReturnTag = 533,
    JSDocThisTag = 534,
    JSDocTypeTag = 535,
    JSDocTemplateTag = 536,
    JSDocTypedefTag = 537,
    JSDocSeeTag = 538,
    JSDocPropertyTag = 539,
    JSDocThrowsTag = 540,
    JSDocSatisfiesTag = 541,
    JSDocImportTag = 542,

    // Transformation nodes
    NotEmittedStatement = 600,
    PartiallyEmittedExpression = 601,
    CommaListExpression = 602,
    SyntheticLeadingComment = 603,
    SyntheticTrailingComment = 604,
}

impl SyntaxKind {
    /// Check if this is a trivia kind (whitespace or comment)
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WhitespaceTrivia
                | SyntaxKind::NewlineTrivia
                | SyntaxKind::SingleLineComment
                | SyntaxKind::MultiLineComment
        )
    }

    /// Check if this is a comment kind
    pub fn is_comment(self) -> bool {
        matches!(
            self,
            SyntaxKind::SingleLineComment | SyntaxKind::MultiLineComment
        )
    }

    /// Check if this is a literal kind
    pub fn is_literal(self) -> bool {
        matches!(
            self,
            SyntaxKind::NumericLiteral
                | SyntaxKind::BigIntLiteral
                | SyntaxKind::StringLiteral
                | SyntaxKind::RegexLiteral
                | SyntaxKind::NoSubstitutionTemplate
        )
    }

    /// Check if this is a template literal part
    pub fn is_template_literal_kind(self) -> bool {
        matches!(
            self,
            SyntaxKind::NoSubstitutionTemplate
                | SyntaxKind::TemplateHead
                | SyntaxKind::TemplateMiddle
                | SyntaxKind::TemplateTail
        )
    }

    /// Check if this is a keyword
    pub fn is_keyword(self) -> bool {
        let val = self as u16;
        val >= SyntaxKind::BreakKeyword as u16 && val <= SyntaxKind::OfKeyword as u16
    }

    /// Check if this is a reserved word (cannot be used as identifier)
    pub fn is_reserved_word(self) -> bool {
        let val = self as u16;
        val >= SyntaxKind::BreakKeyword as u16 && val <= SyntaxKind::WithKeyword as u16
    }

    /// Check if this is a contextual keyword
    pub fn is_contextual_keyword(self) -> bool {
        let val = self as u16;
        val >= SyntaxKind::AbstractKeyword as u16 && val <= SyntaxKind::OfKeyword as u16
    }

    /// Check if this is an assignment operator
    pub fn is_assignment_operator(self) -> bool {
        let val = self as u16;
        val >= SyntaxKind::Equals as u16 && val <= SyntaxKind::QuestionQuestionEquals as u16
    }

    /// Check if this is a binary operator
    pub fn is_binary_operator(self) -> bool {
        self.is_assignment_operator()
            || matches!(
                self,
                SyntaxKind::Plus
                    | SyntaxKind::Minus
                    | SyntaxKind::Asterisk
                    | SyntaxKind::AsteriskAsterisk
                    | SyntaxKind::Slash
                    | SyntaxKind::Percent
                    | SyntaxKind::LessThan
                    | SyntaxKind::LessThanEquals
                    | SyntaxKind::GreaterThan
                    | SyntaxKind::GreaterThanEquals
                    | SyntaxKind::EqualsEquals
                    | SyntaxKind::ExclamationEquals
                    | SyntaxKind::EqualsEqualsEquals
                    | SyntaxKind::ExclamationEqualsEquals
                    | SyntaxKind::LessThanLessThan
                    | SyntaxKind::GreaterThanGreaterThan
                    | SyntaxKind::GreaterThanGreaterThanGreaterThan
                    | SyntaxKind::Ampersand
                    | SyntaxKind::Bar
                    | SyntaxKind::Caret
                    | SyntaxKind::AmpersandAmpersand
                    | SyntaxKind::BarBar
                    | SyntaxKind::QuestionQuestion
                    | SyntaxKind::InKeyword
                    | SyntaxKind::InstanceOfKeyword
            )
    }

    /// Check if this is a prefix unary operator
    pub fn is_prefix_unary_operator(self) -> bool {
        matches!(
            self,
            SyntaxKind::Plus
                | SyntaxKind::Minus
                | SyntaxKind::Tilde
                | SyntaxKind::Exclamation
                | SyntaxKind::PlusPlus
                | SyntaxKind::MinusMinus
        )
    }

    /// Check if this is a postfix unary operator
    pub fn is_postfix_unary_operator(self) -> bool {
        matches!(self, SyntaxKind::PlusPlus | SyntaxKind::MinusMinus)
    }

    /// Check if this is a modifier keyword
    pub fn is_modifier_kind(self) -> bool {
        matches!(
            self,
            SyntaxKind::AbstractKeyword
                | SyntaxKind::AccessorKeyword
                | SyntaxKind::AsyncKeyword
                | SyntaxKind::ConstKeyword
                | SyntaxKind::DeclareKeyword
                | SyntaxKind::DefaultKeyword
                | SyntaxKind::ExportKeyword
                | SyntaxKind::InKeyword
                | SyntaxKind::OutKeyword
                | SyntaxKind::OverrideKeyword
                | SyntaxKind::PrivateKeyword
                | SyntaxKind::ProtectedKeyword
                | SyntaxKind::PublicKeyword
                | SyntaxKind::ReadonlyKeyword
                | SyntaxKind::StaticKeyword
        )
    }

    /// Check if this is a type node kind
    pub fn is_type_node(self) -> bool {
        let val = self as u16;
        val >= SyntaxKind::TypePredicate as u16 && val <= SyntaxKind::ImportType as u16
    }

    /// Check if this is a JSX token
    pub fn is_jsx_token(self) -> bool {
        matches!(
            self,
            SyntaxKind::JsxText | SyntaxKind::JsxTextAllWhiteSpaces | SyntaxKind::LessThanSlash
        )
    }
}

impl Default for SyntaxKind {
    fn default() -> Self {
        SyntaxKind::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_keyword() {
        assert!(SyntaxKind::IfKeyword.is_keyword());
        assert!(SyntaxKind::ConstKeyword.is_keyword());
        assert!(SyntaxKind::AsyncKeyword.is_keyword());
        assert!(!SyntaxKind::Identifier.is_keyword());
        assert!(!SyntaxKind::NumericLiteral.is_keyword());
    }

    #[test]
    fn test_is_assignment() {
        assert!(SyntaxKind::Equals.is_assignment_operator());
        assert!(SyntaxKind::PlusEquals.is_assignment_operator());
        assert!(!SyntaxKind::Plus.is_assignment_operator());
        assert!(!SyntaxKind::EqualsEquals.is_assignment_operator());
    }

    #[test]
    fn test_is_trivia() {
        assert!(SyntaxKind::WhitespaceTrivia.is_trivia());
        assert!(SyntaxKind::SingleLineComment.is_trivia());
        assert!(!SyntaxKind::Identifier.is_trivia());
    }
}
