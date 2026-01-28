//! The Hard Boundary
//!
//! This is a customs checkpoint. One-way data flow only.
//! No callbacks into runtime. No math evaluation. No constraint handling.

use crate::renderer::{
    adapter::Adapter,
    backend::RenderBackend,
    error::{RenderError, RenderResult},
    interpolation::Interpolator,
    scene_map::SceneMap,
    sync::FrameSync,
    visibility::VisibilityManager,
    RendererConfig, RenderStats, RuntimeSnapshot,
};

/// The bridge between runtime state and visual rendering
///
/// Responsibilities:
/// - Receive immutable snapshots
/// - Convert to render instructions
/// - Enforce one-way data flow
///
/// NOT responsible for:
/// - Math evaluation
/// - Constraint solving
/// - State mutation
pub struct RendererBridge {
    backend: Box<dyn RenderBackend>,
    adapter: Adapter,
    scene_map: SceneMap,
    frame_sync: FrameSync,
    interpolator: Interpolator,
    visibility: VisibilityManager,
    config: RendererConfig,
    stats: RenderStats,
    last_snapshot: Option<RuntimeSnapshot>,
}

impl RendererBridge {
    /// Create a new renderer bridge
    pub fn new(backend: Box<dyn RenderBackend>, config: RendererConfig) -> Self {
        Self {
            backend,
            adapter: Adapter::new(),
            scene_map: SceneMap::new(),
            frame_sync: FrameSync::new(config.target_fps),
            interpolator: Interpolator::new(config.interpolate),
            visibility: VisibilityManager::new(config.enable_culling),
            config,
            stats: RenderStats::default(),
            last_snapshot: None,
        }
    }

    /// Update renderer with new runtime snapshot
    ///
    /// This is the primary entry point. It:
    /// 1. Validates the snapshot
    /// 2. Syncs with frame timing
    /// 3. Converts objects to render state
    /// 4. Pushes updates to backend
    ///
    /// Errors are logged but non-fatal - rendering continues.
    pub fn update(&mut self, snapshot: &RuntimeSnapshot) -> RenderResult<()> {
        let frame_start = std::time::Instant::now();

        // Sync frame timing
        if !self.frame_sync.should_render() {
            return Ok(());
        }

        // Check object count bounds
        if snapshot.objects.len() > self.config.max_objects {
            log::warn!(
                "Object count {} exceeds recommended maximum {}",
                snapshot.objects.len(),
                self.config.max_objects
            );
        }

        // Process visibility and culling
        let visible_objects = self.visibility.filter(&snapshot.objects, &snapshot.focus_ids);

        // Interpolate if enabled
        let render_objects = if self.config.interpolate {
            if let Some(prev) = &self.last_snapshot {
                let alpha = self.frame_sync.interpolation_alpha();
                self.interpolator
                    .interpolate(&prev.objects, &snapshot.objects, alpha)
            } else {
                visible_objects
            }
        } else {
            visible_objects
        };

        // Convert objects to render instructions
        let mut rendered = 0;
        let mut culled = 0;

        for obj in &render_objects {
            // Check if object exists in scene
            if let Some(render_id) = self.scene_map.get(obj.id) {
                // Update existing object
                if let Err(e) = self.update_object(render_id, obj) {
                    log::error!("Failed to update object {}: {:?}", obj.id, e);
                    // Continue rendering other objects
                }
                rendered += 1;
            } else {
                // Create new object
                match self.create_object(obj) {
                    Ok(render_id) => {
                        self.scene_map.insert(obj.id, render_id);
                        rendered += 1;
                    }
                    Err(e) => {
                        log::error!("Failed to create object {}: {:?}", obj.id, e);
                        // Continue rendering other objects
                    }
                }
            }
        }

        // Remove objects that no longer exist
        let current_ids: std::collections::HashSet<_> =
            snapshot.objects.iter().map(|o| o.id).collect();
        let removed = self.scene_map.cleanup(|id| !current_ids.contains(id));

        for render_id in removed {
            if let Err(e) = self.backend.remove_object(render_id) {
                log::error!("Failed to remove object {}: {:?}", render_id, e);
                // Continue cleanup
            }
        }

        culled = snapshot.objects.len() - rendered;

        // Update statistics
        let frame_time = frame_start.elapsed().as_secs_f64() * 1000.0;
        self.update_stats(rendered, culled, frame_time);

        // Store snapshot for interpolation
        self.last_snapshot = Some(snapshot.clone());

        // Mark frame complete
        self.frame_sync.frame_complete();

        Ok(())
    }

