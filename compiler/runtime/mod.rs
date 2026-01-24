// runtime/mod.rs
// Runtime execution and state engine
// No physics shortcuts. Pure mathematical state machine.

pub mod engine;
pub mod state;
pub mod executor;
pub mod constraint;
pub mod motion;
pub mod error;
pub mod snapshot;

pub use engine::RuntimeEngine;
pub use error::{RuntimeError, RuntimeResult};
pub use snapshot::Snapshot;

use state::WorldState;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum execution steps before watchdog triggers
    pub max_steps: usize,
    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,
    /// Time step for integration (fixed or adaptive)
    pub time_step: TimeStep,
    /// Enable snapshot history
    pub enable_snapshots: bool,
    /// Maximum snapshots to keep
    pub max_snapshots: usize,
    /// Constraint solver tolerance
    pub constraint_tolerance: f64,
    /// Maximum constraint iterations
    pub max_constraint_iterations: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TimeStep {
    Fixed(f64),
    Adaptive { min: f64, max: f64, target_error: f64 },
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_steps: 100_000,
            max_execution_time_ms: 5000,
            time_step: TimeStep::Fixed(0.016), // ~60 FPS
            enable_snapshots: true,
            max_snapshots: 1000,
            constraint_tolerance: 1e-6,
            max_constraint_iterations: 100,
        }
    }
}

/// Runtime execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionState {
    Idle,
    Running,
    Paused,
    Stopped,
    Error,
}

/// Runtime control commands
#[derive(Debug, Clone, Copy)]
pub enum RuntimeCommand {
    Start,
    Pause,
    Resume,
    Stop,
    Step,
    Reset,
}