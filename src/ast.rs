use anyhow::anyhow;
use derive_more::Constructor;

use crate::scanner::{Literal, Token};

#[allow(dead_code)]
pub enum UnOp {
    Minus,
    Bang,
}

#[allow(dead_code)]
pub enum BinOp {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[allow(dead_code)]
#[derive(Default)]
pub enum LitKind {
    Number(f32),
    String(String),
    Boolean(bool),
    #[default]
    Nil,
}

// impl Default for LitKind {
//     fn default() -> Self {
//         match  {
//             Self::Number(n) => Self::Number(f32::default()),
//             Self::String(s) => Self::String(String::default()),
//             Self::Boolean(b) => Self::Boolean(bool::default()),
//             Self::Nil => Self::Nil,
//         }
//     }
// }

#[allow(dead_code)]
pub enum ExprKind {
    Literal(LitKind),
    Unary(Box<Expr>, UnOp),
    Binary(Box<Expr>, Box<Expr>, BinOp),
    Grouping(Box<Expr>),
}

/* NOTE: This will get more fields for diagnostics
* Note that the key here is that an expr is just one type of node in AST,
* which is why this representation works.
*/
#[derive(Constructor)]
pub struct Expr {
    pub kind: ExprKind,
    pub token: Token,
}

impl TryFrom<Literal> for LitKind {
    type Error = anyhow::Error;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Null => Err(anyhow!("Cannot creat LitKind from Null Literal")),
            Literal::Text(t) => Ok(LitKind::String(t)),
            Literal::Number(n) => Ok(LitKind::Number(n)),
        }
    }
}

pub trait Visitor: Sized {
    fn visit_expr(&mut self, expr: &Expr) -> () {
        walk_expr(self, expr)
    }
}

pub fn walk_expr<V>(v: &mut V, expr: &Expr) -> ()
where
    V: Visitor,
{
    match &expr.kind {
        ExprKind::Binary(left, right, _) => {
            v.visit_expr(left);
            v.visit_expr(right);
        }
        ExprKind::Unary(expr, _) => {
            v.visit_expr(expr);
        }
        ExprKind::Grouping(expr) => {
            v.visit_expr(expr);
        }
        _ => {}
    }
}

// pub struct PrettyPrinter {}

// impl Visitor for PrettyPrinter {
//     type Result = String;
//     fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
//         match expr.kind {
//             ExprKind::Unary(expr, op) => {
//                 let op_str = match op {
//                     UnOp::Minus => "-",
//                     UnOp::Bang => "!",
//                 };
//                 format!("({}{})", op_str, self.visit_expr(expr.as_ref()))
//             }
//             ExprKind::Binary(left, right, op) => {
//                 let op_str = match op {
//                     BinOp::Bang => "!",
//                     BinOp::BangEqual => "!=",
//                     BinOp::Equal => "=",
//                     BinOp::EqualEqual => "==",
//                     BinOp::Greater => ">",
//                     BinOp::GreaterEqual => ">=",
//                     BinOp::Less => "<",
//                     BinOp::LessEqual => "<=",
//                     BinOp::Plus => "+",
//                     BinOp::Minus => "-",
//                     BinOp::Star => "*",
//                     BinOp::Slash => "/",
//                 };
//                 format!(
//                     "( {} {} {} )",
//                     op_str,
//                     self.visit_expr(left.as_ref()),
//                     self.visit_expr(right.as_ref())
//                 )
//             }
//             ExprKind::Grouping(expr) => format!("(gr {})", self.visit_expr(expr.as_ref())),
//             ExprKind::Literal(kind) => match kind {
//                 LitKind::Nil => "nil".to_string(),
//                 LitKind::True => "true".to_string(),
//                 LitKind::False => "false".to_string(),
//                 LitKind::Number(n) => n.to_string(),
//                 LitKind::String(s) => format!("\"{s}\""),
//             },
//         }
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_printer() {
//         let expr = ExprKind::Binary(
//             ExprKind::Unary(ExprKind::Literal(LitKind::Number(123.)).into(), UnOp::Minus).into(),
//             ExprKind::Grouping(ExprKind::Literal(LitKind::String("45.67".into())).into()).into(),
//             BinOp::Star,
//         );
//         let mut printer = PrettyPrinter {};
//         let repr = printer.visit_expr(&expr);
//         assert_eq!(repr, "( * (-123) (gr \"45.67\") )");
//     }
// }
