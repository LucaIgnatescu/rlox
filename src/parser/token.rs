use anyhow::{anyhow, Result};
use derive_more::{Constructor, Display};

#[derive(Display, Debug, PartialEq, Eq)]
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

#[derive(Debug, Display, PartialEq, Eq)]
enum Literal {
    Null,
    Text(String),
}

#[derive(Debug, Display, Constructor, PartialEq, Eq)]
#[display("{} {} {:?}", token_type, lexeme, literal)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = vec![];
    let mut line = 0;

    macro_rules! push_token {
        ($v: path, $c: ident) => {
            tokens.push(Token::new($v, $c.to_string(), Literal::Null, line))
        };
        ($v: path, $c: literal) => {
            tokens.push(Token::new($v, String::from($c), Literal::Null, line))
        };
        ($token_type: path, $text: ident, $lexeme: ident) => {
            tokens.push(Token::new($token_type, $text, Literal::Text($lexeme), line))
        };
    }

    type TT = TokenType;
    let mut chrs = source.chars().peekable();

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
            '\n' => line += 1,
            '"' => {
                let lexeme: String = chrs
                    .by_ref()
                    .inspect(|&c| {
                        if c == '\n' {
                            line += 1
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

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let input = " \"abc\"  ";
        let tokens = scan_tokens(input).unwrap();
        let token = Token::new(
            TokenType::String,
            String::from("\"abc\""),
            Literal::Text(String::from("abc")),
            0,
        );
        println!("{:?}", token);
        assert_eq!(tokens.len(), 1);
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
        ];
        let tokens = scan_tokens(input).unwrap();
        assert_eq!(want, tokens);
    }
}
