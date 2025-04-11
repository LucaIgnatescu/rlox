use anyhow::{anyhow, Result};
use derive_more::{Constructor, Display};

#[derive(Display, Debug)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

type Literal = ();

#[derive(Debug, Display, Constructor)]
#[display("{} {} {:?}", token_type, lexeme, literal)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    // source line number
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            line: 1,
        }
    }

    pub fn scanTokens(&mut self) -> Result<()> {
        macro_rules! push {
            ($v: path, $c: ident) => {
                self.tokens
                    .push(Token::new($v, $c.to_string(), (), self.line))
            };
        }

        type TT = TokenType;
        let mut chrs = self.source.chars();
        while let Some(c) = chrs.next() {
            match c {
                '(' => push!(TT::LEFT_PAREN, c),
                ')' => push!(TT::RIGHT_PAREN, c),
                '{' => push!(TT::LEFT_BRACE, c),
                '}' => push!(TT::RIGHT_BRACE, c),
                ',' => push!(TT::COMMA, c),
                '.' => push!(TT::DOT, c),
                '-' => push!(TT::MINUS, c),
                '+' => push!(TT::PLUS, c),
                ';' => push!(TT::SEMICOLON, c),
                '*' => push!(TT::STAR, c),
                _ => return Err(anyhow!("Unexpected character.")),
            }
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_display() {}
// }
