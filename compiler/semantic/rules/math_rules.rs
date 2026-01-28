pub struct MathRuleEngine {
    context: Arc<SemanticContext>,
}

impl MathRuleEngine {
    /// Check equation dimensional correctness
    pub fn check_dimensions(&self, expr: &Expr) -> Result<Unit, DimensionalError> {
        match expr {
            Expr::Binary(op, lhs, rhs) => {
                let lhs_unit = self.check_dimensions(lhs)?;
                let rhs_unit = self.check_dimensions(rhs)?;
                
                match op {
                    BinOp::Add | BinOp::Sub => {
                        if lhs_unit != rhs_unit {
                            return Err(DimensionalError::Mismatch {
                                expected: lhs_unit,
                                found: rhs_unit,
                                location: expr.location(),
                            });
                        }
                        Ok(lhs_unit)
                    }
                    BinOp::Mul => Ok(lhs_unit * rhs_unit),
                    BinOp::Div => Ok(lhs_unit / rhs_unit),
                    _ => todo!(),
                }
            }
            // ... other cases
        }
    }
    
    /// Check if expression is differentiable at runtime
    pub fn check_differentiability(&self, expr: &Expr) -> Result<(), NonDifferentiableError> {
        // Detect abs(), step functions, discontinuities
    }
    
    /// Detect potential singularities
    pub fn check_singularities(&self, expr: &Expr) -> Result<(), SingularityError> {
        // Check for division by zero, log(0), tan(π/2), etc.
    }
}
```

