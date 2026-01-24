// compiler/optimize/canonical/time_normalize.rs

use crate::ir::{TimeDomain, Expr, BinaryOp};

/// Normalizes time domains and aligns sampling windows
pub struct TimeNormalizer;

#[derive(Debug, Clone)]
pub struct NormalizedTime {
    pub domain: TimeDomain,
    pub offset: f64,      // Original start time
    pub duration: f64,    // Original duration
}

impl TimeNormalizer {
    /// Normalize time domain to [0, duration]
    pub fn normalize(domain: TimeDomain) -> NormalizedTime {
        match domain {
            TimeDomain::Range { start, end } => {
                let duration = end - start;
                NormalizedTime {
                    domain: TimeDomain::Range { start: 0.0, end: duration },
                    offset: start,
                    duration,
                }
            }

            TimeDomain::Periodic { period, phase } => {
                // Normalize phase to [0, period)
                let normalized_phase = phase % period;
                NormalizedTime {
                    domain: TimeDomain::Periodic {
                        period,
                        phase: normalized_phase,
                    },
                    offset: phase,
                    duration: period,
                }
            }

            TimeDomain::Infinite => {
                NormalizedTime {
                    domain: TimeDomain::Infinite,
                    offset: 0.0,
                    duration: f64::INFINITY,
                }
            }
        }
    }

    /// Rebase time expression to normalized domain
    pub fn rebase_expr(expr: Expr, offset: f64) -> Expr {
        if offset == 0.0 {
            return expr;
        }

        // t → t + offset
        Expr::Binary(
            BinaryOp::Add,
            Box::new(expr),
            Box::new(Expr::Literal(offset)),
        )
    }

    /// Align multiple time domains to common reference
    pub fn align_domains(domains: Vec<TimeDomain>) -> (Vec<NormalizedTime>, f64) {
        let normalized: Vec<_> = domains.into_iter()
            .map(Self::normalize)
            .collect();

        // Find common start time
        let min_offset = normalized.iter()
            .map(|nt| nt.offset)
            .fold(f64::INFINITY, f64::min);

        // Adjust all offsets relative to earliest start
        let aligned = normalized.into_iter()
            .map(|mut nt| {
                nt.offset -= min_offset;
                nt
            })
            .collect();

        (aligned, min_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_normalization() {
        let domain = TimeDomain::Range { start: 5.0, end: 10.0 };
        let normalized = TimeNormalizer::normalize(domain);

        assert_eq!(normalized.offset, 5.0);
        assert_eq!(normalized.duration, 5.0);
        
        if let TimeDomain::Range { start, end } = normalized.domain {
            assert_eq!(start, 0.0);
            assert_eq!(end, 5.0);
        }
    }

    #[test]
    fn test_periodic_normalization() {
        let domain = TimeDomain::Periodic { period: 2.0, phase: 5.5 };
        let normalized = TimeNormalizer::normalize(domain);

        if let TimeDomain::Periodic { period, phase } = normalized.domain {
            assert_eq!(period, 2.0);
            assert_eq!(phase, 1.5); // 5.5 % 2.0
        }
    }

    #[test]
    fn test_domain_alignment() {
        let domains = vec![
            TimeDomain::Range { start: 5.0, end: 10.0 },
            TimeDomain::Range { start: 2.0, end: 8.0 },
            TimeDomain::Range { start: 7.0, end: 12.0 },
        ];

        let (aligned, min_offset) = TimeNormalizer::align_domains(domains);

        assert_eq!(min_offset, 2.0);
        assert_eq!(aligned[0].offset, 3.0); // 5 - 2
        assert_eq!(aligned[1].offset, 0.0); // 2 - 2
        assert_eq!(aligned[2].offset, 5.0); // 7 - 2
    }
}