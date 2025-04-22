use anyhow::{anyhow, Result};
use derive_more::{Constructor, Display};
use itertools::Itertools;

#[derive(Display, Debug, PartialEq, Eq)]
#[allow(dead_code)]
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

impl TokenType {
    fn from_keyword(identifier: &str) -> Self {
        match identifier {
            "and" => Self::And,
            "class" => Self::Class,
            "else" => Self::Else,
            "false" => Self::False,
            "for" => Self::For,
            "fun" => Self::Fun,
            "if" => Self::If,
            "nil" => Self::Nil,
            "or" => Self::Or,
            "print" => Self::Print,
            "return" => Self::Return,
            "super" => Self::Super,
            "this" => Self::This,
            "true" => Self::True,
            "var" => Self::Var,
            "while" => Self::While,
            _ => Self::Identifier,
        }
    }
}

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
enum Literal {
    Null,
    Text(String),
    Number(f32), // NOTE: it would prob be good to have multiple number types
}

#[derive(Debug, Display, Constructor, PartialEq)]
#[display("{} {} {:?}", token_type, lexeme, literal)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

impl Token {
    fn new_simple(token_type: TokenType, text: impl ToString, line: u32) -> Self {
        Self::new(token_type, text.to_string(), Literal::Null, line)
    }

    fn new_number(text: &str, line: u32) -> Result<Self> {
        let number: f32 = text.parse().map_err(|_| anyhow!("Invalid number."))?;
        Ok(Self::new(
            TokenType::Number,
            text.to_string(),
            Literal::Number(number),
            line,
        ))
    }
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = vec![];
    let mut line = 0;

    type TT = TokenType;
    let mut chrs = source.chars().peekable();

    while let Some(c) = chrs.next() {
        match c {
            '(' => tokens.push(Token::new_simple(TT::LeftParen, c, line)),
            ')' => tokens.push(Token::new_simple(TT::RightParen, c, line)),
            '{' => tokens.push(Token::new_simple(TT::LeftBrace, c, line)),
            '}' => tokens.push(Token::new_simple(TT::RightBrace, c, line)),
            ',' => tokens.push(Token::new_simple(TT::Comma, c, line)),
            '.' => tokens.push(Token::new_simple(TT::Dot, c, line)),
            '-' => tokens.push(Token::new_simple(TT::Minus, c, line)),
            '+' => tokens.push(Token::new_simple(TT::Plus, c, line)),
            ';' => tokens.push(Token::new_simple(TT::Semicolon, c, line)),
            '*' => tokens.push(Token::new_simple(TT::Star, c, line)),
            '!' => {
                if let Some(&c1) = chrs.peek() {
                    if c1 == '=' {
                        tokens.push(Token::new_simple(TT::BangEqual, "!=", line));
                        chrs.next();
                    } else {
                        tokens.push(Token::new_simple(TT::Bang, "!", line));
                    }
                }
            }
            '=' => {
                if let Some(&c1) = chrs.peek() {
                    if c1 == '=' {
                        tokens.push(Token::new_simple(TT::EqualEqual, "==", line));
                        chrs.next();
                    } else {
                        tokens.push(Token::new_simple(TT::Equal, c, line));
                    }
                }
            }
            '<' => {
                if let Some(&c1) = chrs.peek() {
                    if c1 == '=' {
                        tokens.push(Token::new_simple(TT::LessEqual, "<=", line));
                        chrs.next();
                    } else {
                        tokens.push(Token::new_simple(TT::Less, c, line));
                    }
                }
            }
            '>' => {
                if let Some(&c1) = chrs.peek() {
                    if c1 == '=' {
                        tokens.push(Token::new_simple(TT::GreaterEqual, ">=", line));
                        chrs.next();
                    } else {
                        tokens.push(Token::new_simple(TT::Greater, c, line));
                    }
                }
            }
            '/' => {
                if let Some(&c1) = chrs.peek() {
                    if c1 == '/' {
                        let _ = chrs.by_ref().take_while(|&c| c != '\n');
                    } else {
                        tokens.push(Token::new_simple(TT::Slash, '/', line));
                    }
                }
            }
            ' ' => continue,
            '\r' => continue,
            '\t' => continue,
            '\n' => line += 1,
            '"' => {
                let literal: String = chrs
                    .by_ref()
                    .peeking_take_while(|&c| c != '"')
                    .inspect(|&c| {
                        if c == '\n' {
                            line += 1
                        }
                    })
                    .collect();

                if let None = chrs.next() {
                    return Err(anyhow!("Unterminated string."));
                }

                let lexeme = format!("\"{}\"", literal);

                tokens.push(Token::new(TT::String, lexeme, Literal::Text(literal), line));
            }
            _ => {
                if c.is_digit(10) {
                    let decimal: String = std::iter::once(c)
                        .chain(
                            chrs.by_ref()
                                .peeking_take_while(|&c| c != '.' && c.is_digit(10)),
                        )
                        .collect();
                    match chrs.peek() {
                        None => {
                            tokens.push(Token::new_number(&decimal, line)?);
                            continue;
                        }
                        Some(&c) => {
                            if c != '.' {
                                tokens.push(Token::new_number(&decimal, line)?);
                                continue;
                            }
                            chrs.next();
                            let fractional: String = chrs
                                .by_ref()
                                .peeking_take_while(|&c| c.is_digit(10))
                                .collect();
                            if fractional.len() == 0 {
                                return Err(anyhow!(
                                    "Invalid number: {}. is not a valid number",
                                    decimal
                                ));
                            }
                            let text = format!("{}.{}", decimal, fractional);
                            tokens.push(Token::new_number(&text, line)?);
                        }
                    }
                } else if c.is_alphabetic() || c == '_' {
                    let keyword: String = std::iter::once(c)
                        .chain(
                            chrs.by_ref()
                                .peeking_take_while(|&c| c.is_alphanumeric() || c == '_'),
                        )
                        .collect();
                    let token_type = TokenType::from_keyword(&keyword);
                    tokens.push(Token::new_simple(token_type, keyword, line));
                } else {
                    return Err(anyhow!("Unexpected character."));
                }
            }
        }
    }

