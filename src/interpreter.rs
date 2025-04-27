use crate::{
    ast::{BinaryEval, Expr, ExprKind, LitKind, UnaryEval, Visitor},
    errors::LoxError,
};

pub struct Interpreter {
    pub result: Result<LitKind, LoxError>,
}

impl Visitor for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) {
        self.result = visit_helper(self, expr);
    }
}

fn visit_helper(intr: &mut Interpreter, expr: &Expr) -> Result<LitKind, LoxError> {
    match &expr.kind {
        ExprKind::Binary(l, r, op) => {
            let left = visit_helper(intr, l)?;
            let right = visit_helper(intr, r)?;
            let err = LoxError::new_parse(&expr.token, "incompatible types");
            Ok(match (left, right) {
                (LitKind::Number(a), LitKind::Number(b)) => {
                    LitKind::Number(op.bin_eval(a, b).ok_or(err)?)
                }
                (LitKind::String(a), LitKind::String(b)) => {
                    LitKind::String(op.bin_eval(a, b).ok_or(err)?)
                }
                (LitKind::Nil, LitKind::Nil) => LitKind::Nil,
                _ => return Err(err),
            })
        }
        ExprKind::Grouping(ex) => visit_helper(intr, ex),
        ExprKind::Unary(ex, op) => {
            let err = LoxError::new_parse(&expr.token, "invalid operation");
            Ok(match visit_helper(intr, ex)? {
                LitKind::Boolean(b) => LitKind::Boolean(op.unary_eval(b).ok_or(err)?),
                LitKind::Number(n) => LitKind::Number(op.unary_eval(n).ok_or(err)?),
                _ => return Err(err),
            })
        }
        ExprKind::Literal(lit) => Ok(lit.clone()),
    }
}
