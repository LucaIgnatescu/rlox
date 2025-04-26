use std::mem::discriminant;

use crate::{
    ast::{Expr, ExprKind, LitKind, Visitor},
    errors::LoxError,
};

pub struct Interpreter {
    pub result: Result<LitKind, LoxError>,
}

impl Visitor for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Binary(l, r, op) => {
                let left = self.visit_expr(l);
                let right = self.visit_expr(r);
                if discriminant(&left) != discriminant(&right) {}
            }
            ExprKind::Unary(l, op) => {},
            ExprKind::Grouping(ex) => ,
            ExprKind::Literal(lit) => ,
        }
    }
}