    tokens.push(Token::new(
        TokenType::EOF,
        "".to_string(),
        Literal::Null,
        line,
    ));

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let input = " \"abc\"";
        let tokens = scan_tokens(input).unwrap();
        let token = Token::new(
            TokenType::String,
            String::from("\"abc\""),
            Literal::Text(String::from("abc")),
            0,
        );
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], token);
    }

    #[test]
    fn test_misc_tokens() {
        let input = "! != = == () \n <=<.";
        let want: Vec<Token> = vec![
            Token::new(TokenType::Bang, String::from("!"), Literal::Null, 0),
            Token::new(TokenType::BangEqual, String::from("!="), Literal::Null, 0),
            Token::new(TokenType::Equal, String::from("="), Literal::Null, 0),
            Token::new(TokenType::EqualEqual, String::from("=="), Literal::Null, 0),
            Token::new(TokenType::LeftParen, String::from("("), Literal::Null, 0),
            Token::new(TokenType::RightParen, String::from(")"), Literal::Null, 0),
            Token::new(TokenType::LessEqual, String::from("<="), Literal::Null, 1),
            Token::new(TokenType::Less, String::from("<"), Literal::Null, 1),
            Token::new(TokenType::Dot, String::from("."), Literal::Null, 1),
            Token::new(TokenType::EOF, "".to_string(), Literal::Null, 1),
        ];
        let tokens = scan_tokens(input).unwrap();
        assert_eq!(want, tokens);
    }

    #[test]
    fn test_number() {
        let input = "123 123.23";
        let want: Vec<Token> = vec![
            Token::new(
                TokenType::Number,
                "123".to_string(),
                Literal::Number(123.),
                0,
            ),
            Token::new(
                TokenType::Number,
                "123.23".to_string(),
                Literal::Number(123.23),
                0,
            ),
            Token::new(TokenType::EOF, "".to_string(), Literal::Null, 0),
        ];
        let tokens = scan_tokens(input).unwrap();
        assert_eq!(want, tokens);
    }

    #[test]
    fn test_identifier() {
        let input = "while if true xy_zt\n__x1";
        let want: Vec<Token> = vec![
            Token::new(TokenType::While, "while".to_string(), Literal::Null, 0),
            Token::new(TokenType::If, "if".to_string(), Literal::Null, 0),
            Token::new(TokenType::True, "true".to_string(), Literal::Null, 0),
            Token::new(TokenType::Identifier, "xy_zt".to_string(), Literal::Null, 0),
            Token::new(TokenType::Identifier, "__x1".to_string(), Literal::Null, 1),
            Token::new(TokenType::EOF, "".to_string(), Literal::Null, 1),
        ];
        let tokens = scan_tokens(input).unwrap();
        assert_eq!(want, tokens);
    }
}
