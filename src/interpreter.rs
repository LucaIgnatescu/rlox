use crate::ast::{Expr, LitKind, Visitor};

struct Interpreter {}

impl Visitor for Interpreter {
    type Result = LitKind;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Result {
        crate::ast::walk_expr(self, expr)
    }
}
