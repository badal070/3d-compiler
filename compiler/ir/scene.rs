pub mod scene {
    use crate::entity::Entity;
    use crate::motion::Motion;
    use crate::timeline::Timeline;
    use crate::constraint::Constraint;
    use crate::ids::{EntityId, MotionId};

    #[derive(Debug, Clone)]
    pub struct Scene {
        pub entities: Vec<Entity>,
        pub motions: Vec<Motion>,
        pub timelines: Vec<Timeline>,
        pub constraints: Vec<Constraint>,
    }

    impl Scene {
        pub fn new() -> Self {
            Self {
                entities: Vec::new(),
                motions: Vec::new(),
                timelines: Vec::new(),
                constraints: Vec::new(),
            }
        }

        pub fn add_entity(&mut self, entity: Entity) {
            self.entities.push(entity);
        }

        pub fn add_motion(&mut self, motion: Motion) {
            self.motions.push(motion);
        }

        pub fn add_timeline(&mut self, timeline: Timeline) {
            self.timelines.push(timeline);
        }

        pub fn add_constraint(&mut self, constraint: Constraint) {
            self.constraints.push(constraint);
        }

        pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
            self.entities.iter().find(|e| e.id == id)
        }

        pub fn get_motion(&self, id: MotionId) -> Option<&Motion> {
            self.motions.iter().find(|m| m.id == id)
        }

        pub fn validate(&self) -> Result<(), String> {
            // Validate all motion targets exist
            for motion in &self.motions {
                if self.get_entity(motion.target).is_none() {
                    return Err(format!("Motion {} targets non-existent entity {}", motion.id, motion.target));
                }
            }

            // Validate all timeline events reference existing motions
            for timeline in &self.timelines {
                for event in &timeline.events {
                    if self.get_motion(event.motion).is_none() {
                        return Err(format!("Timeline {} references non-existent motion {}", timeline.id, event.motion));
                    }
                }
            }

            // Validate all constraints reference existing entities
            for constraint in &self.constraints {
                match constraint {
                    Constraint::GearRelation { driver, driven, .. } => {
                        if self.get_entity(*driver).is_none() {
                            return Err(format!("Constraint references non-existent driver entity {}", driver));
                        }
                        if self.get_entity(*driven).is_none() {
                            return Err(format!("Constraint references non-existent driven entity {}", driven));
                        }
                    }
                    Constraint::ParentChild { parent, child } => {
                        if self.get_entity(*parent).is_none() {
                            return Err(format!("Constraint references non-existent parent entity {}", parent));
                        }
                        if self.get_entity(*child).is_none() {
                            return Err(format!("Constraint references non-existent child entity {}", child));
                        }
                    }
                    Constraint::Distance { entity_a, entity_b, .. } => {
                        if self.get_entity(*entity_a).is_none() {
                            return Err(format!("Constraint references non-existent entity {}", entity_a));
                        }
                        if self.get_entity(*entity_b).is_none() {
                            return Err(format!("Constraint references non-existent entity {}", entity_b));
                        }
                    }
                    _ => {}
                }
            }

            Ok(())
        }
    }

    impl Default for Scene {
        fn default() -> Self {
            Self::new()
        }
    }
}


pub use ids::*;
pub use value::*;
pub use entity::*;
pub use component::*;
pub use constraint::*;
pub use motion::*;
pub use timeline::*;
pub use scene::*;