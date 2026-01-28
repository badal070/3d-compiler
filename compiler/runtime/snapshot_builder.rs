// runtime/snapshot_builder.rs
// Builds immutable snapshots from runtime state for renderer consumption

use crate::state::{RuntimeState, ObjectState as StateObjectState};
use serde::{Deserialize, Serialize};

/// Snapshot builder - converts runtime state to renderer-friendly format
pub struct SnapshotBuilder {
    #[allow(dead_code)]
    next_id: u64,
}

impl SnapshotBuilder {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    /// Build a snapshot from current runtime state
    pub fn build_snapshot(&mut self, state: &RuntimeState) -> RendererSnapshot {
        let tick = state.time.step_count;
        let timestamp = state.time.current_time;

        let objects: Vec<SnapshotObject> = state
            .world
            .objects
            .iter()
            .map(|(id, obj)| self.convert_object(id, obj))
            .collect();

        RendererSnapshot {
            tick,
            timestamp,
            objects,
            focus_ids: vec![], // TODO: implement focus tracking
        }
    }

    fn convert_object(&self, id: &str, obj: &StateObjectState) -> SnapshotObject {
        SnapshotObject {
            id: self.object_id_hash(id),
            geometry: self.convert_geometry(obj),
            transform: SnapshotTransform {
                position: [obj.position.x, obj.position.y, obj.position.z],
                rotation: [obj.orientation.x, obj.orientation.y, obj.orientation.z, obj.orientation.w],
                scale: [obj.scale.x, obj.scale.y, obj.scale.z],
            },
            material: SnapshotMaterial {
                color: [0.5, 0.7, 1.0, 1.0],
                metallic: 0.3,
                roughness: 0.7,
                opacity: 1.0,
                emissive: [0.0, 0.0, 0.0],
            },
            visible: obj.visible,
            highlighted: false,
        }
    }

    fn convert_geometry(&self, obj: &StateObjectState) -> SnapshotGeometry {
        match obj.kind {
            crate::state::ObjectKind::Sphere => SnapshotGeometry::Sphere { radius: 1.0 },
            crate::state::ObjectKind::Box => SnapshotGeometry::Box {
                width: obj.scale.x,
                height: obj.scale.y,
                depth: obj.scale.z,
            },
            crate::state::ObjectKind::Cylinder => SnapshotGeometry::Cylinder {
                radius: obj.scale.x * 0.5,
                height: obj.scale.y,
            },
            crate::state::ObjectKind::Plane => SnapshotGeometry::Plane {
                width: obj.scale.x,
                height: obj.scale.z,
            },
            _ => SnapshotGeometry::Box {
                width: 1.0,
                height: 1.0,
                depth: 1.0,
            },
        }
    }

    fn object_id_hash(&self, id: &str) -> u64 {
        // Simple hash for demo - would use proper hash in production
        id.bytes().map(|b| b as u64).sum()
    }
}

impl Default for SnapshotBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable snapshot sent to renderer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererSnapshot {
    pub tick: u64,
    pub timestamp: f64,
    pub objects: Vec<SnapshotObject>,
    pub focus_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotObject {
    pub id: u64,
    pub geometry: SnapshotGeometry,
    pub transform: SnapshotTransform,
    pub material: SnapshotMaterial,
    pub visible: bool,
    pub highlighted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SnapshotGeometry {
    Sphere { radius: f64 },
    Box { width: f64, height: f64, depth: f64 },
    Cylinder { radius: f64, height: f64 },
    Plane { width: f64, height: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotTransform {
    pub position: [f64; 3],
    pub rotation: [f64; 4], // quaternion [x, y, z, w]
    pub scale: [f64; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMaterial {
    pub color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub opacity: f32,
    pub emissive: [f32; 3],
}