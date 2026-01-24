// compiler/optimize/mod.rs

pub mod pipeline;
pub mod canonical;
pub mod optimize;
pub mod equivalence;
pub mod cost_model;

pub use pipeline::{OptimizationPipeline, OptimizedOutput};
pub use equivalence::{EquivalenceDetector, EquivalenceSignature};
pub use cost_model::{CostModel, CostProfile};

use crate::ir::Scene;

/// High-level optimization interface
pub fn optimize_scene(scene: Scene) -> OptimizedOutput {
    let mut pipeline = OptimizationPipeline::new();
    pipeline.optimize(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    #[test]
    fn test_expression_normalization() {
        // b + a → a + b
        let expr = Expr::Binary(
            BinaryOp::Add,
            Box::new(Expr::Variable("b".into())),
            Box::new(Expr::Variable("a".into())),
        );

        let mut normalizer = canonical::ExprNormalizer::new();
        let normalized = normalizer.normalize(expr);

        // Verify order
        if let Expr::Binary(BinaryOp::Add, left, right) = normalized {
            assert_eq!(*left, Expr::Variable("a".into()));
            assert_eq!(*right, Expr::Variable("b".into()));
        } else {
            panic!("Expected binary add");
        }
    }

    #[test]
    fn test_algebraic_optimization() {
        // x * 2 → x + x
        let expr = Expr::Binary(
            BinaryOp::Multiply,
            Box::new(Expr::Variable("x".into())),
            Box::new(Expr::Literal(2.0)),
        );

        let optimized = optimize::AlgebraicOptimizer::optimize(expr);

        if let Expr::Binary(BinaryOp::Add, left, right) = optimized {
            assert_eq!(*left, Expr::Variable("x".into()));
            assert_eq!(*right, Expr::Variable("x".into()));
        } else {
            panic!("Expected x + x");
        }
    }

    #[test]
    fn test_equivalence_detection() {
        let anim1 = Animation {
            id: "anim1".into(),
            target_id: "shape1".into(),
            expression: Expr::Variable("t".into()),
            time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
            motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
        };

        let anim2 = Animation {
            id: "anim2".into(),
            target_id: "shape2".into(),
            expression: Expr::Variable("t".into()),
            time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
            motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
        };

        assert!(EquivalenceDetector::equivalent(&anim1, &anim2));
    }

    #[test]
    fn test_cost_estimation() {
        let scene = Scene {
            shapes: vec![],
            animations: vec![
                Animation {
                    id: "test".into(),
                    target_id: "shape".into(),
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
    }
}