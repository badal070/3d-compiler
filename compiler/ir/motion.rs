pub mod motion {
    use crate::ids::{EntityId, MotionId};
    use crate::value::{Vector3, Angle};

    #[derive(Debug, Clone)]
    pub struct Motion {
        pub id: MotionId,
        pub target: EntityId,
        pub kind: MotionKind,
    }

    #[derive(Debug, Clone)]
    pub enum MotionKind {
        Rotation {
            axis: Vector3,
            speed: Angle,  // radians per second
        },
        Translation {
            direction: Vector3,
            speed: f64,  // units per second
        },
        Scale {
            factor: Vector3,
            speed: f64,  // scale change per second
        },
    }

    impl Motion {
        pub fn new(id: MotionId, target: EntityId, kind: MotionKind) -> Self {
            Self { id, target, kind }
        }

        pub fn rotation(id: MotionId, target: EntityId, axis: Vector3, speed: Angle) -> Self {
            Self::new(id, target, MotionKind::Rotation { axis, speed })
        }

        pub fn translation(id: MotionId, target: EntityId, direction: Vector3, speed: f64) -> Self {
            Self::new(id, target, MotionKind::Translation { direction, speed })
        }

        pub fn scale(id: MotionId, target: EntityId, factor: Vector3, speed: f64) -> Self {
            Self::new(id, target, MotionKind::Scale { factor, speed })
        }
    }
}