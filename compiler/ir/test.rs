pub use ids::*;
pub use value::*;
pub use entity::*;
pub use component::*;
pub use constraint::*;
pub use motion::*;
pub use timeline::*;
pub use scene::*;

// ============================================================================
// EXAMPLE USAGE
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_scene_construction() {
        let mut id_gen = IDGenerator::new();
        let mut scene = Scene::new();

        // Create a rotating cube
        let cube_id = id_gen.next_entity();
        let mut cube = Entity::new(cube_id, EntityKind::Solid);
        cube.add_component(Component::Transform(Transform::identity()));
        cube.add_component(Component::Geometry(Geometry::Primitive(Primitive::Cube)));
        cube.add_component(Component::Physical(Physical::rigid_body(1.0)));
        scene.add_entity(cube);

        // Create rotation motion
        let motion_id = id_gen.next_motion();
        let rotation = Motion::rotation(
            motion_id,
            cube_id,
            Vector3::new(0.0, 1.0, 0.0),
            Angle::radians(std::f64::consts::PI), // 180 degrees per second
        );
        scene.add_motion(rotation);

        // Create timeline
        let timeline_id = id_gen.next_timeline();
        let mut timeline = Timeline::new(timeline_id);
        timeline.add_event(TimedEvent::new(
            Time::seconds(0.0),
            Time::seconds(2.0),
            motion_id,
        ));
        scene.add_timeline(timeline);

        // Validate scene
        assert!(scene.validate().is_ok());
        assert_eq!(scene.entities.len(), 1);
        assert_eq!(scene.motions.len(), 1);
        assert_eq!(scene.timelines.len(), 1);
    }

    #[test]
    fn test_gear_constraint() {
        let mut id_gen = IDGenerator::new();
        let driver_id = id_gen.next_entity();
        let driven_id = id_gen.next_entity();

        let constraint = Constraint::gear_relation(driver_id, driven_id, 2.0);
        
        assert!(constraint.references_entity(driver_id));
        assert!(constraint.references_entity(driven_id));
    }

    #[test]
    fn test_timeline_queries() {
        let mut id_gen = IDGenerator::new();
        let motion_id = id_gen.next_motion();
        
        let mut timeline = Timeline::new(id_gen.next_timeline());
        timeline.add_event(TimedEvent::new(
            Time::seconds(1.0),
            Time::seconds(2.0),
            motion_id,
        ));

        let active_at_0 = timeline.events_at(Time::seconds(0.0));
        let active_at_2 = timeline.events_at(Time::seconds(2.0));
        
        assert_eq!(active_at_0.len(), 0);
        assert_eq!(active_at_2.len(), 1);
    }
}