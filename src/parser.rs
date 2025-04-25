use std::iter::Peekable;

use crate::{
    ast::{BinOp, Expr, ExprKind, LitKind, UnOp},
    scanner::{Token, TokenType},
};

use anyhow::{anyhow, Result};

/*
*    expression     → equality ;
*    equality       → comparison ( ( "!=" | "==" ) comparison )* ;
*    comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
*    term           → factor ( ( "-" | "+" ) factor )* ;
*    factor         → unary ( ( "/" | "*" ) unary )* ;
*    unary          → ( "!" | "-" ) unary
*                   | primary ;
*    primary        → NUMBER | STRING | "true" | "false" | "nil"
*                   | "(" expression ")" ;
*/

/*
* NOTE: Error handling:
* When we can't parse, we return an error, which we propagate up (?)
* until we hit the statement handler.
* When we hit this point, we synchronize the parser, i.e. chug
* through tokens until we can start parsing a new statement.
*/

pub fn parse_tokens(tokens: &[Token]) -> Result<Expr> {
    let mut it = tokens.iter().peekable();
    parse_expr(&mut it)
}

// expression → equality ;
fn parse_expr<'a, I>(it: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    parse_equality(it)
}

// equality → comparison ( ( "!=" | "==" ) comparison )* ;
fn parse_equality<'a, I>(it: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_comparison(it)?;
    loop {
        let op = match it.peek().map(|t| &t.token_type) {
            Some(TokenType::EqualEqual) => BinOp::EqualEqual,
            Some(TokenType::LessEqual) => BinOp::LessEqual,
            Some(TokenType::EOF) => break,
            _ => break,
        };
        it.next();
        left = Expr::new(ExprKind::Binary(
            Box::new(left),
            Box::new(parse_comparison(it)?),
            op,
        ));
    }
    Ok(left)
}

// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
fn parse_comparison<'a, I>(it: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_term(it)?;
    loop {
        let op = match it.peek().map(|t| &t.token_type) {
            Some(TokenType::Greater) => BinOp::Greater,
            Some(TokenType::GreaterEqual) => BinOp::GreaterEqual,
            Some(TokenType::Less) => BinOp::Less,
            Some(TokenType::LessEqual) => BinOp::LessEqual,
            Some(TokenType::EOF) => break,
            _ => break,
        };
        it.next();
        left = Expr::new(ExprKind::Binary(
            Box::new(left),
            Box::new(parse_comparison(it)?),
            op,
        ));
    }
    Ok(left)
}

// term → factor ( ( "-" | "+" ) factor )* ;
fn parse_term<'a, I>(it: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_factor(it)?;
    loop {
        let op = match it.peek().map(|t| &t.token_type) {
            Some(TokenType::Minus) => BinOp::Minus,
            Some(TokenType::Plus) => BinOp::Plus,
            Some(TokenType::EOF) => break,
            _ => break,
        };
        it.next();
        left = Expr::new(ExprKind::Binary(
            Box::new(left),
            Box::new(parse_factor(it)?),
            op,
        ));
    }
    Ok(left)
}

// factor → unary ( ( "/" | "*" ) unary )* ;
fn parse_factor<'a, I>(it: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    let mut left = parse_unary(it)?;
    loop {
        let op = match it.peek().map(|t| &t.token_type) {
            Some(TokenType::Slash) => BinOp::Slash,
            Some(TokenType::Star) => BinOp::Star,
            Some(TokenType::EOF) => break,
            _ => break,
        };
        it.next();
        left = Expr::new(ExprKind::Binary(
            Box::new(left),
            Box::new(parse_unary(it)?),
            op,
        ));
    }
    Ok(left)
}

// unary → ( "!" | "-" ) unary | primary ;
fn parse_unary<'a, I>(it: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    Ok(match it.peek().map(|t| &t.token_type) {
        Some(TokenType::Bang) => {
            it.next();
            Expr::new(ExprKind::Unary(Box::new(parse_unary(it)?), UnOp::Bang))
        }
        Some(TokenType::Minus) => {
            it.next();
            Expr::new(ExprKind::Unary(Box::new(parse_unary(it)?), UnOp::Minus))
        }
        _ => parse_primary(it)?,
    })
}

// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
fn parse_primary<'a, I>(it: &mut Peekable<I>) -> Result<Expr>
where
    I: Iterator<Item = &'a Token>,
{
    let t = it.peek().ok_or_else(|| anyhow!("Expected expression."))?;
    let kind = match t.token_type {
        TokenType::True => LitKind::True,
        TokenType::False => LitKind::False,
        TokenType::Nil => LitKind::Nil,
        TokenType::Number => LitKind::try_from(t.literal.clone())?,
        TokenType::String => LitKind::try_from(t.literal.clone())?,
        TokenType::LeftParen => {
            it.next();
            let expr = parse_expr(it)?;
            if let Some(TokenType::RightParen) = it.peek().map(|t| t.token_type) {
                it.next();
                return Ok(Expr::new(ExprKind::Grouping(Box::new(expr))));
            }
            return Err(anyhow!("Expected closing )"));
        }
        _ => return Err(anyhow!("Expected expression.")),
    };
    it.next();
    Ok(Expr::new(ExprKind::Literal(kind)))
}
