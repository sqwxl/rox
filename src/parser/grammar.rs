use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

pub struct Expr {
    equality: Equality,
}

pub struct Rhs<O, T> {
    op: O,
    right: Box<T>,
}

pub struct Equality {
    comparison: Comparison,
    rhs: Vec<Rhs<EquaityOp, Equality>>,
}

pub enum EquaityOp {
    Equal,
    NotEqual,
}

pub struct Comparison {
    term: Term,
    rhs: Vec<Rhs<ComparisonOp, Comparison>>,
}

pub enum ComparisonOp {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

pub struct Term {
    factor: Factor,
    rhs: Vec<Rhs<TermOp, Term>>,
}

pub enum TermOp {
    Minus,
    Plus,
}

pub struct Factor {
    unary: Unary,
    rhs: Vec<Rhs<FactorOp, Factor>>,
}

pub enum FactorOp {
    Div,
    Mul,
}

pub enum Unary {
    Unary { op: UnaryOp, right: Box<Unary> },
    Primary(Primary),
}

pub enum UnaryOp {
    Not,
    Minus,
}

pub enum Primary {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Grouping,
}

pub struct Grouping {
    expression: Box<Expr>,
}
