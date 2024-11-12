use anyhow::{anyhow, Result};

pub mod token;

use token::{Token, TokenKind};

#[derive(Default)]
pub struct Lexer {
    source: String,
    rest: String,
    offset: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string(),
            rest: source.to_string(),
            ..Lexer::default()
        }
    }

    fn advance(&mut self) -> Option<char> {
        let mut chars = self.rest.chars();
        if let Some(c) = chars.next() {
            self.rest = chars.collect();
            return Some(c);
        }

        None
    }

    fn peek(&self) -> Option<char> {
        self.rest.chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.rest.chars().nth(1)
    }

    fn lex(&mut self) -> Option<Result<Token>> {
        loop {
            if self.is_at_end() {
                return None;
            }

            let c = self.advance()?;

            let uno = |t: TokenKind| Some(Ok(Token::new(t, c.to_string(), self.line)));

            let duo = |m: char, one: TokenKind, two: TokenKind| {
                if self.rest.starts_with(m) {
                    return Some(Ok(Token::new(two, format!("{c}{m}"), self.line)));
                }

                uno(one)
            };

            enum Longer {
                Slash,
                String,
                Number,
            }

            let begin = match c {
                '\n' => {
                    self.line += 1;
                    continue;
                }
                c if c.is_whitespace() => continue,
                '(' => return uno(TokenKind::LeftParen),
                ')' => return uno(TokenKind::RightParen),
                '{' => return uno(TokenKind::LeftBrace),
                '}' => return uno(TokenKind::RightBrace),
                ',' => return uno(TokenKind::Comma),
                '.' => return uno(TokenKind::Dot),
                '-' => return uno(TokenKind::Minus),
                '+' => return uno(TokenKind::Plus),
                ';' => return uno(TokenKind::Semicolon),
                '*' => return uno(TokenKind::Star),
                '!' => return duo('=', TokenKind::Bang, TokenKind::BangEqual),
                '=' => return duo('=', TokenKind::Equal, TokenKind::EqualEqual),
                '<' => return duo('=', TokenKind::Less, TokenKind::LessEqual),
                '>' => return duo('=', TokenKind::Greater, TokenKind::GreaterEqual),
                '/' => Longer::Slash,
                '"' => Longer::String,
                '0'..='9' => Longer::Number,
                _ => return Some(Err(anyhow!("{}, Unexpected character: '{}'", self.line, c))),
            };

            match begin {
                Longer::Slash => {
                    if self.rest.starts_with('/') {
                        // skip comment
                        let end = self.rest.find('\n').unwrap_or(self.rest.len());
                        self.rest = self.rest[end..].to_string();
                        continue;
                    } else {
                        return uno(TokenKind::Slash);
                    }
                }
                Longer::String => {
                    match self.rest.find('"') {
                        Some(end) => {
                            // count newlines
                            let nl = self.rest[..end].chars().filter(|c| *c == '\n').count();

                            let literal = self.rest[..end].to_string();
                            let result = Some(Ok(Token::new(
                                TokenKind::String(literal.clone()),
                                format!("\"{literal}\""),
                                self.line,
                            )));
                            self.line += nl;

                            // skip closing quote
                            self.rest = self.rest[end + 1..].to_string();

                            return result;
                        }
                        None => return Some(Err(anyhow!("Unterminated string: {}", self.line))),
                    }
                }
                Longer::Number => return self.number(c),
            }
        }
    }

    fn number(&mut self, c: char) -> Option<Result<Token>> {
        let mut lexeme = c.to_string();
        while is_digit(self.peek()) {
            lexeme.push(self.advance().expect("we peeked"));
        }

        if self.peek() == Some('.') && is_digit(self.peek_next()) {
            // consume the '.'
            lexeme.push(self.advance().expect("we peeked"));

            while is_digit(self.peek()) {
                lexeme.push(self.advance().expect("we peeked"));
            }
        }

        let literal: f64 = lexeme.parse().expect("we just parsed this");

        Some(Ok(Token::new(
            TokenKind::Number(literal),
            lexeme,
            self.line,
        )))
    }

    fn is_at_end(&self) -> bool {
        self.rest.is_empty()
    }
}

fn is_digit(c: Option<char>) -> bool {
    if c.is_none() {
        return false;
    }

    c.unwrap().is_digit(10)
}

impl Iterator for Lexer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}
