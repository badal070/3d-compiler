// compiler/runtime/mod.rs
// Simplified runtime for web demo

pub mod state;
pub mod error;
pub mod loader;
pub mod snapshot_builder;

pub use state::RuntimeState;
pub use loader::SceneLoader;
pub use snapshot_builder::{SnapshotBuilder, RendererSnapshot};