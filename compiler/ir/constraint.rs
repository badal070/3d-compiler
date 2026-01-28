use crate::ids::EntityId;
use crate::values::Vector3;

#[derive(Debug, Clone)]
pub enum Constraint {
    FixedAxis {
        axis: Vector3,
    },
    GearRelation {
        driver: EntityId,
        driven: EntityId,
        ratio: f64,
    },
    ParentChild {
        parent: EntityId,
        child: EntityId,
    },
    Distance {
        entity_a: EntityId,
        entity_b: EntityId,
        distance: f64,
    },
}

impl Constraint {
    pub fn fixed_axis(axis: Vector3) -> Self {
        Constraint::FixedAxis { axis }
    }

    pub fn gear_relation(driver: EntityId, driven: EntityId, ratio: f64) -> Self {
        Constraint::GearRelation { driver, driven, ratio }
    }

    pub fn parent_child(parent: EntityId, child: EntityId) -> Self {
        Constraint::ParentChild { parent, child }
    }

    pub fn distance(entity_a: EntityId, entity_b: EntityId, distance: f64) -> Self {
        Constraint::Distance { entity_a, entity_b, distance }
    }

    pub fn references_entity(&self, entity_id: EntityId) -> bool {
        match self {
            Constraint::FixedAxis { .. } => false,
            Constraint::GearRelation { driver, driven, .. } => {
                *driver == entity_id || *driven == entity_id
            }
            Constraint::ParentChild { parent, child } => {
                *parent == entity_id || *child == entity_id
            }
            Constraint::Distance { entity_a, entity_b, .. } => {
                *entity_a == entity_id || *entity_b == entity_id
            }
        }
    }
}
