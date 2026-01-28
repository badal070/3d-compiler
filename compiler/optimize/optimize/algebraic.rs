// compiler/optimize/optimize/algebraic.rs

use crate::ir::{Expr, BinaryOp};

/// Performs algebraic simplifications and optimizations
/// Constant folding, strength reduction, symbolic simplification
pub struct AlgebraicOptimizer;

impl AlgebraicOptimizer {
    pub fn optimize(expr: Expr) -> Expr {
        let mut current = expr;
        let mut previous;

        // Fixed-point iteration: optimize until no changes
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        loop {
            previous = current.clone();
            current = Self::optimize_once(current);
            
            iterations += 1;
            if Self::structurally_equal(&current, &previous) || iterations >= MAX_ITERATIONS {
                break;
            }
        }

        current
    }

    fn optimize_once(expr: Expr) -> Expr {
        match expr {
            Expr::Binary(op, left, right) => {
                let left = Self::optimize(*left);
                let right = Self::optimize(*right);
                Self::optimize_binary(op, left, right)
            }

            Expr::Call(name, args) => {
                let args = args.into_iter()
                    .map(Self::optimize)
                    .collect();
                Self::optimize_call(name, args)
            }

            _ => expr,
        }
    }

    fn optimize_binary(op: BinaryOp, left: Expr, right: Expr) -> Expr {
        match op {
            BinaryOp::Multiply => Self::optimize_multiply(left, right),
            BinaryOp::Add => Self::optimize_add(left, right),
            BinaryOp::Divide => Self::optimize_divide(left, right),
            BinaryOp::Power => Self::optimize_power(left, right),
            _ => Expr::Binary(op, Box::new(left), Box::new(right)),
        }
    }

    fn optimize_multiply(left: Expr, right: Expr) -> Expr {
        match (&left, &right) {
            // Strength reduction: x * 2 → x + x
            (_, Expr::Literal(v)) if (*v - 2.0).abs() < 1e-10 => {
                Expr::Binary(BinaryOp::Add, Box::new(left.clone()), Box::new(left))
            }
            (Expr::Literal(v), _) if (*v - 2.0).abs() < 1e-10 => {
                Expr::Binary(BinaryOp::Add, Box::new(right.clone()), Box::new(right))
            }

            // x * 0 → 0
            (_, Expr::Literal(v)) | (Expr::Literal(v), _) if v.abs() < 1e-10 => {
                Expr::Literal(0.0)
            }

            // x * 1 → x
            (_, Expr::Literal(v)) if (*v - 1.0).abs() < 1e-10 => left,
            (Expr::Literal(v), _) if (*v - 1.0).abs() < 1e-10 => right,

            // Constant folding
            (Expr::Literal(a), Expr::Literal(b)) => Expr::Literal(a * b),

            _ => Expr::Binary(BinaryOp::Multiply, Box::new(left), Box::new(right)),
        }
    }

    fn optimize_add(left: Expr, right: Expr) -> Expr {
        match (&left, &right) {
            // x + 0 → x
            (_, Expr::Literal(v)) if v.abs() < 1e-10 => left,
            (Expr::Literal(v), _) if v.abs() < 1e-10 => right,

            // Constant folding
            (Expr::Literal(a), Expr::Literal(b)) => Expr::Literal(a + b),

            _ => Expr::Binary(BinaryOp::Add, Box::new(left), Box::new(right)),
        }
    }

    fn optimize_divide(left: Expr, right: Expr) -> Expr {
        match (&left, &right) {
            // x / 1 → x
            (_, Expr::Literal(v)) if (*v - 1.0).abs() < 1e-10 => left,

            // x / x → 1 (for simple expressions)
            _ if Self::structurally_equal(&left, &right) => Expr::Literal(1.0),

            // Constant folding
            (Expr::Literal(a), Expr::Literal(b)) if b.abs() > 1e-10 => {
                Expr::Literal(a / b)
            }

            // Strength reduction: x / 2 → x * 0.5
            (_, Expr::Literal(v)) if v.abs() > 1e-10 => {
                Expr::Binary(
                    BinaryOp::Multiply,
                    Box::new(left),
                    Box::new(Expr::Literal(1.0 / v)),
                )
            }

            _ => Expr::Binary(BinaryOp::Divide, Box::new(left), Box::new(right)),
        }
    }

