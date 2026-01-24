/// Abstract Syntax Tree definitions.
/// Mirrors DSL structure exactly - no semantic interpretation.
/// Preserves source spans for excellent error reporting.

use crate::errors::SourceSpan;

/// Complete DSL file representation
#[derive(Debug, Clone)]
pub struct AstFile {
    pub scene: AstScene,
    pub library_imports: AstLibraryImports,
    pub entities: Vec<AstEntity>,
    pub constraints: Vec<AstConstraint>,
    pub motions: Vec<AstMotion>,
    pub timelines: Vec<AstTimeline>,
    pub span: SourceSpan,
}

/// Scene header
#[derive(Debug, Clone)]
pub struct AstScene {
    pub name: String,
    pub version: i64,
    pub ir_version: String,
    pub unit_system: String,
    pub span: SourceSpan,
}

/// Library imports section
#[derive(Debug, Clone)]
pub struct AstLibraryImports {
    pub imports: Vec<AstImport>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub struct AstImport {
    pub alias: String,
    pub library_name: String,
    pub span: SourceSpan,
}

/// Entity definition
#[derive(Debug, Clone)]
pub struct AstEntity {
    pub name: String,
    pub kind: String,
    pub components: Vec<AstComponent>,
    pub span: SourceSpan,
}

/// Component within an entity
#[derive(Debug, Clone)]
pub struct AstComponent {
    pub name: String,
    pub fields: Vec<AstField>,
    pub span: SourceSpan,
}

/// Field within a component or other block
#[derive(Debug, Clone)]
pub struct AstField {
    pub name: String,
    pub value: AstValue,
    pub span: SourceSpan,
}

/// Value types
#[derive(Debug, Clone)]
pub enum AstValue {
    Number(f64, SourceSpan),
    String(String, SourceSpan),
    Identifier(String, SourceSpan),
    Vector(Vec<f64>, SourceSpan),
}

impl AstValue {
    pub fn span(&self) -> SourceSpan {
        match self {
            AstValue::Number(_, span)
            | AstValue::String(_, span)
            | AstValue::Identifier(_, span)
            | AstValue::Vector(_, span) => *span,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            AstValue::Number(n, _) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            AstValue::String(s, _) => Some(s),
            _ => None,
        }
    }

    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            AstValue::Identifier(id, _) => Some(id),
            _ => None,
        }
    }

    pub fn as_vector(&self) -> Option<&[f64]> {
        match self {
            AstValue::Vector(vec, _) => Some(vec),
            _ => None,
        }
    }
}

/// Constraint definition
#[derive(Debug, Clone)]
pub struct AstConstraint {
    pub name: String,
    pub fields: Vec<AstField>,
    pub span: SourceSpan,
}

impl AstConstraint {
    pub fn get_field(&self, name: &str) -> Option<&AstField> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn constraint_type(&self) -> Option<&str> {
        self.get_field("type")
            .and_then(|f| f.value.as_identifier())
    }
}

/// Motion definition
#[derive(Debug, Clone)]
pub struct AstMotion {
    pub name: String,
    pub fields: Vec<AstField>,
    pub span: SourceSpan,
}

impl AstMotion {
    pub fn get_field(&self, name: &str) -> Option<&AstField> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn target(&self) -> Option<&str> {
        self.get_field("target")
            .and_then(|f| f.value.as_identifier())
    }

    pub fn motion_type(&self) -> Option<&str> {
        self.get_field("type")
            .and_then(|f| f.value.as_identifier())
    }
}

/// Timeline definition
#[derive(Debug, Clone)]
pub struct AstTimeline {
    pub name: String,
    pub events: Vec<AstEvent>,
    pub span: SourceSpan,
}

/// Event within a timeline
#[derive(Debug, Clone)]
pub struct AstEvent {
    pub fields: Vec<AstField>,
    pub span: SourceSpan,
}

impl AstEvent {
    pub fn get_field(&self, name: &str) -> Option<&AstField> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn motion(&self) -> Option<&str> {
        self.get_field("motion")
            .and_then(|f| f.value.as_identifier())
    }

    pub fn start(&self) -> Option<f64> {
        self.get_field("start")
            .and_then(|f| f.value.as_number())
    }

    pub fn duration(&self) -> Option<f64> {
        self.get_field("duration")
            .and_then(|f| f.value.as_number())
    }
}

