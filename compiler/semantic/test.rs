#[test]
fn test_full_validation_pipeline() {
    let ir = parse_animation("pendulum.anim");
    let result = Validator::validate(&ir);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().diagnostics.errors.len(), 0);
}

#[test]
fn test_cyclic_graph_rejection() {
    let ir = parse_animation("cyclic_scene.anim");
    let result = Validator::validate(&ir);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        SemanticError::CyclicGraph(_)
    ));
}