    fn optimize_power(base: Expr, exp: Expr) -> Expr {
        match (&base, &exp) {
            // x^0 → 1
            (_, Expr::Literal(v)) if v.abs() < 1e-10 => Expr::Literal(1.0),

            // x^1 → x
            (_, Expr::Literal(v)) if (*v - 1.0).abs() < 1e-10 => base,

            // x^2 → x * x (strength reduction)
            (_, Expr::Literal(v)) if (*v - 2.0).abs() < 1e-10 => {
                Expr::Binary(
                    BinaryOp::Multiply,
                    Box::new(base.clone()),
                    Box::new(base),
                )
            }

            // Constant folding
            (Expr::Literal(a), Expr::Literal(b)) => {
                Expr::Literal(a.powf(*b))
            }

            _ => Expr::Binary(BinaryOp::Power, Box::new(base), Box::new(exp)),
        }
    }

    fn optimize_call(name: String, args: Vec<Expr>) -> Expr {
        // Function-specific optimizations
        match name.as_str() {
            "sin" if matches!(args.get(0), Some(Expr::Literal(v)) if v.abs() < 1e-10) => {
                Expr::Literal(0.0)
            }
            
            "cos" if matches!(args.get(0), Some(Expr::Literal(v)) if v.abs() < 1e-10) => {
                Expr::Literal(1.0)
            }

            "sqrt" if matches!(args.get(0), Some(Expr::Literal(v)) if *v >= 0.0) => {
                if let Some(Expr::Literal(v)) = args.get(0) {
                    Expr::Literal(v.sqrt())
                } else {
                    Expr::Call(name, args)
                }
            }

            "abs" if matches!(args.get(0), Some(Expr::Literal(_))) => {
                if let Some(Expr::Literal(v)) = args.get(0) {
                    Expr::Literal(v.abs())
                } else {
                    Expr::Call(name, args)
                }
            }

            _ => Expr::Call(name, args),
        }
    }

    fn structurally_equal(a: &Expr, b: &Expr) -> bool {
        match (a, b) {
            (Expr::Literal(a), Expr::Literal(b)) => (a - b).abs() < 1e-10,
            (Expr::Variable(a), Expr::Variable(b)) => a == b,
            (Expr::Binary(op_a, l_a, r_a), Expr::Binary(op_b, l_b, r_b)) => {
                op_a == op_b &&
                Self::structurally_equal(l_a, l_b) &&
                Self::structurally_equal(r_a, r_b)
            }
            (Expr::Call(n_a, args_a), Expr::Call(n_b, args_b)) => {
                n_a == n_b &&
                args_a.len() == args_b.len() &&
                args_a.iter().zip(args_b.iter()).all(|(a, b)| Self::structurally_equal(a, b))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength_reduction_multiply() {
        // x * 2 → x + x
        let expr = Expr::Binary(
            BinaryOp::Multiply,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Literal(2.0)),
        );

        let result = AlgebraicOptimizer::optimize(expr);
        
        if let Expr::Binary(BinaryOp::Add, left, right) = result {
            assert_eq!(*left, Expr::Variable("x".into()));
            assert_eq!(*right, Expr::Variable("x".into()));
        } else {
            panic!("Expected x + x");
        }
    }

    #[test]
    fn test_power_optimization() {
        // x^2 → x * x
        let expr = Expr::Binary(
            BinaryOp::Power,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Literal(2.0)),
        );

        let result = AlgebraicOptimizer::optimize(expr);
        
        assert!(matches!(result, Expr::Binary(BinaryOp::Multiply, _, _)));
    }

    #[test]
    fn test_constant_folding() {
        // 3 + 5 → 8
        let expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Literal(3.0)),
            Box::new(Expr::Literal(5.0)),
        );

        let result = AlgebraicOptimizer::optimize(expr);
        assert_eq!(result, Expr::Literal(8.0));
    }
}