use anyhow::{anyhow, Result};
use derive_more::{Constructor, Display};

#[derive(Display, Debug)]
enum TokenType {
    // Single-character tokens.
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
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
    EOF,
}

#[derive(Debug, Display)]
enum Literal {
    Null,
    Text(String),
}

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
        macro_rules! push_token {
            ($v: path, $c: ident) => {
                self.tokens
                    .push(Token::new($v, $c.to_string(), Literal::Null, self.line))
            };
            ($v: path, $c: literal) => {
                self.tokens
                    .push(Token::new($v, String::from($c), Literal::Null, self.line))
            };
            ($token_type: path, $text: ident, $lexeme: ident) => {
                self.tokens.push(Token::new(
                    $token_type,
                    $text,
                    Literal::Text($lexeme),
                    self.line,
                ))
            };
        }

        type TT = TokenType;
        let mut chrs = self.source.chars().peekable();
        while let Some(c) = chrs.next() {
            match c {
                '(' => push_token!(TT::LeftParen, c),
                ')' => push_token!(TT::RightParen, c),
                '{' => push_token!(TT::LeftBrace, c),
                '}' => push_token!(TT::RightBrace, c),
                ',' => push_token!(TT::Comma, c),
                '.' => push_token!(TT::Dot, c),
                '-' => push_token!(TT::Minus, c),
                '+' => push_token!(TT::Plus, c),
                ';' => push_token!(TT::Semicolon, c),
                '*' => push_token!(TT::Star, c),
                '!' => {
                    if let Some(&c1) = chrs.peek() {
                        if c1 == '=' {
                            push_token!(TT::BangEqual, "!=");
                            chrs.next();
                        } else {
                            push_token!(TT::Bang, c);
                        }
                    }
                }
                '=' => {
                    if let Some(&c1) = chrs.peek() {
                        if c1 == '=' {
                            push_token!(TT::EqualEqual, "==");
                            chrs.next();
                        } else {
                            push_token!(TT::Equal, c);
                        }
                    }
                }
                '<' => {
                    if let Some(&c1) = chrs.peek() {
                        if c1 == '=' {
                            push_token!(TT::LessEqual, "<=");
                            chrs.next();
                        } else {
                            push_token!(TT::Less, c);
                        }
                    }
                }
                '>' => {
                    if let Some(&c1) = chrs.peek() {
                        if c1 == '=' {
                            push_token!(TT::GreaterEqual, ">=");
                            chrs.next();
                        } else {
                            push_token!(TT::Greater, c);
                        }
                    }
                }
                '/' => {
                    if let Some(&c1) = chrs.peek() {
                        if c1 == '/' {
                            let _ = chrs.by_ref().take_while(|&c| c != '\n');
                        } else {
                            push_token!(TT::Slash, '/');
                        }
                    }
                }
                ' ' => continue,
                '\r' => continue,
                '\t' => continue,
                '\n' => self.line += 1,
                '"' => {
                    let lexeme: String = chrs
                        .by_ref()
                        .inspect(|&c| {
                            if c == '\n' {
                                self.line += 1
                            }
                        })
                        .take_while(|&c| c != '"')
                        .collect();

                    if let None = chrs.next() {
                        return Err(anyhow!("Unterminated string."));
                    }

                    let text = format!("\"{}\"", lexeme);

                    chrs.next();
                    push_token!(TT::String, text, lexeme);
                }
                _ => {
                    if c.is_digit(10) {
                        let decimal: String = std::iter::once(c)
                            .chain(chrs.take_while(|&c| c != '.' && c.is_digit(10)))
                            .collect();
                    }
                    return Err(anyhow!("Unexpected character."));
                }
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
