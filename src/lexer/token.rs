use std::fmt::Display;

#[derive(Clone)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Token { kind, lexeme, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match &self.kind {
            TokenKind::Eof => return write!(f, "EOF null"),
            TokenKind::String(s) => return write!(f, "STRING {} {}", self.lexeme, s),
            TokenKind::Number(n) => return write!(f, "NUMBER {} {}", self.lexeme, n),
            TokenKind::LeftParen => "LEFT_PAREN",
            TokenKind::RightParen => "RIGHT_PAREN",
            TokenKind::LeftBrace => "LEFT_BRACE",
            TokenKind::RightBrace => "RIGHT_BRACE",
            TokenKind::Comma => "COMMA",
            TokenKind::Dot => "DOT",
            TokenKind::Minus => "MINUS",
            TokenKind::Plus => "PLUS",
            TokenKind::Semicolon => "SEMICOLON",
            TokenKind::Slash => "SLASH",
            TokenKind::Star => "STAR",
            TokenKind::Bang => "BANG",
            TokenKind::BangEqual => "BANG_EQUAL",
            TokenKind::Equal => "EQUAL",
            TokenKind::EqualEqual => "EQUAL_EQUAL",
            TokenKind::Greater => "GREATER",
            TokenKind::GreaterEqual => "GREATER_EQUAL",
            TokenKind::Less => "LESS",
            TokenKind::LessEqual => "LESS_EQUAL",
            TokenKind::Identifier => "IDENTIFIER",
            TokenKind::And => "AND",
            TokenKind::Class => "CLASS",
            TokenKind::Else => "ELSE",
            TokenKind::False => "FALSE",
            TokenKind::Fun => "FUN",
            TokenKind::For => "FOR",
            TokenKind::If => "IF",
            TokenKind::Nil => "NIL",
            TokenKind::Or => "OR",
            TokenKind::Print => "PRINT",
            TokenKind::Return => "RETURN",
            TokenKind::Super => "SUPER",
            TokenKind::This => "THIS",
            TokenKind::True => "TRUE",
            TokenKind::Var => "VAR",
            TokenKind::While => "WHILE",
        };

        write!(f, "{} {} null", kind, self.lexeme)
    }
}
