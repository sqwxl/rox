use crate::lexer::Token;

mod grammar;

pub struct Parser {
    tokens: Vec<Token>,
}
