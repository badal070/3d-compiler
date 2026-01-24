// compiler/optimize/canonical/transform_normalize.rs

use crate::ir::{Transform, Mat4};

/// Normalizes geometric transforms to canonical form
/// Standard order: Scale → Rotate → Translate
pub struct TransformNormalizer;

impl TransformNormalizer {
    /// Normalize transform list to canonical form
    pub fn normalize(transforms: Vec<Transform>) -> Vec<Transform> {
        // Remove identity transforms
        let non_identity: Vec<_> = transforms.into_iter()
            .filter(|t| !Self::is_identity(t))
            .collect();

        // Collapse consecutive transforms of same type
        let collapsed = Self::collapse_transforms(non_identity);

        // Reorder to canonical: Scale → Rotate → Translate
        Self::reorder_canonical(collapsed)
    }

    fn is_identity(transform: &Transform) -> bool {
        match transform {
            Transform::Translate(x, y, z) => {
                Self::is_zero(*x) && Self::is_zero(*y) && Self::is_zero(*z)
            }
            Transform::Scale(x, y, z) => {
                Self::approx_eq(*x, 1.0) &&
                Self::approx_eq(*y, 1.0) &&
                Self::approx_eq(*z, 1.0)
            }
            Transform::Rotate(angle, _, _, _) => {
                Self::is_zero(*angle)
            }
            Transform::Matrix(m) => m.is_identity(),
        }
    }

    fn is_zero(val: f64) -> bool {
        val.abs() < 1e-10
    }

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    fn collapse_transforms(transforms: Vec<Transform>) -> Vec<Transform> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < transforms.len() {
            let current = &transforms[i];
            
            // Look ahead for mergeable transforms
            let mut merged = current.clone();
            let mut j = i + 1;
            
            while j < transforms.len() {
                if let Some(combined) = Self::try_merge(&merged, &transforms[j]) {
                    merged = combined;
                    j += 1;
                } else {
                    break;
                }
            }

            result.push(merged);
            i = j;
        }

        result
    }

    fn try_merge(a: &Transform, b: &Transform) -> Option<Transform> {
        match (a, b) {
            // Merge translations
            (Transform::Translate(x1, y1, z1), Transform::Translate(x2, y2, z2)) => {
                Some(Transform::Translate(x1 + x2, y1 + y2, z1 + z2))
            }

            // Merge scales
            (Transform::Scale(x1, y1, z1), Transform::Scale(x2, y2, z2)) => {
                Some(Transform::Scale(x1 * x2, y1 * y2, z1 * z2))
            }

            // Merge rotations around same axis
            (Transform::Rotate(a1, x1, y1, z1), Transform::Rotate(a2, x2, y2, z2))
                if Self::same_axis((*x1, *y1, *z1), (*x2, *y2, *z2)) => {
                Some(Transform::Rotate(a1 + a2, *x1, *y1, *z1))
            }

            _ => None,
        }
    }

    fn same_axis(a: (f64, f64, f64), b: (f64, f64, f64)) -> bool {
        (a.0 - b.0).abs() < 1e-6 &&
        (a.1 - b.1).abs() < 1e-6 &&
        (a.2 - b.2).abs() < 1e-6
    }

    fn reorder_canonical(transforms: Vec<Transform>) -> Vec<Transform> {
        let mut scales = Vec::new();
        let mut rotates = Vec::new();
        let mut translates = Vec::new();
        let mut matrices = Vec::new();

        for t in transforms {
            match t {
                Transform::Scale(_, _, _) => scales.push(t),
                Transform::Rotate(_, _, _, _) => rotates.push(t),
                Transform::Translate(_, _, _) => translates.push(t),
                Transform::Matrix(_) => matrices.push(t),
            }
        }

        // Canonical order: Scale → Rotate → Translate → Matrix
        scales.into_iter()
            .chain(rotates)
            .chain(translates)
            .chain(matrices)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_removal() {
        let transforms = vec![
            Transform::Translate(1.0, 2.0, 3.0),
            Transform::Translate(0.0, 0.0, 0.0), // Identity
            Transform::Scale(2.0, 2.0, 2.0),
        ];

        let result = TransformNormalizer::normalize(transforms);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_transform_merging() {
        let transforms = vec![
            Transform::Translate(1.0, 2.0, 3.0),
            Transform::Translate(4.0, 5.0, 6.0),
        ];

        let result = TransformNormalizer::normalize(transforms);
        assert_eq!(result.len(), 1);
        
        if let Transform::Translate(x, y, z) = result[0] {
            assert_eq!(x, 5.0);
            assert_eq!(y, 7.0);
            assert_eq!(z, 9.0);
        }
    }

    #[test]
    fn test_canonical_ordering() {
        let transforms = vec![
            Transform::Translate(1.0, 0.0, 0.0),
            Transform::Scale(2.0, 2.0, 2.0),
            Transform::Rotate(90.0, 0.0, 1.0, 0.0),
        ];

        let result = TransformNormalizer::normalize(transforms);
        
        // Should be: Scale, Rotate, Translate
        assert!(matches!(result[0], Transform::Scale(_, _, _)));
        assert!(matches!(result[1], Transform::Rotate(_, _, _, _)));
        assert!(matches!(result[2], Transform::Translate(_, _, _)));
    }
}