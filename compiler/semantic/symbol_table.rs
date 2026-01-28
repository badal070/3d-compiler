pub struct SymbolTable {
    /// Scoped symbol map (supports nested scopes)
    scopes: Vec<Scope>,
    
    /// Global object registry (scene nodes, animations)
    globals: HashMap<SymbolId, SymbolEntry>,
    
    /// Reverse lookup: IR node → symbol
    node_to_symbol: HashMap<NodeId, SymbolId>,
}

pub struct SymbolEntry {
    pub id: SymbolId,
    pub name: String,
    pub kind: SymbolKind,  // Object, Animation, Parameter, Constant
    pub ir_node: NodeId,
    pub scope_depth: usize,
    pub declared_at: SourceLocation,
}

pub enum SymbolKind {
    SceneObject,
    AnimationClip,
    Parameter,
    LocalVariable,
    Constant,
}