/// Helper trait for field lookup
pub trait HasFields {
    fn get_field(&self, name: &str) -> Option<&AstField>;
    
    fn get_string_field(&self, name: &str) -> Option<&str> {
        self.get_field(name)
            .and_then(|f| f.value.as_string())
    }
    
    fn get_number_field(&self, name: &str) -> Option<f64> {
        self.get_field(name)
            .and_then(|f| f.value.as_number())
    }
    
    fn get_identifier_field(&self, name: &str) -> Option<&str> {
        self.get_field(name)
            .and_then(|f| f.value.as_identifier())
    }
    
    fn get_vector_field(&self, name: &str) -> Option<&[f64]> {
        self.get_field(name)
            .and_then(|f| f.value.as_vector())
    }
}

impl HasFields for AstComponent {
    fn get_field(&self, name: &str) -> Option<&AstField> {
        self.fields.iter().find(|f| f.name == name)
    }
}

impl HasFields for AstConstraint {
    fn get_field(&self, name: &str) -> Option<&AstField> {
        self.fields.iter().find(|f| f.name == name)
    }
}

impl HasFields for AstMotion {
    fn get_field(&self, name: &str) -> Option<&AstField> {
        self.fields.iter().find(|f| f.name == name)
    }
}

impl HasFields for AstEvent {
    fn get_field(&self, name: &str) -> Option<&AstField> {
        self.fields.iter().find(|f| f.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_accessors() {
        let span = SourceSpan::single_point(1, 1, 0);
        
        let num_val = AstValue::Number(42.0, span);
        assert_eq!(num_val.as_number(), Some(42.0));
        assert_eq!(num_val.as_string(), None);
        
        let str_val = AstValue::String("test".to_string(), span);
        assert_eq!(str_val.as_string(), Some("test"));
        assert_eq!(str_val.as_number(), None);
        
        let id_val = AstValue::Identifier("cube".to_string(), span);
        assert_eq!(id_val.as_identifier(), Some("cube"));
        
        let vec_val = AstValue::Vector(vec![1.0, 2.0, 3.0], span);
        assert_eq!(vec_val.as_vector(), Some(&[1.0, 2.0, 3.0][..]));
    }

    #[test]
    fn test_field_lookup() {
        let span = SourceSpan::single_point(1, 1, 0);
        
        let component = AstComponent {
            name: "transform".to_string(),
            fields: vec![
                AstField {
                    name: "position".to_string(),
                    value: AstValue::Vector(vec![0.0, 0.0, 0.0], span),
                    span,
                },
                AstField {
                    name: "mass".to_string(),
                    value: AstValue::Number(1.0, span),
                    span,
                },
            ],
            span,
        };
        
        assert!(component.get_field("position").is_some());
        assert!(component.get_field("mass").is_some());
        assert!(component.get_field("nonexistent").is_none());
        
        assert_eq!(component.get_vector_field("position"), Some(&[0.0, 0.0, 0.0][..]));
        assert_eq!(component.get_number_field("mass"), Some(1.0));
    }

    #[test]
    fn test_motion_helpers() {
        let span = SourceSpan::single_point(1, 1, 0);
        
        let motion = AstMotion {
            name: "spin".to_string(),
            fields: vec![
                AstField {
                    name: "target".to_string(),
                    value: AstValue::Identifier("cube1".to_string(), span),
                    span,
                },
                AstField {
                    name: "type".to_string(),
                    value: AstValue::Identifier("rotation".to_string(), span),
                    span,
                },
            ],
            span,
        };
        
        assert_eq!(motion.target(), Some("cube1"));
        assert_eq!(motion.motion_type(), Some("rotation"));
    }

    #[test]
    fn test_event_helpers() {
        let span = SourceSpan::single_point(1, 1, 0);
        
        let event = AstEvent {
            fields: vec![
                AstField {
                    name: "motion".to_string(),
                    value: AstValue::Identifier("spin".to_string(), span),
                    span,
                },
                AstField {
                    name: "start".to_string(),
                    value: AstValue::Number(0.0, span),
                    span,
                },
                AstField {
                    name: "duration".to_string(),
                    value: AstValue::Number(2.0, span),
                    span,
                },
            ],
            span,
        };
        
        assert_eq!(event.motion(), Some("spin"));
        assert_eq!(event.start(), Some(0.0));
        assert_eq!(event.duration(), Some(2.0));
    }
}