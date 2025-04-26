use crate::ast::{LitKind, Visitor};

struct Interpreter {}

impl Visitor for Interpreter {
    type Result = LitKind;

    fn visit_expr(&mut self, expr: &crate::ast::Expr) -> Self::Result {
        crate::ast::walk_expr(self, expr)
    }
}
