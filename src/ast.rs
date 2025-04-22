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

#[allow(dead_code)]
enum Expr {
    Literal(LitKind),
    Unary(Box<Expr>, UnOp),
    Binary(Box<Expr>, Box<Expr>, BinOp),
    Grouping(Box<Expr>),
}

trait Visitor: Sized {
    type Result: Default;
    fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
        walk_expr(self, expr)
    }
}

//walk call visits into subchildren
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
