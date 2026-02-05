//! AST node definitions

use trust_common::{SyntaxKind, TextSpan};
use std::sync::Arc;

pub type NodeId = u32;

#[derive(Debug, Clone)]
pub struct SrcRange {
    pub start: u32,
    pub end: u32,
}

impl SrcRange {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
    
    pub fn to_span(&self) -> TextSpan {
        TextSpan::from_bounds(self.start, self.end)
    }
}

#[derive(Debug)]
pub struct NodeBase {
    pub id: NodeId,
    pub kind: SyntaxKind,
    pub range: SrcRange,
    pub flags: NodeFlags,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NodeFlags(u32);

impl NodeFlags {
    pub const NONE: u32 = 0;
    pub const HAS_IMPLICIT_RETURN: u32 = 1 << 0;
    pub const HAS_EXPLICIT_RETURN: u32 = 1 << 1;
    pub const GLOBAL_AUGMENTATION: u32 = 1 << 2;
    pub const HAS_ASYNC_FUNCTIONS: u32 = 1 << 3;
    pub const DISALLOW_IN_CONTEXT: u32 = 1 << 4;
    pub const YIELD_CONTEXT: u32 = 1 << 5;
    pub const DECORATOR_CONTEXT: u32 = 1 << 6;
    pub const AWAIT_CONTEXT: u32 = 1 << 7;
    pub const THIS_NODE_HAS_ERROR: u32 = 1 << 8;
    pub const JAVASCRIPT_FILE: u32 = 1 << 9;
    pub const THIS_NODE_OR_ANY_SUB_NODE_HAS_ERROR: u32 = 1 << 10;
    pub const HAS_AGGREGATED_CHILD_DATA: u32 = 1 << 11;

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn has(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }

