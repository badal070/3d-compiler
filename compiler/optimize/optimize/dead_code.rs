// compiler/optimize/optimize/dead_code.rs

use crate::ir::{Scene, Shape, Animation};
use std::collections::HashSet;

/// Eliminates unused shapes, equations, and animations
pub struct DeadCodeEliminator;

impl DeadCodeEliminator {
    pub fn eliminate(scene: Scene) -> Scene {
        let referenced = Self::find_referenced(&scene);
        
        Scene {
            shapes: scene.shapes.into_iter()
                .filter(|s| referenced.contains(&s.id))
                .collect(),
            
            animations: scene.animations.into_iter()
                .filter(|a| referenced.contains(&a.target_id))
                .collect(),
            
            constraints: scene.constraints,
            groups: scene.groups,
        }
    }

    fn find_referenced(scene: &Scene) -> HashSet<String> {
        let mut referenced = HashSet::new();

        // Mark all animated shapes as referenced
        for anim in &scene.animations {
            referenced.insert(anim.target_id.clone());
        }

        // Mark shapes in groups as referenced
        for group in &scene.groups {
            for member in &group.members {
                referenced.insert(member.clone());
            }
        }

        // Mark dependencies as referenced
        for shape in &scene.shapes {
            if let Some(deps) = &shape.dependencies {
                for dep in deps {
                    referenced.insert(dep.clone());
                }
            }
            
            // The shape itself might be referenced by the above
            if referenced.contains(&shape.id) {
                continue;
            }
            
            // If shape is referenced, keep it
            if Self::is_root_shape(shape, scene) {
                referenced.insert(shape.id.clone());
            }
        }

        referenced
    }

    fn is_root_shape(shape: &Shape, _scene: &Scene) -> bool {
        // A root shape is one that's explicitly created, not derived
        // Simplified: assume shapes without dependencies are roots
        shape.dependencies.is_none() || shape.dependencies.as_ref().unwrap().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    #[test]
    fn test_removes_unused_shapes() {
        let scene = Scene {
            shapes: vec![
                Shape {
                    id: "used".into(),
                    shape_type: ShapeType::Circle,
                    parameters: vec![],
                    transforms: vec![],
                    dependencies: None,
                },
                Shape {
                    id: "unused".into(),
                    shape_type: ShapeType::Circle,
                    parameters: vec![],
                    transforms: vec![],
                    dependencies: None,
                },
            ],
            animations: vec![
                Animation {
                    id: "anim1".into(),
                    target_id: "used".into(),
                    expression: Expr::Variable("t".into()),
                    time_domain: TimeDomain::Range { start: 0.0, end: 1.0 },
                    motion: Motion::Static { position: Point3D { x: 0.0, y: 0.0, z: 0.0 } },
                }
            ],
            constraints: vec![],
            groups: vec![],
        };

        let optimized = DeadCodeEliminator::eliminate(scene);
        
        assert_eq!(optimized.shapes.len(), 1);
        assert_eq!(optimized.shapes[0].id, "used");
    }

    #[test]
    fn test_keeps_grouped_shapes() {
        let scene = Scene {
            shapes: vec![
                Shape {
                    id: "shape1".into(),
                    shape_type: ShapeType::Circle,
                    parameters: vec![],
                    transforms: vec![],
                    dependencies: None,
                },
                Shape {
                    id: "shape2".into(),
                    shape_type: ShapeType::Circle,
                    parameters: vec![],
                    transforms: vec![],
                    dependencies: None,
                },
            ],
            animations: vec![],
            constraints: vec![],
            groups: vec![
                Group {
                    id: "group1".into(),
                    members: vec!["shape1".into(), "shape2".into()],
                }
            ],
        };

        let optimized = DeadCodeEliminator::eliminate(scene);
        
        assert_eq!(optimized.shapes.len(), 2);
    }
}