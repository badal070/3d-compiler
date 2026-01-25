// wasm/lib.rs
use wasm_bindgen::prelude::*;
use dsl_compiler::{compile_source, Compiler};
use compiler::runtime::{SceneLoader, SnapshotBuilder, RuntimeEngine, RuntimeConfig};
use compiler::renderer::{Renderer, RendererConfig};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct WasmCompiler {
    runtime: Option<RuntimeEngine>,
    tick: u64,
}

#[wasm_bindgen]
impl WasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        console_log!("WASM Compiler initialized");
        
        Self {
            runtime: None,
            tick: 0,
        }
    }
    
    /// Compile DSL source and initialize runtime
    #[wasm_bindgen]
    pub fn compile(&mut self, dsl_source: &str) -> Result<JsValue, JsValue> {
        console_log!("Compiling DSL...");
        
        // Compile DSL to IR
        let ir = compile_source(dsl_source)
            .map_err(|e| JsValue::from_str(&format!("Compilation failed: {:?}", e)))?;
        
        console_log!("DSL compiled successfully");
        
        // Load IR into runtime
        let state = SceneLoader::load_scene(&ir)
            .map_err(|e| JsValue::from_str(&format!("Scene load failed: {:?}", e)))?;
        
        console_log!("Scene loaded into runtime");
        
        // Create runtime engine
        let mut runtime = RuntimeEngine::new(RuntimeConfig::default());
        runtime.initialize(state, compiler::runtime::ExecutionPlan::standard())
            .map_err(|e| JsValue::from_str(&format!("Runtime init failed: {:?}", e)))?;
        
        self.runtime = Some(runtime);
        self.tick = 0;
        
        Ok(JsValue::from_str("Compilation successful"))
    }
    
    /// Get current scene state as JSON for renderer
    #[wasm_bindgen]
    pub fn get_snapshot(&mut self) -> Result<JsValue, JsValue> {
        let runtime = self.runtime.as_ref()
            .ok_or_else(|| JsValue::from_str("No scene loaded"))?;
        
        let state = runtime.state();
        let snapshot = SnapshotBuilder::build(state, self.tick);
        
        // Convert to JS object
        let js_snapshot = js_sys::Object::new();
        
        // Add tick
        js_sys::Reflect::set(
            &js_snapshot,
            &JsValue::from_str("tick"),
            &JsValue::from_f64(self.tick as f64),
        )?;
        
        // Add timestamp
        js_sys::Reflect::set(
            &js_snapshot,
            &JsValue::from_str("timestamp"),
            &JsValue::from_f64(snapshot.timestamp),
        )?;
        
        // Add objects array
        let objects_array = js_sys::Array::new();
        for obj in &snapshot.objects {
            let js_obj = self.object_to_js(obj)?;
            objects_array.push(&js_obj);
        }
        
        js_sys::Reflect::set(
            &js_snapshot,
            &JsValue::from_str("objects"),
            &objects_array,
        )?;
        
        Ok(js_snapshot.into())
    }
    
    /// Step runtime forward one frame
    #[wasm_bindgen]
    pub fn step(&mut self) -> Result<(), JsValue> {
        let runtime = self.runtime.as_mut()
            .ok_or_else(|| JsValue::from_str("No scene loaded"))?;
        
        runtime.execute_single_step()
            .map_err(|e| JsValue::from_str(&format!("Step failed: {:?}", e)))?;
        
        self.tick += 1;
        Ok(())
    }
    
    fn object_to_js(&self, obj: &compiler::renderer::ObjectState) -> Result<JsValue, JsValue> {
        let js_obj = js_sys::Object::new();
        
        js_sys::Reflect::set(&js_obj, &"id".into(), &JsValue::from_f64(obj.id as f64))?;
        js_sys::Reflect::set(&js_obj, &"visible".into(), &JsValue::from_bool(obj.visible))?;
        
        // Geometry
        let geom_str = match &obj.geometry {
            compiler::renderer::GeometryType::Sphere { radius } => {
                format!("{{\"type\":\"sphere\",\"radius\":{}}}", radius)
            }
            compiler::renderer::GeometryType::Box { width, height, depth } => {
                format!("{{\"type\":\"box\",\"width\":{},\"height\":{},\"depth\":{}}}", 
                    width, height, depth)
            }
            _ => "{\"type\":\"box\",\"width\":1,\"height\":1,\"depth\":1}".to_string(),
        };
        js_sys::Reflect::set(&js_obj, &"geometry".into(), &JsValue::from_str(&geom_str))?;
        
        // Transform
        let pos = obj.transform.position;
        let rot = obj.transform.rotation;
        let scale = obj.transform.scale;
        
        let transform_str = format!(
            "{{\"position\":[{},{},{}],\"rotation\":[{},{},{},{}],\"scale\":[{},{},{}]}}",
            pos[0], pos[1], pos[2],
            rot[0], rot[1], rot[2], rot[3],
            scale[0], scale[1], scale[2]
        );
        js_sys::Reflect::set(&js_obj, &"transform".into(), &JsValue::from_str(&transform_str))?;
        
        Ok(js_obj.into())
    }
}

// Add panic hook for better error messages
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}