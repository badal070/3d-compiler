// compiler/optimize/optimize/motion.rs

use crate::ir::{Motion, Curve, Expr, TimeDomain, Point3D, Keyframe, Interpolation};
use std::collections::HashMap;

/// Optimizes motion specifications
/// Merges identical curves, precomputes static paths, caches primitives
pub struct MotionOptimizer {
    curve_cache: HashMap<String, Curve>,
}

impl MotionOptimizer {
    pub fn new() -> Self {
        Self {
            curve_cache: HashMap::new(),
        }
    }

    pub fn optimize(&mut self, motions: Vec<Motion>) -> Vec<Motion> {
        motions.into_iter()
            .map(|m| self.optimize_motion(m))
            .collect()
    }

    fn optimize_motion(&mut self, motion: Motion) -> Motion {
        match motion {
            Motion::Parametric { curve, domain } => {
                // Check if curve is static (time-independent)
                if Self::is_static_curve(&curve) {
                    return Self::precompute_static(curve, domain);
                }

                // Cache and reuse identical curves
                let curve = self.deduplicate_curve(curve);

                Motion::Parametric { curve, domain }
            }

            Motion::Keyframed { keyframes, interpolation } => {
                // Optimize keyframe sequences
                let keyframes = Self::optimize_keyframes(keyframes);
                Motion::Keyframed { keyframes, interpolation }
            }

            other => other,
        }
    }

    fn is_static_curve(curve: &Curve) -> bool {
        // Check if curve has no time dependency
        !Self::contains_time_variable(&curve.x) &&
        !Self::contains_time_variable(&curve.y) &&
        !Self::contains_time_variable(&curve.z)
    }

    fn contains_time_variable(expr: &Expr) -> bool {
        match expr {
            Expr::Variable(name) => name == "t",
            Expr::Binary(_, left, right) => {
                Self::contains_time_variable(left) ||
                Self::contains_time_variable(right)
            }
            Expr::Call(_, args) => {
                args.iter().any(Self::contains_time_variable)
            }
            _ => false,
        }
    }

    fn precompute_static(curve: Curve, _domain: TimeDomain) -> Motion {
        // Evaluate curve once and store result (time-independent)
        // In real implementation, evaluate at t=0
        let point = Point3D {
            x: Self::eval_expr(&curve.x, 0.0),
            y: Self::eval_expr(&curve.y, 0.0),
            z: Self::eval_expr(&curve.z, 0.0),
        };
        
        Motion::Static { position: point }
    }

    fn eval_expr(expr: &Expr, _t: f64) -> f64 {
        // Simplified evaluation for static expressions
        match expr {
            Expr::Literal(v) => *v,
            _ => 0.0, // Static expressions should be literals
        }
    }

    fn deduplicate_curve(&mut self, curve: Curve) -> Curve {
        let hash = Self::curve_hash(&curve);
        
        if let Some(cached) = self.curve_cache.get(&hash) {
            return cached.clone();
        }

        self.curve_cache.insert(hash, curve.clone());
        curve
    }

    fn curve_hash(curve: &Curve) -> String {
        // Simple hash based on curve representation
        format!("{:?}", curve)
    }

    fn optimize_keyframes(keyframes: Vec<Keyframe>) -> Vec<Keyframe> {
        if keyframes.is_empty() {
            return keyframes;
        }

        // Remove redundant keyframes (identical consecutive values)
        let mut result = Vec::new();
        result.push(keyframes[0].clone());

        for i in 1..keyframes.len() {
            let prev = &keyframes[i - 1];
            let current = &keyframes[i];
            
            if !Self::keyframes_equal(prev, current) {
                result.push(current.clone());
            }
        }

        result
    }

    fn keyframes_equal(a: &Keyframe, b: &Keyframe) -> bool {
        (a.time - b.time).abs() < 1e-6 &&
        (a.value.x - b.value.x).abs() < 1e-6 &&
        (a.value.y - b.value.y).abs() < 1e-6 &&
        (a.value.z - b.value.z).abs() < 1e-6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_detection() {
        let curve = Curve {
            x: Expr::Literal(1.0),
            y: Expr::Literal(2.0),
            z: Expr::Literal(3.0),
        };

        assert!(MotionOptimizer::is_static_curve(&curve));
    }

    #[test]
    fn test_dynamic_detection() {
        let curve = Curve {
            x: Expr::Variable("t".into()),
            y: Expr::Literal(2.0),
            z: Expr::Literal(3.0),
        };

        assert!(!MotionOptimizer::is_static_curve(&curve));
    }

    #[test]
    fn test_keyframe_optimization() {
        let keyframes = vec![
            Keyframe {
                time: 0.0,
                value: Point3D { x: 1.0, y: 1.0, z: 1.0 },
            },
            Keyframe {
                time: 1.0,
                value: Point3D { x: 1.0, y: 1.0, z: 1.0 },
            },
            Keyframe {
                time: 2.0,
                value: Point3D { x: 2.0, y: 2.0, z: 2.0 },
            },
        ];

        let optimized = MotionOptimizer::optimize_keyframes(keyframes);
        assert_eq!(optimized.len(), 2); // Middle duplicate removed
    }
}