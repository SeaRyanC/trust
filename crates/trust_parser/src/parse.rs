//! Parser for TypeScript - transforms tokens into AST

use crate::ast::*;
use trust_common::SyntaxKind;
use trust_scanner::{Lexer, Tok};
use std::sync::Arc;

pub struct Parser<'a> {
    lex: Lexer<'a>,
    cur: Tok,
    next_id: NodeId,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lex = Lexer::create(src);
        let cur = lex.produce_next();
        Self { lex, cur, next_id: 1 }
    }

    fn alloc_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn advance(&mut self) {
        self.cur = self.lex.produce_next();
    }

    fn at(&self, k: SyntaxKind) -> bool {
        self.cur.k == k
    }

    fn eat(&mut self, k: SyntaxKind) -> bool {
        if self.at(k) { self.advance(); true } else { false }
    }

    pub fn parse_source_file(mut self, file_name: String) -> SourceFileNode {
        let start = self.cur.p0;
        let mut stmts = Vec::new();
        
        while !self.at(SyntaxKind::EndOfFile) {
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            } else {
                self.advance();
            }
        }

        let eof_base = NodeBase {
            id: self.alloc_id(),
            kind: SyntaxKind::EndOfFile,
            range: SrcRange::new(self.cur.p0, self.cur.p1),
            flags: NodeFlags::empty(),
        };

        SourceFileNode {
            base: NodeBase {
                id: self.alloc_id(),
                kind: SyntaxKind::SourceFile,
                range: SrcRange::new(start, self.cur.p1),
                flags: NodeFlags::empty(),
            },
            file_name,
            statements: stmts,
            end_of_file_token: eof_base,
        }
    }

    fn parse_stmt(&mut self) -> Option<Node> {
        match self.cur.k {
            SyntaxKind::OpenBrace => self.parse_block().map(|b| Node::Stmt(StmtNode::Block(b))),
            SyntaxKind::VarKeyword | SyntaxKind::LetKeyword | SyntaxKind::ConstKeyword => {
                self.parse_var_stmt().map(|v| Node::Stmt(StmtNode::Variable(v)))
            }
            SyntaxKind::IfKeyword => self.parse_if_stmt().map(|i| Node::Stmt(StmtNode::If(i))),
            SyntaxKind::ReturnKeyword => self.parse_return_stmt().map(|r| Node::Stmt(StmtNode::Return(r))),
            SyntaxKind::Semicolon => {
                let base = self.make_base(SyntaxKind::EmptyStatement);
                self.advance();
                Some(Node::Stmt(StmtNode::Empty(EmptyStmt { base })))
            }
            _ => self.parse_expr_stmt(),
        }
    }

    fn make_base(&mut self, kind: SyntaxKind) -> NodeBase {
        NodeBase {
            id: self.alloc_id(),
            kind,
            range: SrcRange::new(self.cur.p0, self.cur.p1),
            flags: NodeFlags::empty(),
        }
    }

    fn parse_block(&mut self) -> Option<BlockStmt> {
        let start = self.cur.p0;
        if !self.eat(SyntaxKind::OpenBrace) { return None; }
        
        let mut stmts = Vec::new();
        while !self.at(SyntaxKind::CloseBrace) && !self.at(SyntaxKind::EndOfFile) {
            if let Some(s) = self.parse_stmt() { stmts.push(s); }
            else { self.advance(); }
        }
        let end = self.cur.p1;
        self.eat(SyntaxKind::CloseBrace);

        Some(BlockStmt {
            base: NodeBase {
                id: self.alloc_id(),
                kind: SyntaxKind::Block,
                range: SrcRange::new(start, end),
                flags: NodeFlags::empty(),
            },
            statements: stmts,
        })
    }

    fn parse_var_stmt(&mut self) -> Option<VariableStmt> {
        let start = self.cur.p0;
        let decl_kind = match self.cur.k {
            SyntaxKind::VarKeyword => VarDeclKind::Var,
            SyntaxKind::LetKeyword => VarDeclKind::Let,
            SyntaxKind::ConstKeyword => VarDeclKind::Const,
            _ => return None,
        };
        self.advance();

        let mut decls = Vec::new();
        loop {
            if let Some(d) = self.parse_var_decl() { decls.push(d); }
            if !self.eat(SyntaxKind::Comma) { break; }
        }
        let end = self.cur.p1;
        self.eat(SyntaxKind::Semicolon);

        Some(VariableStmt {
            base: NodeBase {
                id: self.alloc_id(),
                kind: SyntaxKind::VariableStatement,
                range: SrcRange::new(start, end),
                flags: NodeFlags::empty(),
            },
            declaration_list: VariableDeclList {
                base: NodeBase {
                    id: self.alloc_id(),
                    kind: SyntaxKind::VariableDeclarationList,
                    range: SrcRange::new(start, end),
                    flags: NodeFlags::empty(),
                },
                declarations: decls,
                decl_kind,
            },
        })
    }

    fn parse_var_decl(&mut self) -> Option<VariableDecl> {
        let start = self.cur.p0;
        let name = self.parse_binding_name()?;
        
        let type_ann = if self.eat(SyntaxKind::Colon) {
            self.parse_type().map(Box::new)
        } else { None };

        let init = if self.eat(SyntaxKind::Equals) {
            self.parse_assignment_expr().map(Box::new)
        } else { None };

        Some(VariableDecl {
            base: NodeBase {
                id: self.alloc_id(),
                kind: SyntaxKind::VariableDeclaration,
                range: SrcRange::new(start, self.cur.p0),
                flags: NodeFlags::empty(),
            },
            name,
            type_annotation: type_ann,
            initializer: init,
        })
    }

    fn parse_binding_name(&mut self) -> Option<PatternNode> {
        if self.at(SyntaxKind::Identifier) {
            let id = self.parse_ident()?;
            Some(PatternNode::Ident(id))
        } else { None }
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        if !self.at(SyntaxKind::Identifier) { return None; }
        let text: Arc<str> = self.cur.txt.as_deref().unwrap_or("").into();
        let base = self.make_base(SyntaxKind::Identifier);
        self.advance();
        Some(Ident { base, text })
    }

    fn parse_if_stmt(&mut self) -> Option<IfStmt> {
        let start = self.cur.p0;
        self.advance(); // if
        self.eat(SyntaxKind::OpenParen);
        let cond = Box::new(self.parse_expr()?);
        self.eat(SyntaxKind::CloseParen);
        let then_stmt = Box::new(self.parse_stmt()?);
        let else_stmt = if self.eat(SyntaxKind::ElseKeyword) {
            self.parse_stmt().map(Box::new)
        } else { None };

        Some(IfStmt {
            base: NodeBase {
                id: self.alloc_id(),
                kind: SyntaxKind::IfStatement,
                range: SrcRange::new(start, self.cur.p0),
                flags: NodeFlags::empty(),
            },
            condition: cond,
            then_stmt,
            else_stmt,
        })
    }

    fn parse_return_stmt(&mut self) -> Option<ReturnStmt> {
        let start = self.cur.p0;
        self.advance();
        let expr = if !self.at(SyntaxKind::Semicolon) && !self.cur.nl {
            self.parse_expr().map(Box::new)
        } else { None };
        self.eat(SyntaxKind::Semicolon);

        Some(ReturnStmt {
            base: NodeBase {
                id: self.alloc_id(),
                kind: SyntaxKind::ReturnStatement,
                range: SrcRange::new(start, self.cur.p0),
                flags: NodeFlags::empty(),
            },
            expression: expr,
        })
    }

    fn parse_expr_stmt(&mut self) -> Option<Node> {
        let start = self.cur.p0;
        let expr = self.parse_expr()?;
        self.eat(SyntaxKind::Semicolon);
        Some(Node::Stmt(StmtNode::Expression(ExpressionStmt {
            base: NodeBase {
                id: self.alloc_id(),
                kind: SyntaxKind::ExpressionStatement,
                range: SrcRange::new(start, self.cur.p0),
                flags: NodeFlags::empty(),
            },
            expression: Box::new(expr),
        })))
    }

    fn parse_expr(&mut self) -> Option<ExprNode> {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Option<ExprNode> {
        self.parse_conditional_expr()
    }

    fn parse_conditional_expr(&mut self) -> Option<ExprNode> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Option<ExprNode> {
        let mut left = self.parse_unary_expr()?;
        
        loop {
            let prec = self.binary_op_precedence();
            if prec <= min_prec { break; }
            
            let op = self.cur.k;
            let start = self.cur.p0;
            self.advance();
            
            let right = self.parse_binary_expr(prec)?;
            left = ExprNode::Binary(BinaryExpr {
                base: NodeBase {
                    id: self.alloc_id(),
                    kind: SyntaxKind::BinaryExpression,
                    range: SrcRange::new(start, self.cur.p0),
                    flags: NodeFlags::empty(),
                },
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            });
        }
        Some(left)
    }

    fn binary_op_precedence(&self) -> u8 {
        match self.cur.k {
            SyntaxKind::BarBar => 1,
            SyntaxKind::AmpersandAmpersand => 2,
            SyntaxKind::Bar => 3,
            SyntaxKind::Caret => 4,
            SyntaxKind::Ampersand => 5,
            SyntaxKind::EqualsEquals | SyntaxKind::ExclamationEquals |
            SyntaxKind::EqualsEqualsEquals | SyntaxKind::ExclamationEqualsEquals => 6,
            SyntaxKind::LessThan | SyntaxKind::GreaterThan |
            SyntaxKind::LessThanEquals | SyntaxKind::GreaterThanEquals => 7,
            SyntaxKind::Plus | SyntaxKind::Minus => 9,
            SyntaxKind::Asterisk | SyntaxKind::Slash | SyntaxKind::Percent => 10,
            _ => 0,
        }
    }

    fn parse_unary_expr(&mut self) -> Option<ExprNode> {
        self.parse_postfix_expr()
    }

    fn parse_postfix_expr(&mut self) -> Option<ExprNode> {
        let mut expr = self.parse_primary_expr()?;
        
        loop {
            match self.cur.k {
                SyntaxKind::Dot => {
                    self.advance();
                    let prop = self.parse_ident()?;
                    expr = ExprNode::Member(MemberExpr {
                        base: NodeBase {
                            id: self.alloc_id(),
                            kind: SyntaxKind::PropertyAccessExpression,
                            range: SrcRange::new(0, self.cur.p0),
                            flags: NodeFlags::empty(),
                        },
                        object: Box::new(expr),
                        property: prop,
                        optional: false,
                    });
                }
                SyntaxKind::OpenParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.at(SyntaxKind::CloseParen) && !self.at(SyntaxKind::EndOfFile) {
                        if let Some(a) = self.parse_assignment_expr() { args.push(a); }
                        if !self.eat(SyntaxKind::Comma) { break; }
                    }
                    self.eat(SyntaxKind::CloseParen);
                    expr = ExprNode::Call(CallExpr {
                        base: NodeBase {
                            id: self.alloc_id(),
                            kind: SyntaxKind::CallExpression,
                            range: SrcRange::new(0, self.cur.p0),
                            flags: NodeFlags::empty(),
                        },
                        callee: Box::new(expr),
                        type_arguments: None,
                        arguments: args,
                    });
                }
                _ => break,
            }
        }
        Some(expr)
    }

    fn parse_primary_expr(&mut self) -> Option<ExprNode> {
        match self.cur.k {
            SyntaxKind::Identifier => {
                let id = self.parse_ident()?;
                Some(ExprNode::Ident(id))
            }
            SyntaxKind::NumericLiteral => {
                let val = self.cur.num.unwrap_or(0.0);
                let base = self.make_base(SyntaxKind::NumericLiteral);
                self.advance();
                Some(ExprNode::Literal(LiteralExpr::Number(NumberLit { base, value: val })))
            }
            SyntaxKind::StringLiteral => {
                let val: Arc<str> = self.cur.txt.as_deref().unwrap_or("").into();
                let base = self.make_base(SyntaxKind::StringLiteral);
                self.advance();
                Some(ExprNode::Literal(LiteralExpr::String(StringLit { base, value: val })))
            }
            SyntaxKind::TrueKeyword => {
                let base = self.make_base(SyntaxKind::TrueKeyword);
                self.advance();
                Some(ExprNode::True(TrueExpr { base }))
            }
            SyntaxKind::FalseKeyword => {
                let base = self.make_base(SyntaxKind::FalseKeyword);
                self.advance();
                Some(ExprNode::False(FalseExpr { base }))
            }
            SyntaxKind::NullKeyword => {
                let base = self.make_base(SyntaxKind::NullKeyword);
                self.advance();
                Some(ExprNode::Null(NullExpr { base }))
            }
            SyntaxKind::ThisKeyword => {
                let base = self.make_base(SyntaxKind::ThisKeyword);
                self.advance();
                Some(ExprNode::This(ThisExpr { base }))
            }
            SyntaxKind::OpenParen => {
                let start = self.cur.p0;
                self.advance();
                let inner = self.parse_expr()?;
                self.eat(SyntaxKind::CloseParen);
                Some(ExprNode::Paren(ParenExpr {
                    base: NodeBase {
                        id: self.alloc_id(),
                        kind: SyntaxKind::ParenthesizedExpression,
                        range: SrcRange::new(start, self.cur.p0),
                        flags: NodeFlags::empty(),
                    },
                    expression: Box::new(inner),
                }))
            }
            _ => None,
        }
    }

    fn parse_type(&mut self) -> Option<TypeNode> {
        match self.cur.k {
            SyntaxKind::StringKeyword | SyntaxKind::NumberKeyword | SyntaxKind::BooleanKeyword |
            SyntaxKind::AnyKeyword | SyntaxKind::VoidKeyword | SyntaxKind::NeverKeyword |
            SyntaxKind::UnknownKeyword | SyntaxKind::UndefinedKeyword | SyntaxKind::NullKeyword => {
                let base = self.make_base(self.cur.k);
                let kw = self.cur.k;
                self.advance();
                Some(TypeNode::Keyword(KeywordType { base, keyword: kw }))
            }
            SyntaxKind::Identifier => {
                let id = self.parse_ident()?;
                Some(TypeNode::Reference(TypeRef {
                    base: NodeBase {
                        id: self.alloc_id(),
                        kind: SyntaxKind::TypeReference,
                        range: id.base.range.clone(),
                        flags: NodeFlags::empty(),
                    },
                    type_name: TypeName::Ident(id),
                    type_args: None,
                }))
            }
            _ => None,
        }
    }
}
