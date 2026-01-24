pub struct DiagnosticEngine {
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
}

pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: ErrorCode,
    pub message: String,
    pub location: SourceLocation,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

impl DiagnosticEngine {
    pub fn emit_error(&mut self, error: impl Into<Diagnostic>) {
        self.errors.push(error.into());
    }
    
    pub fn format_for_human(&self) -> String {
        // Rust-style formatted output with source snippets
    }
    
    pub fn format_for_machine(&self) -> serde_json::Value {
        // LSP-compatible JSON diagnostics
    }
}