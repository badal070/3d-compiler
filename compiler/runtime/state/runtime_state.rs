use serde::{Serialize, Deserialize};
use super::{WorldState, TimeState};

#[derive(Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    pub world: WorldState,
    pub time: TimeState,
}

impl RuntimeState {
    pub fn new(world: WorldState, time: TimeState) -> Self {
        Self { world, time }
    }
}
