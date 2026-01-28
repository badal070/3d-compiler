// compiler/optimize/canonical/expr_normalize.rs

use crate::ir::{Expr, BinaryOp};
use std::collections::HashMap;

/// Normalizes expressions to canonical form
/// Same meaning → same representation
pub struct ExprNormalizer {
    constant_cache: HashMap<String, Expr>,
}

impl ExprNormalizer {
    pub fn new() -> Self {
        Self {
            constant_cache: HashMap::new(),
        }
    }

    /// Normalize an expression to canonical form
    pub fn normalize(&mut self, expr: Expr) -> Expr {
        match expr {
            // Flatten nested operations
            Expr::Binary(op, box left, box right) => {
                self.normalize_binary(op, left, right)
            }
            
            // Normalize constants
            Expr::Literal(val) => self.normalize_constant(val),
            
            // Recursively normalize compound expressions
            Expr::Call(name, args) => {
                let normalized_args = args.into_iter()
                    .map(|arg| self.normalize(arg))
                    .collect();
                Expr::Call(name, normalized_args)
            }
            
            // Pass through atomic expressions
            _ => expr,
        }
    }

    fn normalize_binary(&mut self, op: BinaryOp, left: Expr, right: Expr) -> Expr {
        let left = self.normalize(left);
        let right = self.normalize(right);

        match op {
            // Commutative operations: sort operands
            BinaryOp::Add | BinaryOp::Multiply => {
                let (left, right) = self.sort_commutative(left, right);
                self.flatten_associative(op, left, right)
            }

            // Simplify operations
            BinaryOp::Add => self.simplify_add(left, right),
            BinaryOp::Multiply => self.simplify_multiply(left, right),
            BinaryOp::Subtract => self.simplify_subtract(left, right),
            
            _ => Expr::Binary(op, Box::new(left), Box::new(right)),
        }
    }

    fn sort_commutative(&self, left: Expr, right: Expr) -> (Expr, Expr) {
        // Canonical ordering: constants < variables < expressions
        if self.expression_order(&left) <= self.expression_order(&right) {
            (left, right)
        } else {
            (right, left)
        }
    }

    fn expression_order(&self, expr: &Expr) -> u32 {
        match expr {
            Expr::Literal(_) => 0,
            Expr::Variable(_) => 1,
            Expr::Binary(_, _, _) => 2,
            Expr::Call(_, _) => 3,
            _ => 4,
        }
    }

    fn flatten_associative(&mut self, op: BinaryOp, left: Expr, right: Expr) -> Expr {
        // Flatten: (a + b) + c → a + b + c (as nested binaries in canonical order)
        let mut terms = Vec::new();
        self.collect_terms(&left, &op, &mut terms);
        self.collect_terms(&right, &op, &mut terms);

        // Rebuild as right-associated tree with sorted terms
        terms.sort_by(|a, b| {
            self.expression_order(a).cmp(&self.expression_order(b))
        });

        terms.into_iter().reduce(|acc, term| {
            Expr::Binary(op, Box::new(acc), Box::new(term))
        }).unwrap()
    }

    fn collect_terms(&self, expr: &Expr, op: &BinaryOp, terms: &mut Vec<Expr>) {
        if let Expr::Binary(expr_op, left, right) = expr {
            if expr_op == op {
                self.collect_terms(left, op, terms);
                self.collect_terms(right, op, terms);
                return;
            }
        }
        terms.push(expr.clone());
    }

    fn simplify_add(&mut self, left: Expr, right: Expr) -> Expr {
        match (&left, &right) {
            // x + 0 → x
            (_, Expr::Literal(v)) if *v == 0.0 => left,
            (Expr::Literal(v), _) if *v == 0.0 => right,
            
            // Constant folding
            (Expr::Literal(a), Expr::Literal(b)) => Expr::Literal(a + b),
            
            _ => Expr::Binary(BinaryOp::Add, Box::new(left), Box::new(right)),
        }
    }

    fn simplify_multiply(&mut self, left: Expr, right: Expr) -> Expr {
        match (&left, &right) {
            // x * 0 → 0
            (_, Expr::Literal(v)) | (Expr::Literal(v), _) if *v == 0.0 => {
                Expr::Literal(0.0)
            }
            
            // x * 1 → x
            (_, Expr::Literal(v)) if *v == 1.0 => left,
            (Expr::Literal(v), _) if *v == 1.0 => right,
            
            // Constant folding
            (Expr::Literal(a), Expr::Literal(b)) => Expr::Literal(a * b),
            
            _ => Expr::Binary(BinaryOp::Multiply, Box::new(left), Box::new(right)),
        }
    }

    fn simplify_subtract(&mut self, left: Expr, right: Expr) -> Expr {
        match (&left, &right) {
            // x - 0 → x
            (_, Expr::Literal(v)) if *v == 0.0 => left,
            
            // x - x → 0 (only for variables, not complex expressions)
            _ if self.structurally_equal(&left, &right) => Expr::Literal(0.0),
            
            // Constant folding
            (Expr::Literal(a), Expr::Literal(b)) => Expr::Literal(a - b),
            
            _ => Expr::Binary(BinaryOp::Subtract, Box::new(left), Box::new(right)),
        }
    }

    fn normalize_constant(&mut self, value: f64) -> Expr {
        // Round to canonical precision to avoid floating-point drift
        let canonical = (value * 1e10).round() / 1e10;
        Expr::Literal(canonical)
    }

    pub fn structurally_equal(&self, a: &Expr, b: &Expr) -> bool {
        // Deep structural equality check
        match (a, b) {
            (Expr::Literal(a), Expr::Literal(b)) => (a - b).abs() < 1e-10,
            (Expr::Variable(a), Expr::Variable(b)) => a == b,
            (Expr::Binary(op_a, l_a, r_a), Expr::Binary(op_b, l_b, r_b)) => {
                op_a == op_b &&
                self.structurally_equal(l_a, l_b) &&
                self.structurally_equal(r_a, r_b)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commutative_sort() {
        let mut normalizer = ExprNormalizer::new();
        
        // b + a → a + b
        let expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Variable("b".into())),
            Box::new(Expr::Variable("a".into())),
        );

        let result = normalizer.normalize(expr);
        
        if let Expr::Binary(_, left, right) = result {
            assert!(matches!(*left, Expr::Variable(s) if s == "a"));
            assert!(matches!(*right, Expr::Variable(s) if s == "b"));
        }
    }

    #[test]
    fn test_constant_folding() {
        let mut normalizer = ExprNormalizer::new();
        
        // 2 + 3 → 5
        let expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Literal(2.0)),
            Box::new(Expr::Literal(3.0)),
        );

        let result = normalizer.normalize(expr);
        assert_eq!(result, Expr::Literal(5.0));
    }

    #[test]
    fn test_identity_elimination() {
        let mut normalizer = ExprNormalizer::new();
        
        // x + 0 → x
        let expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Literal(0.0)),
        );

        let result = normalizer.normalize(expr);
        assert_eq!(result, Expr::Variable("x".into()));
    }
}