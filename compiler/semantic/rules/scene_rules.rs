pub struct SceneRuleEngine {
    symbol_table: Arc<SymbolTable>,
}

impl SceneRuleEngine {
    /// Detect cycles in scene graph
    pub fn check_acyclicity(&self, root: NodeId) -> Result<(), CyclicGraphError> {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();
        
        fn dfs(
            node: NodeId,
            visited: &mut HashSet<NodeId>,
            stack: &mut HashSet<NodeId>,
            graph: &SceneGraph,
        ) -> Result<(), CyclicGraphError> {
            if stack.contains(&node) {
                return Err(CyclicGraphError::cycle_detected(node));
            }
            if visited.contains(&node) {
                return Ok(());
            }
            
            stack.insert(node);
            for child in graph.children(node) {
                dfs(child, visited, stack, graph)?;
            }
            stack.remove(&node);
            visited.insert(node);
            Ok(())
        }
        
        dfs(root, &mut visited, &mut stack, self.graph())
    }
    
    /// Verify all object references resolve
    pub fn check_references(&self, ir: &IR) -> Result<(), UndefinedReferenceError> {
        for node in ir.nodes() {
            for reference in node.get_references() {
                if self.symbol_table.resolve(&reference).is_none() {
                    return Err(UndefinedReferenceError {
                        name: reference.clone(),
                        referenced_at: node.location(),
                    });
                }
            }
        }
        Ok(())
    }
}