    pub fn set(&mut self, flag: u32) {
        self.0 |= flag;
    }
}

#[derive(Debug)]
pub enum Node {
    SourceFile(Box<SourceFileNode>),
    Stmt(StmtNode),
    Expr(ExprNode),
    Decl(DeclNode),
    TypeNode(TypeNode),
    Pattern(PatternNode),
}

#[derive(Debug)]
pub struct SourceFileNode {
    pub base: NodeBase,
    pub file_name: String,
    pub statements: Vec<Node>,
    pub end_of_file_token: NodeBase,
}

#[derive(Debug)]
pub enum StmtNode {
    Block(BlockStmt),
    Variable(VariableStmt),
    Expression(ExpressionStmt),
    If(IfStmt),
    While(WhileStmt),
    Do(DoStmt),
    For(ForStmt),
    ForIn(ForInStmt),
    ForOf(ForOfStmt),
    Return(ReturnStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Switch(SwitchStmt),
    Throw(ThrowStmt),
    Try(TryStmt),
    Empty(EmptyStmt),
    Debugger(DebuggerStmt),
    Labeled(LabeledStmt),
    With(WithStmt),
}

#[derive(Debug)]
pub struct BlockStmt {
    pub base: NodeBase,
    pub statements: Vec<Node>,
}

#[derive(Debug)]
pub struct VariableStmt {
    pub base: NodeBase,
    pub declaration_list: VariableDeclList,
}

#[derive(Debug)]
pub struct VariableDeclList {
    pub base: NodeBase,
    pub declarations: Vec<VariableDecl>,
    pub decl_kind: VarDeclKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarDeclKind {
    Var,
    Let,
    Const,
    Using,
    AwaitUsing,
}

#[derive(Debug)]
pub struct VariableDecl {
    pub base: NodeBase,
    pub name: PatternNode,
    pub type_annotation: Option<Box<TypeNode>>,
    pub initializer: Option<Box<ExprNode>>,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub base: NodeBase,
    pub condition: Box<ExprNode>,
    pub then_stmt: Box<Node>,
    pub else_stmt: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub base: NodeBase,
    pub condition: Box<ExprNode>,
    pub body: Box<Node>,
}

#[derive(Debug)]
pub struct DoStmt {
    pub base: NodeBase,
    pub body: Box<Node>,
    pub condition: Box<ExprNode>,
}

#[derive(Debug)]
pub struct ForStmt {
    pub base: NodeBase,
    pub initializer: Option<ForInit>,
    pub condition: Option<Box<ExprNode>>,
    pub incrementor: Option<Box<ExprNode>>,
    pub body: Box<Node>,
}

#[derive(Debug)]
pub enum ForInit {
    VarDecl(VariableDeclList),
    Expr(Box<ExprNode>),
}

#[derive(Debug)]
pub struct ForInStmt {
    pub base: NodeBase,
    pub initializer: ForInInit,
    pub expression: Box<ExprNode>,
    pub body: Box<Node>,
}

#[derive(Debug)]
pub struct ForOfStmt {
    pub base: NodeBase,
    pub await_modifier: bool,
    pub initializer: ForInInit,
    pub expression: Box<ExprNode>,
    pub body: Box<Node>,
}

#[derive(Debug)]
pub enum ForInInit {
    VarDecl(VariableDeclList),
    Expr(Box<ExprNode>),
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub base: NodeBase,
    pub expression: Option<Box<ExprNode>>,
}

#[derive(Debug)]
pub struct BreakStmt {
    pub base: NodeBase,
    pub label: Option<Ident>,
}

#[derive(Debug)]
pub struct ContinueStmt {
    pub base: NodeBase,
    pub label: Option<Ident>,
}

#[derive(Debug)]
pub struct SwitchStmt {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
    pub case_block: CaseBlock,
}

#[derive(Debug)]
pub struct CaseBlock {
    pub base: NodeBase,
    pub clauses: Vec<CaseOrDefault>,
}

#[derive(Debug)]
pub enum CaseOrDefault {
    Case(CaseClause),
    Default(DefaultClause),
}

#[derive(Debug)]
pub struct CaseClause {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
    pub statements: Vec<Node>,
}

#[derive(Debug)]
pub struct DefaultClause {
    pub base: NodeBase,
    pub statements: Vec<Node>,
}

#[derive(Debug)]
pub struct ThrowStmt {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
}

#[derive(Debug)]
pub struct TryStmt {
    pub base: NodeBase,
    pub try_block: BlockStmt,
    pub catch_clause: Option<CatchClause>,
    pub finally_block: Option<BlockStmt>,
}

#[derive(Debug)]
pub struct CatchClause {
    pub base: NodeBase,
    pub variable: Option<CatchVar>,
    pub block: BlockStmt,
}

#[derive(Debug)]
pub struct CatchVar {
    pub base: NodeBase,
    pub name: PatternNode,
    pub type_annotation: Option<Box<TypeNode>>,
}

#[derive(Debug)]
pub struct EmptyStmt {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct DebuggerStmt {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct LabeledStmt {
    pub base: NodeBase,
    pub label: Ident,
    pub statement: Box<Node>,
}

#[derive(Debug)]
pub struct WithStmt {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
    pub statement: Box<Node>,
}

#[derive(Debug)]
pub enum ExprNode {
    Ident(Ident),
    Literal(LiteralExpr),
    Array(ArrayExpr),
    Object(ObjectExpr),
    Paren(ParenExpr),
    Call(CallExpr),
    New(NewExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    TaggedTemplate(TaggedTemplateExpr),
    Template(TemplateExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Conditional(ConditionalExpr),
    Assignment(AssignmentExpr),
    Arrow(ArrowFnExpr),
    Function(FunctionExpr),
    Class(ClassExpr),
    Yield(YieldExpr),
    Await(AwaitExpr),
    Spread(SpreadExpr),
    As(AsExpr),
    Satisfies(SatisfiesExpr),
    NonNull(NonNullExpr),
    TypeAssertion(TypeAssertionExpr),
    This(ThisExpr),
    Super(SuperExpr),
    Null(NullExpr),
    True(TrueExpr),
    False(FalseExpr),
    Void(VoidExpr),
    Typeof(TypeofExpr),
    Delete(DeleteExpr),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub base: NodeBase,
    pub text: Arc<str>,
}

#[derive(Debug)]
pub enum LiteralExpr {
    String(StringLit),
    Number(NumberLit),
    BigInt(BigIntLit),
    Regex(RegexLit),
}

#[derive(Debug)]
pub struct StringLit {
    pub base: NodeBase,
    pub value: Arc<str>,
}

#[derive(Debug)]
pub struct NumberLit {
    pub base: NodeBase,
    pub value: f64,
}

#[derive(Debug)]
pub struct BigIntLit {
    pub base: NodeBase,
    pub value: Arc<str>,
}

#[derive(Debug)]
pub struct RegexLit {
    pub base: NodeBase,
    pub pattern: Arc<str>,
    pub flags: Arc<str>,
}

#[derive(Debug)]
pub struct ArrayExpr {
    pub base: NodeBase,
    pub elements: Vec<Option<ExprNode>>,
}

#[derive(Debug)]
pub struct ObjectExpr {
    pub base: NodeBase,
    pub properties: Vec<ObjectProp>,
}

#[derive(Debug)]
pub enum ObjectProp {
    Property(PropertyAssign),
    Shorthand(ShorthandProp),
    Spread(SpreadProp),
    Method(MethodProp),
    Getter(GetterProp),
    Setter(SetterProp),
}

#[derive(Debug)]
pub struct PropertyAssign {
    pub base: NodeBase,
    pub key: PropKey,
    pub value: Box<ExprNode>,
}

#[derive(Debug)]
pub struct ShorthandProp {
    pub base: NodeBase,
    pub name: Ident,
}

#[derive(Debug)]
pub struct SpreadProp {
    pub base: NodeBase,
    pub argument: Box<ExprNode>,
}

#[derive(Debug)]
pub struct MethodProp {
    pub base: NodeBase,
    pub key: PropKey,
    pub params: Vec<ParamDecl>,
    pub body: Option<BlockStmt>,
    pub type_params: Option<TypeParamList>,
    pub return_type: Option<Box<TypeNode>>,
    pub is_async: bool,
    pub is_generator: bool,
}

#[derive(Debug)]
pub struct GetterProp {
    pub base: NodeBase,
    pub key: PropKey,
    pub body: Option<BlockStmt>,
    pub return_type: Option<Box<TypeNode>>,
}

#[derive(Debug)]
pub struct SetterProp {
    pub base: NodeBase,
    pub key: PropKey,
    pub param: ParamDecl,
    pub body: Option<BlockStmt>,
}

#[derive(Debug)]
pub enum PropKey {
    Ident(Ident),
    String(StringLit),
    Number(NumberLit),
    Computed(Box<ExprNode>),
}

#[derive(Debug)]
pub struct ParenExpr {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
}

#[derive(Debug)]
pub struct CallExpr {
    pub base: NodeBase,
    pub callee: Box<ExprNode>,
    pub type_arguments: Option<TypeArgList>,
    pub arguments: Vec<ExprNode>,
}

#[derive(Debug)]
pub struct NewExpr {
    pub base: NodeBase,
    pub callee: Box<ExprNode>,
    pub type_arguments: Option<TypeArgList>,
    pub arguments: Option<Vec<ExprNode>>,
}

#[derive(Debug)]
pub struct MemberExpr {
    pub base: NodeBase,
    pub object: Box<ExprNode>,
    pub property: Ident,
    pub optional: bool,
}

#[derive(Debug)]
pub struct IndexExpr {
    pub base: NodeBase,
    pub object: Box<ExprNode>,
    pub index: Box<ExprNode>,
    pub optional: bool,
}

#[derive(Debug)]
pub struct TaggedTemplateExpr {
    pub base: NodeBase,
    pub tag: Box<ExprNode>,
    pub type_arguments: Option<TypeArgList>,
    pub template: TemplateExpr,
}

#[derive(Debug)]
pub struct TemplateExpr {
    pub base: NodeBase,
    pub head: TemplateHead,
    pub spans: Vec<TemplateSpan>,
}

#[derive(Debug)]
pub struct TemplateHead {
    pub base: NodeBase,
    pub raw: Arc<str>,
    pub cooked: Option<Arc<str>>,
}

#[derive(Debug)]
pub struct TemplateSpan {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
    pub literal: TemplatePart,
}

#[derive(Debug)]
pub enum TemplatePart {
    Middle(TemplateMiddle),
    Tail(TemplateTail),
}

#[derive(Debug)]
pub struct TemplateMiddle {
    pub base: NodeBase,
    pub raw: Arc<str>,
    pub cooked: Option<Arc<str>>,
}

#[derive(Debug)]
pub struct TemplateTail {
    pub base: NodeBase,
    pub raw: Arc<str>,
    pub cooked: Option<Arc<str>>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub base: NodeBase,
    pub operator: SyntaxKind,
    pub operand: Box<ExprNode>,
    pub prefix: bool,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub base: NodeBase,
    pub left: Box<ExprNode>,
    pub operator: SyntaxKind,
    pub right: Box<ExprNode>,
}

#[derive(Debug)]
pub struct ConditionalExpr {
    pub base: NodeBase,
    pub condition: Box<ExprNode>,
    pub when_true: Box<ExprNode>,
    pub when_false: Box<ExprNode>,
}

#[derive(Debug)]
pub struct AssignmentExpr {
    pub base: NodeBase,
    pub left: Box<ExprNode>,
    pub operator: SyntaxKind,
    pub right: Box<ExprNode>,
}

#[derive(Debug)]
pub struct ArrowFnExpr {
    pub base: NodeBase,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Option<Box<TypeNode>>,
    pub body: ArrowBody,
    pub is_async: bool,
}

#[derive(Debug)]
pub enum ArrowBody {
    Expr(Box<ExprNode>),
    Block(BlockStmt),
}

#[derive(Debug)]
pub struct FunctionExpr {
    pub base: NodeBase,
    pub name: Option<Ident>,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Option<Box<TypeNode>>,
    pub body: Option<BlockStmt>,
    pub is_async: bool,
    pub is_generator: bool,
}

#[derive(Debug)]
pub struct ClassExpr {
    pub base: NodeBase,
    pub name: Option<Ident>,
    pub type_params: Option<TypeParamList>,
    pub extends: Option<ExtendsClause>,
    pub implements: Option<ImplementsClause>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug)]
pub struct YieldExpr {
    pub base: NodeBase,
    pub argument: Option<Box<ExprNode>>,
    pub delegate: bool,
}

#[derive(Debug)]
pub struct AwaitExpr {
    pub base: NodeBase,
    pub argument: Box<ExprNode>,
}

#[derive(Debug)]
pub struct SpreadExpr {
    pub base: NodeBase,
    pub argument: Box<ExprNode>,
}

#[derive(Debug)]
pub struct AsExpr {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
    pub type_node: Box<TypeNode>,
}

#[derive(Debug)]
pub struct SatisfiesExpr {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
    pub type_node: Box<TypeNode>,
}

#[derive(Debug)]
pub struct NonNullExpr {
    pub base: NodeBase,
    pub expression: Box<ExprNode>,
}

#[derive(Debug)]
pub struct TypeAssertionExpr {
    pub base: NodeBase,
    pub type_node: Box<TypeNode>,
    pub expression: Box<ExprNode>,
}

#[derive(Debug)]
pub struct ThisExpr {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct SuperExpr {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct NullExpr {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct TrueExpr {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct FalseExpr {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct VoidExpr {
    pub base: NodeBase,
    pub argument: Box<ExprNode>,
}

#[derive(Debug)]
pub struct TypeofExpr {
    pub base: NodeBase,
    pub argument: Box<ExprNode>,
}

#[derive(Debug)]
pub struct DeleteExpr {
    pub base: NodeBase,
    pub argument: Box<ExprNode>,
}

#[derive(Debug)]
pub enum DeclNode {
    Function(FunctionDecl),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    TypeAlias(TypeAliasDecl),
    Enum(EnumDecl),
    Module(ModuleDecl),
    Import(ImportDecl),
    Export(ExportDecl),
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub base: NodeBase,
    pub name: Option<Ident>,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Option<Box<TypeNode>>,
    pub body: Option<BlockStmt>,
    pub is_async: bool,
    pub is_generator: bool,
    pub is_declare: bool,
}

#[derive(Debug)]
pub struct ClassDecl {
    pub base: NodeBase,
    pub name: Option<Ident>,
    pub type_params: Option<TypeParamList>,
    pub extends: Option<ExtendsClause>,
    pub implements: Option<ImplementsClause>,
    pub members: Vec<ClassMember>,
    pub is_declare: bool,
    pub is_abstract: bool,
}

#[derive(Debug)]
pub struct ExtendsClause {
    pub base: NodeBase,
    pub type_expr: Box<ExprNode>,
    pub type_args: Option<TypeArgList>,
}

#[derive(Debug)]
pub struct ImplementsClause {
    pub base: NodeBase,
    pub types: Vec<TypeNode>,
}

#[derive(Debug)]
pub enum ClassMember {
    Property(ClassProperty),
    Method(ClassMethod),
    Constructor(ClassConstructor),
    Getter(ClassGetter),
    Setter(ClassSetter),
    IndexSig(ClassIndexSig),
    StaticBlock(ClassStaticBlock),
    Empty(NodeBase),
}

#[derive(Debug)]
pub struct ClassProperty {
    pub base: NodeBase,
    pub key: PropKey,
    pub type_annotation: Option<Box<TypeNode>>,
    pub initializer: Option<Box<ExprNode>>,
    pub modifiers: ClassMemberMods,
}

#[derive(Debug)]
pub struct ClassMethod {
    pub base: NodeBase,
    pub key: PropKey,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Option<Box<TypeNode>>,
    pub body: Option<BlockStmt>,
    pub modifiers: ClassMemberMods,
    pub is_async: bool,
    pub is_generator: bool,
}

#[derive(Debug)]
pub struct ClassConstructor {
    pub base: NodeBase,
    pub params: Vec<ParamDecl>,
    pub body: Option<BlockStmt>,
    pub modifiers: ClassMemberMods,
}

#[derive(Debug)]
pub struct ClassGetter {
    pub base: NodeBase,
    pub key: PropKey,
    pub return_type: Option<Box<TypeNode>>,
    pub body: Option<BlockStmt>,
    pub modifiers: ClassMemberMods,
}

#[derive(Debug)]
pub struct ClassSetter {
    pub base: NodeBase,
    pub key: PropKey,
    pub param: ParamDecl,
    pub body: Option<BlockStmt>,
    pub modifiers: ClassMemberMods,
}

#[derive(Debug)]
pub struct ClassIndexSig {
    pub base: NodeBase,
    pub params: Vec<ParamDecl>,
    pub type_annotation: Box<TypeNode>,
    pub modifiers: ClassMemberMods,
}

#[derive(Debug)]
pub struct ClassStaticBlock {
    pub base: NodeBase,
    pub body: BlockStmt,
}

#[derive(Debug, Default)]
pub struct ClassMemberMods {
    pub is_static: bool,
    pub is_readonly: bool,
    pub is_abstract: bool,
    pub is_override: bool,
    pub is_accessor: bool,
    pub visibility: Visibility,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    #[default]
    None,
    Public,
    Protected,
    Private,
}

#[derive(Debug)]
pub struct InterfaceDecl {
    pub base: NodeBase,
    pub name: Ident,
    pub type_params: Option<TypeParamList>,
    pub extends: Option<Vec<TypeNode>>,
    pub members: Vec<TypeElement>,
}

#[derive(Debug)]
pub enum TypeElement {
    PropertySig(PropertySig),
    MethodSig(MethodSig),
    CallSig(CallSig),
    ConstructSig(ConstructSig),
    IndexSig(IndexSig),
}

#[derive(Debug)]
pub struct PropertySig {
    pub base: NodeBase,
    pub key: PropKey,
    pub optional: bool,
    pub readonly: bool,
    pub type_annotation: Option<Box<TypeNode>>,
}

#[derive(Debug)]
pub struct MethodSig {
    pub base: NodeBase,
    pub key: PropKey,
    pub optional: bool,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Option<Box<TypeNode>>,
}

#[derive(Debug)]
pub struct CallSig {
    pub base: NodeBase,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Option<Box<TypeNode>>,
}

#[derive(Debug)]
pub struct ConstructSig {
    pub base: NodeBase,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Option<Box<TypeNode>>,
}

#[derive(Debug)]
pub struct IndexSig {
    pub base: NodeBase,
    pub params: Vec<ParamDecl>,
    pub type_annotation: Box<TypeNode>,
    pub readonly: bool,
}

#[derive(Debug)]
pub struct TypeAliasDecl {
    pub base: NodeBase,
    pub name: Ident,
    pub type_params: Option<TypeParamList>,
    pub type_node: Box<TypeNode>,
}

#[derive(Debug)]
pub struct EnumDecl {
    pub base: NodeBase,
    pub name: Ident,
    pub members: Vec<EnumMember>,
    pub is_const: bool,
}

#[derive(Debug)]
pub struct EnumMember {
    pub base: NodeBase,
    pub name: PropKey,
    pub initializer: Option<Box<ExprNode>>,
}

#[derive(Debug)]
pub struct ModuleDecl {
    pub base: NodeBase,
    pub name: ModuleName,
    pub body: Option<ModuleBody>,
    pub is_declare: bool,
    pub is_global: bool,
}

#[derive(Debug)]
pub enum ModuleName {
    Ident(Ident),
    String(StringLit),
}

#[derive(Debug)]
pub enum ModuleBody {
    Block(ModuleBlock),
    Nested(Box<ModuleDecl>),
}

#[derive(Debug)]
pub struct ModuleBlock {
    pub base: NodeBase,
    pub statements: Vec<Node>,
}

#[derive(Debug)]
pub struct ImportDecl {
    pub base: NodeBase,
    pub import_clause: Option<ImportClause>,
    pub module_specifier: StringLit,
    pub attributes: Option<ImportAttrs>,
}

#[derive(Debug)]
pub struct ImportClause {
    pub base: NodeBase,
    pub default_binding: Option<Ident>,
    pub namespace_import: Option<NamespaceImport>,
    pub named_bindings: Option<NamedImports>,
    pub is_type_only: bool,
}

#[derive(Debug)]
pub struct NamespaceImport {
    pub base: NodeBase,
    pub name: Ident,
}

#[derive(Debug)]
pub struct NamedImports {
    pub base: NodeBase,
    pub elements: Vec<ImportSpecifier>,
}

#[derive(Debug)]
pub struct ImportSpecifier {
    pub base: NodeBase,
    pub property_name: Option<Ident>,
    pub name: Ident,
    pub is_type_only: bool,
}

#[derive(Debug)]
pub struct ImportAttrs {
    pub base: NodeBase,
    pub elements: Vec<ImportAttr>,
}

#[derive(Debug)]
pub struct ImportAttr {
    pub base: NodeBase,
    pub key: Ident,
    pub value: StringLit,
}

#[derive(Debug)]
pub struct ExportDecl {
    pub base: NodeBase,
    pub export_clause: Option<NamedExports>,
    pub module_specifier: Option<StringLit>,
    pub is_type_only: bool,
}

#[derive(Debug)]
pub struct NamedExports {
    pub base: NodeBase,
    pub elements: Vec<ExportSpecifier>,
}

#[derive(Debug)]
pub struct ExportSpecifier {
    pub base: NodeBase,
    pub property_name: Option<Ident>,
    pub name: Ident,
    pub is_type_only: bool,
}

#[derive(Debug)]
pub enum TypeNode {
    Keyword(KeywordType),
    Reference(TypeRef),
    Function(FunctionType),
    Constructor(ConstructorType),
    Array(ArrayType),
    Tuple(TupleType),
    Union(UnionType),
    Intersection(IntersectionType),
    Conditional(ConditionalType),
    Infer(InferType),
    Parenthesized(ParenType),
    Literal(LiteralType),
    TypeLiteral(TypeLiteralType),
    Mapped(MappedType),
    Indexed(IndexedType),
    TypeOperator(TypeOperatorType),
    TypeQuery(TypeQueryType),
    This(ThisType),
    TemplateLiteral(TemplateLiteralType),
    Import(ImportType),
    Predicate(PredicateType),
    Optional(OptionalType),
    Rest(RestType),
    NamedTuple(NamedTupleMember),
}

#[derive(Debug)]
pub struct KeywordType {
    pub base: NodeBase,
    pub keyword: SyntaxKind,
}

#[derive(Debug)]
pub struct TypeRef {
    pub base: NodeBase,
    pub type_name: TypeName,
    pub type_args: Option<TypeArgList>,
}

#[derive(Debug)]
pub enum TypeName {
    Ident(Ident),
    Qualified(QualifiedName),
}

#[derive(Debug)]
pub struct QualifiedName {
    pub base: NodeBase,
    pub left: Box<TypeName>,
    pub right: Ident,
}

#[derive(Debug)]
pub struct TypeArgList {
    pub base: NodeBase,
    pub args: Vec<TypeNode>,
}

#[derive(Debug)]
pub struct FunctionType {
    pub base: NodeBase,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Box<TypeNode>,
}

#[derive(Debug)]
pub struct ConstructorType {
    pub base: NodeBase,
    pub type_params: Option<TypeParamList>,
    pub params: Vec<ParamDecl>,
    pub return_type: Box<TypeNode>,
    pub is_abstract: bool,
}

#[derive(Debug)]
pub struct ArrayType {
    pub base: NodeBase,
    pub element_type: Box<TypeNode>,
}

#[derive(Debug)]
pub struct TupleType {
    pub base: NodeBase,
    pub elements: Vec<TupleElement>,
}

#[derive(Debug)]
pub enum TupleElement {
    Type(TypeNode),
    Named(NamedTupleMember),
}

#[derive(Debug)]
pub struct NamedTupleMember {
    pub base: NodeBase,
    pub name: Ident,
    pub type_node: Box<TypeNode>,
    pub optional: bool,
    pub rest: bool,
}

#[derive(Debug)]
pub struct UnionType {
    pub base: NodeBase,
    pub types: Vec<TypeNode>,
}

#[derive(Debug)]
pub struct IntersectionType {
    pub base: NodeBase,
    pub types: Vec<TypeNode>,
}

#[derive(Debug)]
pub struct ConditionalType {
    pub base: NodeBase,
    pub check_type: Box<TypeNode>,
    pub extends_type: Box<TypeNode>,
    pub true_type: Box<TypeNode>,
    pub false_type: Box<TypeNode>,
}

#[derive(Debug)]
pub struct InferType {
    pub base: NodeBase,
    pub type_param: TypeParam,
}

#[derive(Debug)]
pub struct ParenType {
    pub base: NodeBase,
    pub type_node: Box<TypeNode>,
}

#[derive(Debug)]
pub struct LiteralType {
    pub base: NodeBase,
    pub literal: LiteralTypeValue,
}

#[derive(Debug)]
pub enum LiteralTypeValue {
    String(StringLit),
    Number(NumberLit),
    True,
    False,
    Null,
    BigInt(BigIntLit),
}

#[derive(Debug)]
pub struct TypeLiteralType {
    pub base: NodeBase,
    pub members: Vec<TypeElement>,
}

#[derive(Debug)]
pub struct MappedType {
    pub base: NodeBase,
    pub type_param: TypeParam,
    pub name_type: Option<Box<TypeNode>>,
    pub type_node: Option<Box<TypeNode>>,
    pub readonly_token: Option<PlusMinusToken>,
    pub optional_token: Option<PlusMinusToken>,
}

#[derive(Debug, Clone, Copy)]
pub enum PlusMinusToken {
    Plus,
    Minus,
    None,
}

#[derive(Debug)]
pub struct IndexedType {
    pub base: NodeBase,
    pub object_type: Box<TypeNode>,
    pub index_type: Box<TypeNode>,
}

#[derive(Debug)]
pub struct TypeOperatorType {
    pub base: NodeBase,
    pub operator: SyntaxKind,
    pub type_node: Box<TypeNode>,
}

#[derive(Debug)]
pub struct TypeQueryType {
    pub base: NodeBase,
    pub expr_name: TypeName,
    pub type_args: Option<TypeArgList>,
}

#[derive(Debug)]
pub struct ThisType {
    pub base: NodeBase,
}

#[derive(Debug)]
pub struct TemplateLiteralType {
    pub base: NodeBase,
    pub head: TemplateHead,
    pub spans: Vec<TemplateLiteralTypeSpan>,
}

#[derive(Debug)]
pub struct TemplateLiteralTypeSpan {
    pub base: NodeBase,
    pub type_node: Box<TypeNode>,
    pub literal: TemplatePart,
}

#[derive(Debug)]
pub struct ImportType {
    pub base: NodeBase,
    pub argument: Box<TypeNode>,
    pub qualifier: Option<TypeName>,
    pub type_args: Option<TypeArgList>,
    pub is_typeof: bool,
}

#[derive(Debug)]
pub struct PredicateType {
    pub base: NodeBase,
    pub parameter_name: PredicateParam,
    pub type_node: Option<Box<TypeNode>>,
    pub asserts: bool,
}

#[derive(Debug)]
pub enum PredicateParam {
    Ident(Ident),
    This,
}

#[derive(Debug)]
pub struct OptionalType {
    pub base: NodeBase,
    pub type_node: Box<TypeNode>,
}

#[derive(Debug)]
pub struct RestType {
    pub base: NodeBase,
    pub type_node: Box<TypeNode>,
}

#[derive(Debug)]
pub struct TypeParamList {
    pub base: NodeBase,
    pub params: Vec<TypeParam>,
}

#[derive(Debug)]
pub struct TypeParam {
    pub base: NodeBase,
    pub name: Ident,
    pub constraint: Option<Box<TypeNode>>,
    pub default: Option<Box<TypeNode>>,
    pub variance: TypeParamVariance,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TypeParamVariance {
    #[default]
    None,
    In,
    Out,
    InOut,
}

#[derive(Debug)]
pub struct ParamDecl {
    pub base: NodeBase,
    pub name: PatternNode,
    pub type_annotation: Option<Box<TypeNode>>,
    pub initializer: Option<Box<ExprNode>>,
    pub is_optional: bool,
    pub is_rest: bool,
    pub visibility: Visibility,
    pub is_readonly: bool,
    pub is_override: bool,
}

#[derive(Debug)]
pub enum PatternNode {
    Ident(Ident),
    Object(ObjectPattern),
    Array(ArrayPattern),
}

#[derive(Debug)]
pub struct ObjectPattern {
    pub base: NodeBase,
    pub elements: Vec<BindingProp>,
}

#[derive(Debug)]
pub enum BindingProp {
    Property(BindingProperty),
    Shorthand(Ident),
    Rest(RestBinding),
}

#[derive(Debug)]
pub struct BindingProperty {
    pub base: NodeBase,
    pub key: PropKey,
    pub value: Box<PatternNode>,
    pub initializer: Option<Box<ExprNode>>,
}

#[derive(Debug)]
pub struct RestBinding {
    pub base: NodeBase,
    pub argument: Box<PatternNode>,
}

#[derive(Debug)]
pub struct ArrayPattern {
    pub base: NodeBase,
    pub elements: Vec<Option<ArrayPatternElement>>,
}

#[derive(Debug)]
pub enum ArrayPatternElement {
    Pattern(PatternNode),
    Rest(RestBinding),
}
