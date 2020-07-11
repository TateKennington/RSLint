use super::expr::*;
use super::declaration::Declaration;
use crate::span::Span;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Stmt {
    Variable(VarStmt),
    Empty(EmptyStmt),
    Block(BlockStmt),
    Expr(ExprStmt),
    If(IfStmt),
    Switch(SwitchStmt),
    Throw(ThrowStmt),
    While(WhileStmt),
    DoWhile(DoWhileStmt),
    Labelled(LabelledStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Return(ReturnStmt),
    Try(TryStmt),
    For(ForStmt),
    ForIn(ForInStmt),
    With(WithStmt),
}

impl Stmt {
    pub fn span(&self) -> Span {
        use Stmt::*;
        match self {
            Variable(data) => data.span,
            Empty(data) => data.span,
            Block(data) => data.span,
            Expr(data) => data.span,
            If(data) => data.span,
            Switch(data) => data.span,
            Throw(data) => data.span,
            While(data) => data.span,
            DoWhile(data) => data.span,
            Labelled(data) => data.span,
            Break(data) => data.span,
            Continue(data) => data.span,
            Return(data) => data.span,
            Try(data) => data.span,
            For(data) => data.span,
            ForIn(data) => data.span,
            With(data) => data.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtListItem {
    Declaration(Declaration),
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Semicolon {
    /// An automatically inserted semicolon
    Implicit,
    /// A semicolon explicitly typed out
    Explicit(LiteralWhitespace),
}

impl Semicolon {
    /// Get the span of an explicit semicolon or None if the semicolon is implicit
    pub fn span(&self) -> Option<Span> {
        if let Semicolon::Explicit(ref data) = *self {
            Some(Span::new(data.before.end, data.after.start))
        } else {
            None
        }
    }

    pub fn offset(&self) -> usize {
        if self == &Semicolon::Implicit { 0 } else { 1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declarator {
    pub span: Span,
    pub name: LiteralExpr,
    pub value: Option<Expr>,
    /// The optional whitespace of the `=`
    pub initializer_whitespace: Option<LiteralWhitespace>,
}

impl Declarator {
    /// Get the end to a declarator, the span of the value if it has one, or the span of the name otherwise
    pub fn span(&self) -> Span {
        self.value
            .as_ref()
            .map(|expr| expr.span())
            .unwrap_or(&self.name.span)
            .to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarStmt {
    pub span: Span,
    /// It can only be an identifier so we can just use literal expr for this
    /// We might want to reconsider this choice later on
    pub declared: Vec<Declarator>,
    pub comma_whitespaces: Vec<LiteralWhitespace>,
    /// The whitespace of the `var` keyword
    pub var_whitespace: LiteralWhitespace,
    pub semi: Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockStmt {
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub open_brace_whitespace: LiteralWhitespace,
    pub close_brace_whitespace: LiteralWhitespace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmptyStmt {
    pub span: Span,
    pub semi_whitespace: LiteralWhitespace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Expr,
    pub semi: Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStmt {
    pub span: Span,
    pub if_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub condition: Expr,
    pub cons: Box<Stmt>,
    pub else_whitespace: Option<LiteralWhitespace>,
    pub alt: Option<Box<Stmt>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Case {
    pub span: Span,
    // Whether the case is a `default:` case, there will only be one in total
    pub default: bool,
    pub whitespace: LiteralWhitespace,
    pub colon_whitespace: LiteralWhitespace,
    pub test: Option<Expr>,
    pub cons: Vec<Stmt>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchStmt {
    pub span: Span,
    pub switch_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub test: Expr,
    pub open_brace_whitespace: LiteralWhitespace,
    pub close_brace_whitespace: LiteralWhitespace,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowStmt {
    pub span: Span,
    pub arg: Expr,
    pub semi: Semicolon,
    pub throw_whitespace: LiteralWhitespace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileStmt {
    pub span: Span,
    pub while_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub condition: Expr,
    pub cons: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoWhileStmt {
    pub span: Span,
    pub do_whitespace: LiteralWhitespace,
    pub while_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub condition: Expr,
    pub cons: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelledStmt {
    pub span: Span,
    pub label: LiteralExpr,
    pub colon_whitespace: LiteralWhitespace,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakStmt {
    pub span: Span,
    pub break_whitespace: LiteralWhitespace,
    pub label: Option<LiteralExpr>,
    pub semi: Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueStmt {
    pub span: Span,
    pub continue_whitespace: LiteralWhitespace,
    pub label: Option<LiteralExpr>,
    pub semi: Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStmt {
    pub span: Span,
    pub return_whitespace: LiteralWhitespace,
    pub value: Option<Expr>,
    pub semi: Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchClause {
    pub span: Span,
    pub catch_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub param: LiteralExpr,
    pub body: BlockStmt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryStmt {
    pub span: Span,
    pub try_whitespace: LiteralWhitespace,
    pub test: BlockStmt,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStmt>,
    pub final_whitespace: Option<LiteralWhitespace>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForStmtInit {
    Expr(Expr),
    Var(VarStmt),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
    pub span: Span,
    pub for_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub init: Option<ForStmtInit>,
    pub test: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Box<Stmt>,
    pub init_semicolon_whitespace: LiteralWhitespace,
    pub test_semicolon_whitespace: LiteralWhitespace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForInStmt {
    pub span: Span,
    pub for_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub left: ForStmtInit,
    pub right: Expr,
    pub in_whitespace: LiteralWhitespace,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithStmt {
    pub span: Span,
    pub with_whitespace: LiteralWhitespace,
    pub open_paren_whitespace: LiteralWhitespace,
    pub close_paren_whitespace: LiteralWhitespace,
    pub object: Expr,
    pub body: Box<Stmt>,
}