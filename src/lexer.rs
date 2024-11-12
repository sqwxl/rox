use std::fmt::Display;

use anyhow::anyhow;

#[derive(Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Eof,
}

#[derive(Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: Option<String>,
    literal: Option<String>,
    line: i32,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        lexeme: Option<String>,
        literal: Option<String>,
        line: i32,
    ) -> Self {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TokenKind::Eof => write!(f, "EOF"),
            _ => write!(f, "{:?}", self.kind),
        }
    }
    // add code here
}

pub struct Lexer {
    pub source: String,
    pub rest: String,
    line: i32,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string(),
            rest: source.to_string(),
            line: 0,
        }
    }

    // pub fn lexe_tokens(&mut self) -> Vec<Token> {
    //     let mut tokens = Vec::new();
    //
    //     while !self.is_at_end() {
    //         match self.lexe_token() {
    //             Ok(token) => tokens.push(token),
    //             Err(e) => {
    //                 println!("Lexing error: {}", e);
    //             }
    //         }
    //     }
    //
    //     tokens
    // }

    fn lexe_token(&mut self) -> Option<Result<Token, anyhow::Error>> {
        loop {
            let mut chars = self.rest.chars();
            let c = chars.next();
            let rest = chars.as_str();
            self.rest = rest.to_string();

            enum Started {
                Bang,
                Slash,
            }

            let started = match c {
                None => return Some(Ok(Token::new(TokenKind::Eof, None, None, self.line))),
                Some(c) => {
                    let one = |t: TokenKind| {
                        Some(Ok(Token::new(t, Some(c.to_string()), None, self.line)))
                    };
                    match c {
                        '(' => return one(TokenKind::LeftParen),
                        ')' => return one(TokenKind::RightParen),
                        '{' => return one(TokenKind::LeftBrace),
                        '}' => return one(TokenKind::RightBrace),
                        ',' => return one(TokenKind::Comma),
                        '.' => return one(TokenKind::Dot),
                        '-' => return one(TokenKind::Minus),
                        '+' => return one(TokenKind::Plus),
                        ';' => return one(TokenKind::Semicolon),
                        '*' => return one(TokenKind::Star),
                        '!' => Started::Bang,
                        '/' => Started::Slash,
                        _ => return Some(Err(anyhow!("Unexpected character: {}", c))),
                    }
                }
            };

            match started {
                Started::Bang => todo!(),
                Started::Slash => todo!(),
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.rest.is_empty()
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexe_token()
    }
}
