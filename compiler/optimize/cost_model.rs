// compiler/optimize/cost_model.rs

use crate::ir::{Expr, Animation, Scene, BinaryOp};

/// Cost profile for animations and scenes
#[derive(Debug, Clone)]
pub struct CostProfile {
    pub computation_cost: f64,
    pub memory_cost: f64,
    pub numerical_stability: f64, // 0.0 = unstable, 1.0 = stable
    pub total_cost: f64,
}

/// Estimates computational cost and guides optimization strategies
pub struct CostModel;

impl CostModel {
    /// Estimate cost for entire scene
    pub fn estimate_scene(scene: &Scene) -> CostProfile {
        let mut total_comp = 0.0;
        let mut total_mem = 0.0;
        let mut min_stability = 1.0;

        for anim in &scene.animations {
            let profile = Self::estimate_animation(anim);
            total_comp += profile.computation_cost;
            total_mem += profile.memory_cost;
            min_stability = min_stability.min(profile.numerical_stability);
        }

        CostProfile {
            computation_cost: total_comp,
            memory_cost: total_mem,
            numerical_stability: min_stability,
            total_cost: Self::compute_total_cost(total_comp, total_mem, min_stability),
        }
    }

    /// Estimate cost for single animation
    pub fn estimate_animation(animation: &Animation) -> CostProfile {
        let comp_cost = Self::expression_cost(&animation.expression);
        let mem_cost = Self::memory_cost(animation);
        let stability = Self::numerical_stability(&animation.expression);

        CostProfile {
            computation_cost: comp_cost,
            memory_cost: mem_cost,
            numerical_stability: stability,
            total_cost: Self::compute_total_cost(comp_cost, mem_cost, stability),
        }
    }

    fn compute_total_cost(comp: f64, mem: f64, stability: f64) -> f64 {
        // Weight: computation + memory/10 + instability penalty
        comp + mem * 0.1 + (1.0 - stability) * 100.0
    }

    fn expression_cost(expr: &Expr) -> f64 {
        match expr {
            Expr::Literal(_) | Expr::Variable(_) => 1.0,
            
            Expr::Binary(op, left, right) => {
                let op_cost = match op {
                    BinaryOp::Add | BinaryOp::Subtract => 1.0,
                    BinaryOp::Multiply | BinaryOp::Divide => 2.0,
                    BinaryOp::Power => 10.0,
                    BinaryOp::Modulo => 3.0,
                    _ => 1.0,
                };
                
                op_cost + Self::expression_cost(left) + Self::expression_cost(right)
            }

            Expr::Call(name, args) => {
                let func_cost = match name.as_str() {
                    "sin" | "cos" | "tan" => 20.0,
                    "sqrt" | "exp" | "log" => 15.0,
                    "abs" | "floor" | "ceil" => 5.0,
                    "min" | "max" => 3.0,
                    _ => 10.0,
                };
                
                func_cost + args.iter().map(Self::expression_cost).sum::<f64>()
            }

            _ => 5.0,
        }
    }

    fn memory_cost(animation: &Animation) -> f64 {
        // Estimate memory footprint
        let base_size = 64.0; // bytes for animation struct
        let expr_size = Self::expression_size(&animation.expression);
        
        base_size + expr_size
    }

    fn expression_size(expr: &Expr) -> f64 {
        match expr {
            Expr::Literal(_) => 8.0,
            Expr::Variable(_) => 16.0,
            Expr::Binary(_, left, right) => {
                24.0 + Self::expression_size(left) + Self::expression_size(right)
            }
            Expr::Call(_, args) => {
                32.0 + args.iter().map(Self::expression_size).sum::<f64>()
            }
            _ => 16.0,
        }
    }

    fn numerical_stability(expr: &Expr) -> f64 {
        match expr {
            Expr::Binary(BinaryOp::Divide, _, right) => {
                // Division can be unstable if denominator approaches zero
                let right_stability = Self::numerical_stability(right);
                right_stability * 0.8 // Penalize division
            }

            Expr::Binary(BinaryOp::Power, base, exp) => {
                // Large exponents can cause overflow
                let base_stability = Self::numerical_stability(base);
                let exp_stability = Self::numerical_stability(exp);
                (base_stability * exp_stability).min(0.7)
            }

            Expr::Call(name, args) => {
                let func_stability = match name.as_str() {
                    "sin" | "cos" => 1.0, // Very stable
                    "tan" => 0.8,         // Can have discontinuities
                    "log" => 0.7,         // Undefined for non-positive
                    "sqrt" => 0.8,        // Undefined for negative
                    "exp" => 0.6,         // Can overflow easily
                    _ => 0.9,
                };

                let arg_stability = args.iter()
                    .map(Self::numerical_stability)
                    .fold(1.0, f64::min);

                func_stability * arg_stability
            }

            Expr::Binary(_, left, right) => {
                let left_stability = Self::numerical_stability(left);
                let right_stability = Self::numerical_stability(right);
                left_stability.min(right_stability)
            }

            _ => 1.0, // Literals and variables are perfectly stable
        }
    }

    /// Compare two cost profiles
    pub fn is_cheaper(a: &CostProfile, b: &CostProfile) -> bool {
        a.total_cost < b.total_cost
    }

    /// Estimate optimization benefit
    pub fn optimization_benefit(before: &CostProfile, after: &CostProfile) -> f64 {
        if before.total_cost == 0.0 {
            return 0.0;
        }
        
        (before.total_cost - after.total_cost) / before.total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    #[test]
    fn test_basic_cost_estimation() {
        let expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Literal(5.0)),
        );

        let cost = CostModel::expression_cost(&expr);
        assert!(cost > 0.0);
    }

    #[test]
    fn test_expensive_operations() {
        let power_expr = Expr::Binary(
            BinaryOp::Power,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Literal(3.0)),
        );

        let add_expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Literal(3.0)),
        );

        let power_cost = CostModel::expression_cost(&power_expr);
        let add_cost = CostModel::expression_cost(&add_expr);

        assert!(power_cost > add_cost);
    }

    #[test]
    fn test_stability_analysis() {
        // Division is less stable
        let div_expr = Expr::Binary(
            BinaryOp::Divide,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Variable("y".into())),
        );

        // Addition is stable
        let add_expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Variable("y".into())),
        );

        let div_stability = CostModel::numerical_stability(&div_expr);
        let add_stability = CostModel::numerical_stability(&add_expr);

        assert!(div_stability < add_stability);
    }

    #[test]
    fn test_scene_cost() {
        let scene = Scene {
            shapes: vec![],
            animations: vec![
                Animation {
                    id: "anim1".into(),
                    target_id: "shape1".into(),
                    expression: Expr::Binary(
                        BinaryOp::Power,
                        Box::new(Expr::Variable("t".into())),
                        Box::new(Expr::Literal(3.0)),
                    ),
                    time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
                    motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
                }
            ],
            constraints: vec![],
            groups: vec![],
        };

        let profile = CostModel::estimate_scene(&scene);
        assert!(profile.computation_cost > 0.0);
        assert!(profile.numerical_stability > 0.0 && profile.numerical_stability <= 1.0);
    }
}