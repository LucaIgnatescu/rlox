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
pub enum LitKind {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

// NOTE: might need to mvoe to a exprKind + span structure if I decide to store tokens
#[allow(dead_code)]
pub enum Expr {
    Literal(LitKind),
    Unary(Box<Expr>, UnOp),
    Binary(Box<Expr>, Box<Expr>, BinOp),
    Grouping(Box<Expr>),
}

pub trait Visitor: Sized {
    type Result: Default;
    fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
        walk_expr(self, expr)
    }
}

fn walk_expr<V>(v: &mut V, expr: &Expr) -> V::Result
where
    V: Visitor,
{
    match expr {
        Expr::Binary(left, right, _) => {
            v.visit_expr(left);
            v.visit_expr(right);
        }
        Expr::Unary(expr, _) => {
            v.visit_expr(expr);
        }
        Expr::Grouping(expr) => {
            v.visit_expr(expr);
        }
        _ => {}
    }
    V::Result::default()
}

pub struct PrettyPrinter {}

impl Visitor for PrettyPrinter {
    type Result = String;
    fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
        match expr {
            Expr::Unary(expr, op) => {
                let op_str = match op {
                    UnOp::Minus => "-",
                    UnOp::Bang => "!",
                };
                format!("({}{})", op_str, self.visit_expr(expr))
            }
            Expr::Binary(left, right, op) => {
                let op_str = match op {
                    BinOp::Bang => "!",
                    BinOp::BangEqual => "!=",
                    BinOp::Equal => "=",
                    BinOp::EqualEqual => "==",
                    BinOp::Greater => ">",
                    BinOp::GreaterEqual => ">=",
                    BinOp::Less => "<",
                    BinOp::LessEqual => "<=",
                    BinOp::Plus => "+",
                    BinOp::Minus => "-",
                    BinOp::Star => "*",
                    BinOp::Slash => "/",
                };
                format!(
                    "( {} {} {} )",
                    op_str,
                    self.visit_expr(left),
                    self.visit_expr(right)
                )
            }
            Expr::Grouping(expr) => format!("(gr {})", self.visit_expr(expr)),
            Expr::Literal(kind) => match kind {
                LitKind::Nil => "nil".to_string(),
                LitKind::True => "true".to_string(),
                LitKind::False => "false".to_string(),
                LitKind::Number(n) => n.to_string(),
                LitKind::String(s) => format!("\"{s}\""),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_printer() {
        let expr = Expr::Binary(
            Expr::Unary(Expr::Literal(LitKind::Number(123.)).into(), UnOp::Minus).into(),
            Expr::Grouping(Expr::Literal(LitKind::String("45.67".into())).into()).into(),
            BinOp::Star,
        );
        let mut printer = PrettyPrinter {};
        let repr = printer.visit_expr(&expr);
        assert_eq!(repr, "( * (-123) (gr \"45.67\") )");
    }
}
