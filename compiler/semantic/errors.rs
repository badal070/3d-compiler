pub enum SemanticError {
    // Symbol errors
    UndefinedSymbol(UndefinedSymbolError),
    SymbolShadowing(ShadowingError),
    
    // Type errors
    TypeMismatch(TypeMismatchError),
    InvalidCast(InvalidCastError),
    
    // Math errors
    DimensionalInconsistency(DimensionalError),
    Singularity(SingularityError),
    NonDifferentiable(NonDifferentiableError),
    
    // Scene errors
    CyclicGraph(CyclicGraphError),
    UndefinedReference(UndefinedReferenceError),
    
    // Physics errors
    ConstraintConflict(ConstraintConflictError),
    UnboundedEnergy(UnboundedEnergyError),
    InvalidCollision(CollisionError),
    
    // Time errors
    TimeDomainMismatch(TimeDomainMismatchError),
    InvalidSampling(SamplingError),
}
```

### 4.2 Error Presentation Example
```
error[E0312]: dimensional inconsistency in expression
  --> pendulum.anim:15:20
   |
15 | omega = sqrt(g + length)
   |              ^^^^^^^^^^
   |              |     |
   |              |     this has type `Length` (m)
   |              this has type `Acceleration` (m/s²)
   |
   = note: cannot add values with incompatible dimensions
   = help: did you mean `sqrt(g / length)`?