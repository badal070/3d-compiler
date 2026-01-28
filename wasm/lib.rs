// wasm/lib.rs
// WASM bridge for web compilation and rendering

use wasm_bindgen::prelude::*;
use serde::Deserialize;

#[wasm_bindgen]
pub struct WasmCompiler {
    runtime_state: Option<RuntimeState>,
    snapshot_builder: SnapshotBuilder,
    current_tick: u64,
}

#[wasm_bindgen]
impl WasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set panic hook for better error messages
        console_error_panic_hook::set_once();
        
        Self {
            runtime_state: None,
            snapshot_builder: SnapshotBuilder::new(),
            current_tick: 0,
        }
    }

    /// Compile DSL source and initialize runtime
    #[wasm_bindgen]
    pub fn compile(&mut self, source: &str) -> Result<JsValue, JsValue> {
        // 1. Compile DSL to IR
        let compiler = dsl_compiler::Compiler::new();
        let ir_scene = compiler
            .compile(source.to_string(), std::path::PathBuf::from("input.dsl"))
            .map_err(|errors: Vec<dsl_compiler::errors::DslError>| {
                let error_msg = errors
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join("\n");
                JsValue::from_str(&error_msg)
            })?;

        // 2. Load IR into runtime
        let runtime_state = runtime::loader::SceneLoader::load_scene(&ir_scene)
            .map_err(|e| JsValue::from_str(&format!("Runtime load error: {}", e)))?;

        self.runtime_state = Some(runtime_state);
        self.current_tick = 0;

        Ok(JsValue::from_str("Compilation successful"))
    }

    /// Step simulation forward by one frame
    #[wasm_bindgen]
    pub fn step(&mut self) -> Result<(), JsValue> {
        if let Some(state) = &mut self.runtime_state {
            // Fixed time step: 1/60 second
            let dt = 1.0 / 60.0;
            
            state.time.advance(dt)
                .map_err(|e| JsValue::from_str(&format!("Time advance error: {}", e)))?;
            
            self.current_tick += 1;
            Ok(())
        } else {
            Err(JsValue::from_str("No scene loaded"))
        }
    }

    /// Get current snapshot for rendering
    #[wasm_bindgen]
    pub fn get_snapshot(&mut self) -> Result<JsValue, JsValue> {
        if let Some(state) = &self.runtime_state {
            let snapshot = self.snapshot_builder.build_snapshot(state);
            
            // Serialize to JSON
            serde_wasm_bindgen::to_value(&snapshot)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        } else {
            Err(JsValue::from_str("No scene loaded"))
        }
    }

    /// Reset simulation to initial state
    #[wasm_bindgen]
    pub fn reset(&mut self) -> Result<(), JsValue> {
        if let Some(state) = &mut self.runtime_state {
            state.time.reset();
            self.current_tick = 0;
            Ok(())
        } else {
            Err(JsValue::from_str("No scene loaded"))
        }
    }

    /// Get current simulation time
    #[wasm_bindgen]
    pub fn get_time(&self) -> f64 {
        self.runtime_state
            .as_ref()
            .map(|s| s.time.current_time)
            .unwrap_or(0.0)
    }
}

// Re-export types needed by WASM
use runtime::snapshot_builder::SnapshotBuilder;
use runtime::state::RuntimeState;