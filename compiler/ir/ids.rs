pub mod ids {
    pub type EntityId = u64;
    pub type MotionId = u64;
    pub type TimelineId = u64;

    pub struct IDGenerator {
        entity_counter: u64,
        motion_counter: u64,
        timeline_counter: u64,
    }

    impl IDGenerator {
        pub fn new() -> Self {
            Self {
                entity_counter: 0,
                motion_counter: 0,
                timeline_counter: 0,
            }
        }

        pub fn next_entity(&mut self) -> EntityId {
            self.entity_counter += 1;
            self.entity_counter
        }

        pub fn next_motion(&mut self) -> MotionId {
            self.motion_counter += 1;
            self.motion_counter
        }

        pub fn next_timeline(&mut self) -> TimelineId {
            self.timeline_counter += 1;
            self.timeline_counter
        }
    }

    impl Default for IDGenerator {
        fn default() -> Self {
            Self::new()
        }
    }
}