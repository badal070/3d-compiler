// compiler/optimize/equivalence.rs

use crate::ir::{Scene, Animation, Expr};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Detects mathematically equivalent animations
/// Used for knowledge reuse and avoiding duplicate storage
pub struct EquivalenceDetector;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EquivalenceSignature {
    pub structural_hash: u64,
    pub semantic_hash: u64,
}

impl EquivalenceDetector {
    /// Generate equivalence signature for animation
    pub fn signature(animation: &Animation) -> EquivalenceSignature {
        let structural = Self::structural_hash(animation);
        let semantic = Self::semantic_hash(animation);

        EquivalenceSignature {
            structural_hash: structural,
            semantic_hash: semantic,
        }
    }

    /// Check if two animations are equivalent
    pub fn equivalent(a: &Animation, b: &Animation) -> bool {
        Self::signature(a) == Self::signature(b)
    }

    fn structural_hash(animation: &Animation) -> u64 {
        // Hash based on exact AST structure
        let mut hasher = DefaultHasher::new();
        format!("{:?}", animation).hash(&mut hasher);
        hasher.finish()
    }

    fn semantic_hash(animation: &Animation) -> u64 {
        // Hash based on mathematical meaning (after canonicalization)
        // This accounts for equivalent but structurally different expressions
        
        let canonical = Self::canonicalize_for_hashing(animation);
        
        let mut hasher = DefaultHasher::new();
        canonical.hash(&mut hasher);
        hasher.finish()
    }

    fn canonicalize_for_hashing(animation: &Animation) -> String {
        // Simplified: serialize in canonical form
        // Real implementation: full canonicalization pipeline
        format!("{:?}", animation)
    }

    /// Find similar animations in knowledge base
    pub fn find_similar(
        target: &Animation,
        knowledge_base: &[Animation],
        threshold: f64,
    ) -> Vec<(usize, f64)> {
        let target_sig = Self::signature(target);
        
        knowledge_base.iter()
            .enumerate()
            .filter_map(|(idx, anim)| {
                let similarity = Self::similarity(&target_sig, &Self::signature(anim));
                if similarity >= threshold {
                    Some((idx, similarity))
                } else {
                    None
                }
            })
            .collect()
    }

    fn similarity(a: &EquivalenceSignature, b: &EquivalenceSignature) -> f64 {
        // Compute similarity score
        if a == b {
            return 1.0;
        }

        // Hamming distance on hash bits
        let structural_diff = (a.structural_hash ^ b.structural_hash).count_ones();
        let semantic_diff = (a.semantic_hash ^ b.semantic_hash).count_ones();
        
        let max_bits = 64;
        let structural_sim = 1.0 - (structural_diff as f64 / max_bits as f64);
        let semantic_sim = 1.0 - (semantic_diff as f64 / max_bits as f64);
        
        // Weight semantic similarity higher
        0.3 * structural_sim + 0.7 * semantic_sim
    }

    /// Generate signature for entire scene
    pub fn scene_signature(scene: &Scene) -> EquivalenceSignature {
        let mut hasher = DefaultHasher::new();
        
        // Hash all animations
        for anim in &scene.animations {
            Self::signature(anim).hash(&mut hasher);
        }
        
        // Hash constraints
        format!("{:?}", scene.constraints).hash(&mut hasher);
        
        let hash = hasher.finish();
        
        EquivalenceSignature {
            structural_hash: hash,
            semantic_hash: hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    #[test]
    fn test_identical_animations() {
        let anim1 = Animation {
            id: "anim1".into(),
            target_id: "shape1".into(),
            expression: Expr::Variable("t".into()),
            time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
            motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
        };

        let anim2 = Animation {
            id: "anim2".into(),
            target_id: "shape1".into(),
            expression: Expr::Variable("t".into()),
            time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
            motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
        };

        assert!(EquivalenceDetector::equivalent(&anim1, &anim2));
    }

    #[test]
    fn test_different_animations() {
        let anim1 = Animation {
            id: "anim1".into(),
            target_id: "shape1".into(),
            expression: Expr::Variable("t".into()),
            time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
            motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
        };

        let anim2 = Animation {
            id: "anim2".into(),
            target_id: "shape1".into(),
            expression: Expr::Literal(5.0),
            time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
            motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
        };

        assert!(!EquivalenceDetector::equivalent(&anim1, &anim2));
    }

    #[test]
    fn test_similarity_scoring() {
        let sig1 = EquivalenceSignature {
            structural_hash: 0b1010101010101010,
            semantic_hash: 0b1010101010101010,
        };

        let sig2 = EquivalenceSignature {
            structural_hash: 0b1010101010101010,
            semantic_hash: 0b1010101010101010,
        };

        let similarity = EquivalenceDetector::similarity(&sig1, &sig2);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_find_similar() {
        let target = Animation {
            id: "target".into(),
            target_id: "shape1".into(),
            expression: Expr::Variable("t".into()),
            time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
            motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
        };

        let knowledge_base = vec![
            Animation {
                id: "kb1".into(),
                target_id: "shape1".into(),
                expression: Expr::Variable("t".into()),
                time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
                motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
            },
            Animation {
                id: "kb2".into(),
                target_id: "shape1".into(),
                expression: Expr::Literal(10.0),
                time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
                motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
            },
        ];

        let similar = EquivalenceDetector::find_similar(&target, &knowledge_base, 0.9);
        
        assert!(similar.len() >= 1);
        assert_eq!(similar[0].0, 0); // First animation is similar
    }
}