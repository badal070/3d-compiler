// compiler/optimize/optimize/constraint.rs

use crate::ir::{Constraint, Expr, Relation};
use std::collections::HashMap;

/// Optimizes constraints
/// Removes inactive constraints, pre-solves sets, reduces complexity
pub struct ConstraintOptimizer;

impl ConstraintOptimizer {
    pub fn optimize(constraints: Vec<Constraint>) -> Vec<Constraint> {
        let mut active = Self::remove_inactive(constraints);
        active = Self::resolve_contradictions(active);
        active = Self::simplify_graph(active);
        active
    }

    fn remove_inactive(constraints: Vec<Constraint>) -> Vec<Constraint> {
        constraints.into_iter()
            .filter(|c| !Self::is_tautology(c))
            .collect()
    }

    fn is_tautology(constraint: &Constraint) -> bool {
        match constraint {
            Constraint::Relation { left, op, right } => {
                // Check if relation is always true
                if let (Expr::Literal(l), Expr::Literal(r)) = (left, right) {
                    return Self::eval_relation(*l, op, *r);
                }
                false
            }

            Constraint::Domain { min, max, .. } => {
                // Empty domain is a tautology (always false, remove it)
                min > max
            }

            _ => false,
        }
    }

    fn eval_relation(left: f64, op: &Relation, right: f64) -> bool {
        match op {
            Relation::LessThan => left < right,
            Relation::GreaterThan => left > right,
            Relation::LessOrEqual => left <= right,
            Relation::GreaterOrEqual => left >= right,
            Relation::Equal => (left - right).abs() < 1e-10,
            Relation::NotEqual => (left - right).abs() >= 1e-10,
        }
    }

    fn resolve_contradictions(constraints: Vec<Constraint>) -> Vec<Constraint> {
        // Detect and remove contradictory constraints
        let mut domains: HashMap<String, (f64, f64)> = HashMap::new();
        let mut valid = Vec::new();

        for c in constraints {
            match &c {
                Constraint::Domain { var, min, max } => {
                    if let Some((existing_min, existing_max)) = domains.get(var) {
                        // Intersect domains
                        let new_min = min.max(*existing_min);
                        let new_max = max.min(*existing_max);

                        if new_min > new_max {
                            // Contradiction: skip this constraint
                            continue;
                        }

                        domains.insert(var.clone(), (new_min, new_max));
                    } else {
                        domains.insert(var.clone(), (*min, *max));
                    }
                }
                _ => {}
            }

            valid.push(c);
        }

        valid
    }

    fn simplify_graph(constraints: Vec<Constraint>) -> Vec<Constraint> {
        // Remove transitive dependencies
        // A depends on B, B depends on C → A depends on C (can optimize intermediate)
        
        // Simplified implementation: return as-is
        // Real implementation would build dependency graph and prune
        constraints
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tautology_removal() {
        let constraints = vec![
            Constraint::Relation {
                left: Expr::Literal(5.0),
                op: Relation::LessThan,
                right: Expr::Literal(10.0),
            },
            Constraint::Relation {
                left: Expr::Literal(10.0),
                op: Relation::LessThan,
                right: Expr::Literal(5.0),
            },
        ];

        let result = ConstraintOptimizer::optimize(constraints);
        
        // First is always true (tautology), second is always false (removed)
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_contradiction_detection() {
        let constraints = vec![
            Constraint::Domain {
                var: "x".into(),
                min: 0.0,
                max: 10.0,
            },
            Constraint::Domain {
                var: "x".into(),
                min: 20.0,
                max: 30.0,
            },
        ];

        let result = ConstraintOptimizer::optimize(constraints);
        
        // Contradictory domains should be detected
        assert_eq!(result.len(), 1); // Only first is kept
    }

    #[test]
    fn test_domain_intersection() {
        let constraints = vec![
            Constraint::Domain {
                var: "x".into(),
                min: 0.0,
                max: 10.0,
            },
            Constraint::Domain {
                var: "x".into(),
                min: 5.0,
                max: 15.0,
            },
        ];

        let result = ConstraintOptimizer::optimize(constraints);
        
        // Domains intersect: should keep both
        assert_eq!(result.len(), 2);
    }
}