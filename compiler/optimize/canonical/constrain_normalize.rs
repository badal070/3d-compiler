// compiler/optimize/canonical/constraint_normalize.rs

use crate::ir::{Constraint, Expr, Relation};
use std::collections::HashSet;

/// Normalizes constraints to canonical forms
pub struct ConstraintNormalizer;

impl ConstraintNormalizer {
    /// Normalize constraint set
    pub fn normalize(constraints: Vec<Constraint>) -> Vec<Constraint> {
        let mut normalized = constraints.into_iter()
            .map(Self::normalize_constraint)
            .collect::<Vec<_>>();

        // Sort for canonical ordering
        normalized.sort_by_key(|c| Self::constraint_order(c));

        // Remove duplicates
        Self::deduplicate(normalized)
    }

    fn normalize_constraint(constraint: Constraint) -> Constraint {
        match constraint {
            Constraint::Relation { left, op, right } => {
                // Canonical form: simpler expression on left
                let (left, op, right) = Self::canonicalize_relation(left, op, right);
                Constraint::Relation { left, op, right }
            }

            Constraint::Domain { var, min, max } => {
                // Ensure min <= max
                let (min, max) = if min <= max {
                    (min, max)
                } else {
                    (max, min)
                };
                Constraint::Domain { var, min, max }
            }

            Constraint::Dependency { target, sources } => {
                // Sort dependencies
                let mut sources = sources;
                sources.sort();
                Constraint::Dependency { target, sources }
            }

            other => other,
        }
    }

    fn canonicalize_relation(
        left: Expr,
        op: Relation,
        right: Expr
    ) -> (Expr, Relation, Expr) {
        // Move constants to the right
        match (&left, &right) {
            (Expr::Literal(_), Expr::Variable(_)) |
            (Expr::Literal(_), Expr::Binary(_, _, _)) => {
                // Flip: 5 < x → x > 5
                (right, Self::flip_relation(op), left)
            }
            _ => (left, op, right),
        }
    }

    fn flip_relation(rel: Relation) -> Relation {
        match rel {
            Relation::LessThan => Relation::GreaterThan,
            Relation::GreaterThan => Relation::LessThan,
            Relation::LessOrEqual => Relation::GreaterOrEqual,
            Relation::GreaterOrEqual => Relation::LessOrEqual,
            Relation::Equal => Relation::Equal,
            Relation::NotEqual => Relation::NotEqual,
        }
    }

    fn constraint_order(constraint: &Constraint) -> u32 {
        match constraint {
            Constraint::Domain { .. } => 0,
            Constraint::Relation { .. } => 1,
            Constraint::Dependency { .. } => 2,
            _ => 3,
        }
    }

    fn deduplicate(constraints: Vec<Constraint>) -> Vec<Constraint> {
        let mut seen = HashSet::new();
        constraints.into_iter()
            .filter(|c| seen.insert(Self::constraint_hash(c)))
            .collect()
    }

    fn constraint_hash(constraint: &Constraint) -> String {
        // Simple string representation for hashing
        format!("{:?}", constraint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_ordering() {
        let constraint = Constraint::Domain {
            var: "x".into(),
            min: 10.0,
            max: 5.0,
        };

        let normalized = ConstraintNormalizer::normalize_constraint(constraint);

        if let Constraint::Domain { min, max, .. } = normalized {
            assert_eq!(min, 5.0);
            assert_eq!(max, 10.0);
        }
    }

    #[test]
    fn test_relation_canonicalization() {
        let constraint = Constraint::Relation {
            left: Expr::Literal(5.0),
            op: Relation::LessThan,
            right: Expr::Variable("x".into()),
        };

        let normalized = ConstraintNormalizer::normalize_constraint(constraint);

        if let Constraint::Relation { left, op, right } = normalized {
            assert!(matches!(left, Expr::Variable(_)));
            assert!(matches!(op, Relation::GreaterThan));
            assert!(matches!(right, Expr::Literal(_)));
        }
    }

    #[test]
    fn test_deduplication() {
        let constraints = vec![
            Constraint::Domain { var: "x".into(), min: 0.0, max: 1.0 },
            Constraint::Domain { var: "x".into(), min: 0.0, max: 1.0 },
            Constraint::Domain { var: "y".into(), min: 0.0, max: 1.0 },
        ];

        let result = ConstraintNormalizer::normalize(constraints);
        assert_eq!(result.len(), 2);
    }
}