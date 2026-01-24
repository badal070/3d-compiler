// runtime/state/mod.rs
// State system - this is sacred
// All state must be serializable. No exceptions.

pub mod world_state;
pub mod object_state;
pub mod parameter_state;
pub mod time_state;

pub use world_state::WorldState;
pub use object_state::{ObjectState, ObjectId};
pub use parameter_state::{ParameterState, ParameterId};
pub use time_state::TimeState;

use serde::{Deserialize, Serialize};

/// Complete runtime state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    pub world: WorldState,
    pub time: TimeState,
    pub checksum: StateChecksum,
}

/// State integrity verification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StateChecksum {
    pub object_count: usize,
    pub parameter_count: usize,
    pub constraint_count: usize,
    pub time_hash: u64,
}

impl RuntimeState {
    pub fn new(world: WorldState, time: TimeState) -> Self {
        let checksum = StateChecksum {
            object_count: world.objects.len(),
            parameter_count: world.parameters.values().len(),
            constraint_count: world.constraints.len(),
            time_hash: time.current_time.to_bits(),
        };
        Self {
            world,
            time,
            checksum,
        }
    }

    pub fn verify(&self) -> bool {
        self.checksum.object_count == self.world.objects.len()
            && self.checksum.parameter_count == self.world.parameters.values().len()
            && self.checksum.constraint_count == self.world.constraints.len()
            && self.checksum.time_hash == self.time.current_time.to_bits()
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.verify() {
            return Err("State checksum mismatch".to_string());
        }

        self.world.validate()?;
        self.time.validate()?;

        Ok(())
    }
}