// compiler/runtime/loader.rs
/// Loads IR into executable runtime state
use crate::ir::{IrScene, IrEntity, IrValue, IrComponent, IrMotion, IrTimeline};
use crate::state::{
    RuntimeState, WorldState, ObjectState, ObjectKind, 
    Vector3, Quaternion, TimeState, ParameterState, Parameter
};
use crate::error::{RuntimeError, RuntimeResult};
use std::collections::HashMap;

pub struct SceneLoader;

impl SceneLoader {
    /// Load complete IR scene into runtime state
    pub fn load_scene(ir: &IrScene) -> RuntimeResult<RuntimeState> {
        let mut world = WorldState::new();
        let time = TimeState::new();
        
        // Load entities
        for entity in &ir.entities {
            let object = Self::load_entity(entity)?;
            world.add_object(entity.id.clone(), object)?;
        }
        
        // Load constraints (TODO: implement when needed)
        // for constraint in &ir.constraints { ... }
        
        // Load parameters from metadata
        let mut params = ParameterState::new();
        if let Some(time_param) = Self::create_time_parameter() {
            params.add(time_param)?;
        }
        world.parameters = params;
        
        Ok(RuntimeState::new(world, time))
    }
    
    fn load_entity(entity: &IrEntity) -> RuntimeResult<ObjectState> {
        // Determine object kind
        let kind = match entity.kind.as_str() {
            "solid" => Self::infer_kind_from_geometry(entity),
            _ => ObjectKind::Custom,
        };
        
        let mut obj = ObjectState::new(entity.id.clone(), kind);
        
        // Load transform component
        if let Some(transform) = entity.components.get("transform") {
            obj = Self::load_transform(obj, transform)?;
        }
        
        // Load physical component
        if let Some(physical) = entity.components.get("physical") {
            obj = Self::load_physical(obj, physical)?;
        }
        
        obj.visible = true;
        Ok(obj)
    }
    
    fn infer_kind_from_geometry(entity: &IrEntity) -> ObjectKind {
        if let Some(geom) = entity.components.get("geometry") {
            if let Some(IrValue::Identifier(prim)) = geom.properties.get("primitive") {
                return match prim.as_str() {
                    "cube" => ObjectKind::Box,
                    "sphere" => ObjectKind::Sphere,
                    "cylinder" => ObjectKind::Cylinder,
                    "plane" => ObjectKind::Plane,
                    _ => ObjectKind::Custom,
                };
            }
        }
        ObjectKind::Custom
    }
    
    fn load_transform(
        mut obj: ObjectState, 
        transform: &IrComponent
    ) -> RuntimeResult<ObjectState> {
        // Load position
        if let Some(IrValue::Vector3(pos)) = transform.properties.get("position") {
            obj.position = Vector3::new(pos[0], pos[1], pos[2]);
        }
        
        // Load rotation (Euler angles in radians -> Quaternion)
        if let Some(IrValue::Vector3(rot)) = transform.properties.get("rotation") {
            obj.orientation = Self::euler_to_quaternion(rot[0], rot[1], rot[2]);
        }
        
        // Load scale
        if let Some(IrValue::Vector3(scale)) = transform.properties.get("scale") {
            obj.scale = Vector3::new(scale[0], scale[1], scale[2]);
        }
        
        Ok(obj)
    }
    
    fn load_physical(
        mut obj: ObjectState,
        physical: &IrComponent
    ) -> RuntimeResult<ObjectState> {
        // Check if rigid/static
        if let Some(IrValue::Boolean(rigid)) = physical.properties.get("rigid") {
            if *rigid {
                obj = obj.make_static();
            }
        }
        
        Ok(obj)
    }
    
    fn euler_to_quaternion(roll: f64, pitch: f64, yaw: f64) -> Quaternion {
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        
        Quaternion::new(
            cr * cp * cy + sr * sp * sy,  // w
            sr * cp * cy - cr * sp * sy,  // x
            cr * sp * cy + sr * cp * sy,  // y
            cr * cp * sy - sr * sp * cy,  // z
        )
    }
    
    fn create_time_parameter() -> Option<Parameter> {
        Some(Parameter::new("time".to_string(), 0.0)
            .with_kind(crate::state::ParameterKind::Time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_simple_entity() {
        // Test loading a basic cube entity
    }
}