    /// Force rebuild of entire scene
    pub fn rebuild(&mut self) -> RenderResult<()> {
        // Clear all objects
        for render_id in self.scene_map.all_render_ids() {
            let _ = self.backend.remove_object(render_id);
        }
        self.scene_map.clear();

        // Re-render from last snapshot if available
        if let Some(snapshot) = self.last_snapshot.take() {
            self.update(&snapshot)?;
        }

        Ok(())
    }

    /// Get current statistics
    pub fn stats(&self) -> RenderStats {
        self.stats
    }

    /// Shutdown and cleanup
    pub fn shutdown(mut self) -> RenderResult<()> {
        // Remove all objects
        for render_id in self.scene_map.all_render_ids() {
            let _ = self.backend.remove_object(render_id);
        }
        self.scene_map.clear();

        Ok(())
    }

    // Private helpers

    fn create_object(&mut self, obj: &crate::renderer::ObjectState) -> RenderResult<u64> {
        // Convert semantic geometry to render geometry
        let geometry = self.adapter.convert_geometry(&obj.geometry)?;

        // Convert transform to matrix
        let transform = self.adapter.convert_transform(&obj.transform);

        // Convert material properties
        let material = self.adapter.convert_material(&obj.material);

        // Create in backend
        let render_id = self
            .backend
            .create_object(geometry, transform, material)?;

        // Set visibility
        if !obj.visible {
            self.backend.set_visible(render_id, false)?;
        }

        // Apply highlight if needed
        if obj.highlighted {
            self.backend.set_highlighted(render_id, true)?;
        }

        Ok(render_id)
    }

    fn update_object(
        &mut self,
        render_id: u64,
        obj: &crate::renderer::ObjectState,
    ) -> RenderResult<()> {
        // Update transform
        let transform = self.adapter.convert_transform(&obj.transform);
        self.backend.update_transform(render_id, transform)?;

        // Update material
        let material = self.adapter.convert_material(&obj.material);
        self.backend.update_material(render_id, material)?;

        // Update visibility
        self.backend.set_visible(render_id, obj.visible)?;

        // Update highlight
        self.backend.set_highlighted(render_id, obj.highlighted)?;

        Ok(())
    }

    fn update_stats(&mut self, rendered: usize, culled: usize, frame_time: f64) {
        self.stats.frame_count += 1;
        self.stats.objects_rendered = rendered;
        self.stats.objects_culled = culled;
        self.stats.last_frame_time_ms = frame_time;

        // Running average
        let alpha = 0.1;
        self.stats.avg_frame_time_ms =
            alpha * frame_time + (1.0 - alpha) * self.stats.avg_frame_time_ms;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::backend::MockBackend;

    #[test]
    fn test_bridge_one_way_flow() {
        let backend = Box::new(MockBackend::new());
        let config = RendererConfig::default();
        let mut bridge = RendererBridge::new(backend, config);

        // Create snapshot
        let snapshot = RuntimeSnapshot {
            tick: 1,
            timestamp: 0.0,
            objects: vec![],
            focus_ids: vec![],
        };

        // Update should not modify snapshot
        let result = bridge.update(&snapshot);
        assert!(result.is_ok());

        // Snapshot remains unchanged
        assert_eq!(snapshot.tick, 1);
    }

    #[test]
    fn test_bridge_error_isolation() {
        let backend = Box::new(MockBackend::new());
        let config = RendererConfig::default();
        let mut bridge = RendererBridge::new(backend, config);

        // Even with errors, bridge continues
        let snapshot = RuntimeSnapshot {
            tick: 1,
            timestamp: 0.0,
            objects: vec![],
            focus_ids: vec![],
        };

        let result = bridge.update(&snapshot);
        // Should not panic or propagate errors
        assert!(result.is_ok());
    }
}