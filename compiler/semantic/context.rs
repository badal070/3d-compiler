pub struct SemanticContext {
    /// Type system reference (from type module)
    type_system: Arc<TypeSystem>,
    
    /// Unit registry (SI + custom units)
    units: UnitRegistry,
    
    /// Numerical tolerances for floating-point comparisons
    tolerances: ToleranceConfig,
    
    /// Physics constants (G, c, ℏ, etc.)
    constants: PhysicsConstants,
    
    /// Constraint compatibility matrix
    constraint_rules: ConstraintRuleSet,
    
    /// Time resolution limits
    time_limits: TimeLimits,
}

pub struct ToleranceConfig {
    /// Epsilon for float equality (default: 1e-9)
    pub float_epsilon: f64,
    
    /// Maximum relative error for unit conversion
    pub unit_conversion_tolerance: f64,
    
    /// Minimum time step (prevents division by infinitesimal dt)
    pub min_time_delta: f64,
}

pub struct UnitRegistry {
    /// Base SI units + derived units
    units: HashMap<UnitId, UnitDefinition>,
    
    /// Conversion graph for dimensional analysis
    conversion_graph: UnitConversionGraph,
}