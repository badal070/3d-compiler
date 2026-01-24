/// Schema validation pass.
/// Validates entity components, constraint types, and field schemas.
/// Ensures conformance to known component and constraint schemas.

use crate::ast::*;
use crate::errors::{DslError, ErrorCode, ErrorCollector};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct SchemaValidator {
    file: PathBuf,
    errors: ErrorCollector,
    component_schemas: HashMap<String, ComponentSchema>,
    constraint_schemas: HashMap<String, ConstraintSchema>,
    motion_schemas: HashMap<String, MotionSchema>,
}

/// Schema definition for a component
#[derive(Debug, Clone)]
pub struct ComponentSchema {
    pub name: String,
    pub required_fields: Vec<FieldSchema>,
    pub optional_fields: Vec<FieldSchema>,
}

/// Schema definition for a field
#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub name: String,
    pub field_type: FieldType,
}

/// Field type definitions
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Number,
    String,
    Identifier,
    Vector3,
    Boolean,
}

/// Schema definition for a constraint
#[derive(Debug, Clone)]
pub struct ConstraintSchema {
    pub name: String,
    pub required_fields: Vec<FieldSchema>,
    pub optional_fields: Vec<FieldSchema>,
}

/// Schema definition for a motion
#[derive(Debug, Clone)]
pub struct MotionSchema {
    pub name: String,
    pub required_fields: Vec<FieldSchema>,
    pub optional_fields: Vec<FieldSchema>,
}

impl SchemaValidator {
    pub fn new(file: PathBuf) -> Self {
        let mut validator = Self {
            file,
            errors: ErrorCollector::new(),
            component_schemas: HashMap::new(),
            constraint_schemas: HashMap::new(),
            motion_schemas: HashMap::new(),
        };

        validator.init_default_schemas();
        validator
    }

    pub fn validate(mut self, ast: &AstFile) -> Result<(), Vec<DslError>> {
        self.validate_entities(&ast.entities);
        self.validate_constraints(&ast.constraints);
        self.validate_motions(&ast.motions);

        self.errors.into_result(())
    }

    fn init_default_schemas(&mut self) {
        // Register default component schemas
        self.register_component_schema(ComponentSchema {
            name: "transform".to_string(),
            required_fields: vec![
                FieldSchema {
                    name: "position".to_string(),
                    field_type: FieldType::Vector3,
                },
                FieldSchema {
                    name: "rotation".to_string(),
                    field_type: FieldType::Vector3,
                },
                FieldSchema {
                    name: "scale".to_string(),
                    field_type: FieldType::Vector3,
                },
            ],
            optional_fields: vec![],
        });

        self.register_component_schema(ComponentSchema {
            name: "geometry".to_string(),
            required_fields: vec![FieldSchema {
                name: "primitive".to_string(),
                field_type: FieldType::Identifier,
            }],
            optional_fields: vec![],
        });

        self.register_component_schema(ComponentSchema {
            name: "physical".to_string(),
            required_fields: vec![
                FieldSchema {
                    name: "mass".to_string(),
                    field_type: FieldType::Number,
                },
                FieldSchema {
                    name: "rigid".to_string(),
                    field_type: FieldType::Boolean,
                },
            ],
            optional_fields: vec![],
        });

        // Register default constraint schemas
        self.register_constraint_schema(ConstraintSchema {
            name: "gear_relation".to_string(),
            required_fields: vec![
                FieldSchema {
                    name: "type".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "driver".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "driven".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "ratio".to_string(),
                    field_type: FieldType::Number,
                },
            ],
            optional_fields: vec![],
        });

        self.register_constraint_schema(ConstraintSchema {
            name: "fixed_joint".to_string(),
            required_fields: vec![
                FieldSchema {
                    name: "type".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "parent".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "child".to_string(),
                    field_type: FieldType::Identifier,
                },
            ],
            optional_fields: vec![],
        });

        // Register default motion schemas
        self.register_motion_schema(MotionSchema {
            name: "rotation".to_string(),
            required_fields: vec![
                FieldSchema {
                    name: "target".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "type".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "axis".to_string(),
                    field_type: FieldType::Vector3,
                },
                FieldSchema {
                    name: "speed".to_string(),
                    field_type: FieldType::Number,
                },
            ],
            optional_fields: vec![],
        });

        self.register_motion_schema(MotionSchema {
            name: "translation".to_string(),
            required_fields: vec![
                FieldSchema {
                    name: "target".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "type".to_string(),
                    field_type: FieldType::Identifier,
                },
                FieldSchema {
                    name: "direction".to_string(),
                    field_type: FieldType::Vector3,
                },
                FieldSchema {
                    name: "speed".to_string(),
                    field_type: FieldType::Number,
                },
            ],
            optional_fields: vec![],
        });
    }

    fn register_component_schema(&mut self, schema: ComponentSchema) {
        self.component_schemas.insert(schema.name.clone(), schema);
    }

    fn register_constraint_schema(&mut self, schema: ConstraintSchema) {
        self.constraint_schemas.insert(schema.name.clone(), schema);
    }

    fn register_motion_schema(&mut self, schema: MotionSchema) {
        self.motion_schemas.insert(schema.name.clone(), schema);
    }

    fn validate_entities(&mut self, entities: &[AstEntity]) {
        for entity in entities {
            // Validate entity kind
            let valid_kinds = ["solid", "light", "camera", "particle_system"];
            if !valid_kinds.contains(&entity.kind.as_str()) {
                self.errors.add(
                    DslError::new(
                        ErrorCode::InvalidKind,
                        format!("Unknown entity kind: '{}'", entity.kind),
                        entity.span,
                        self.file.clone(),
                    )
                    .with_help(format!("Valid kinds: {}", valid_kinds.join(", "))),
                );
            }

            // Validate components
            for component in &entity.components {
                self.validate_component(component, &entity.name);
            }
        }
    }

    fn validate_component(&mut self, component: &AstComponent, entity_name: &str) {
        let schema = self.component_schemas.get(&component.name);

        if schema.is_none() {
            self.errors.add(DslError::new(
                ErrorCode::UnknownComponentType,
                format!("Unknown component type: '{}'", component.name),
                component.span,
                self.file.clone(),
            ));
            return;
        }

        let schema = schema.unwrap();

        // Check required fields
        for required in &schema.required_fields {
            if !component.fields.iter().any(|f| f.name == required.name) {
                self.errors.add(
                    DslError::new(
                        ErrorCode::MissingRequiredField,
                        format!(
                            "Missing required field '{}' in component '{}' of entity '{}'",
                            required.name, component.name, entity_name
                        ),
                        component.span,
                        self.file.clone(),
                    )
                    .with_help(format!("Add field '{}' to the component", required.name)),
                );
            }
        }

        // Validate field types
        for field in &component.fields {
            let field_schema = schema
                .required_fields
                .iter()
                .chain(schema.optional_fields.iter())
                .find(|f| f.name == field.name);

            if let Some(field_schema) = field_schema {
                self.validate_field_type(&field.value, &field_schema.field_type, &field.name, component.span);
            }
        }
    }

    fn validate_constraints(&mut self, constraints: &[AstConstraint]) {
        for constraint in constraints {
            let constraint_type = constraint.constraint_type();
            
            if constraint_type.is_none() {
                continue; // Already caught by syntax validator
            }

            let constraint_type = constraint_type.unwrap();
            let schema = self.constraint_schemas.get(constraint_type);

            if schema.is_none() {
                let type_field = constraint.get_field("type").unwrap();
                self.errors.add(DslError::new(
                    ErrorCode::UndefinedConstraintType,
                    format!("Unknown constraint type: '{}'", constraint_type),
                    type_field.span,
                    self.file.clone(),
                ));
                continue;
            }

            let schema = schema.unwrap();

            // Check required fields
            for required in &schema.required_fields {
                if !constraint.fields.iter().any(|f| f.name == required.name) {
                    self.errors.add(
                        DslError::new(
                            ErrorCode::MissingRequiredField,
                            format!(
                                "Missing required field '{}' in constraint '{}' of type '{}'",
                                required.name, constraint.name, constraint_type
                            ),
                            constraint.span,
                            self.file.clone(),
                        )
                        .with_help(format!("Add field '{}' to the constraint", required.name)),
                    );
                }
            }

            // Validate field types
            for field in &constraint.fields {
                let field_schema = schema
                    .required_fields
                    .iter()
                    .chain(schema.optional_fields.iter())
                    .find(|f| f.name == field.name);

                if let Some(field_schema) = field_schema {
                    self.validate_field_type(&field.value, &field_schema.field_type, &field.name, constraint.span);
                }
            }
        }
    }

    fn validate_motions(&mut self, motions: &[AstMotion]) {
        for motion in motions {
            let motion_type = motion.motion_type();
            
            if motion_type.is_none() {
                continue; // Already caught by syntax validator
            }

            let motion_type = motion_type.unwrap();
            let schema = self.motion_schemas.get(motion_type);

            if schema.is_none() {
                let type_field = motion.get_field("type").unwrap();
                self.errors.add(DslError::new(
                    ErrorCode::UnknownLibraryConstruct,
                    format!("Unknown motion type: '{}'", motion_type),
                    type_field.span,
                    self.file.clone(),
                ));
                continue;
            }

            let schema = schema.unwrap();

            // Check required fields
            for required in &schema.required_fields {
                if !motion.fields.iter().any(|f| f.name == required.name) {
                    self.errors.add(
                        DslError::new(
                            ErrorCode::MissingRequiredField,
                            format!(
                                "Missing required field '{}' in motion '{}' of type '{}'",
                                required.name, motion.name, motion_type
                            ),
                            motion.span,
                            self.file.clone(),
                        )
                        .with_help(format!("Add field '{}' to the motion", required.name)),
                    );
                }
            }

            // Validate field types
            for field in &motion.fields {
                let field_schema = schema
                    .required_fields
                    .iter()
                    .chain(schema.optional_fields.iter())
                    .find(|f| f.name == field.name);

                if let Some(field_schema) = field_schema {
                    self.validate_field_type(&field.value, &field_schema.field_type, &field.name, motion.span);
                }
            }
        }
    }

    fn validate_field_type(
        &mut self,
        value: &AstValue,
        expected_type: &FieldType,
        field_name: &str,
        context_span: SourceSpan,
    ) {
        let actual_type = match value {
            AstValue::Number(_, _) => FieldType::Number,
            AstValue::String(_, _) => FieldType::String,
            AstValue::Identifier(id, _) => {
                if id == "true" || id == "false" {
                    FieldType::Boolean
                } else {
                    FieldType::Identifier
                }
            }
            AstValue::Vector(vec, _) => {
                if vec.len() == 3 {
                    FieldType::Vector3
                } else {
                    // Invalid vector length, already caught by syntax validator
                    return;
                }
            }
        };

        if &actual_type != expected_type {
            self.errors.add(
                DslError::new(
                    ErrorCode::InvalidFieldType,
                    format!(
                        "Invalid type for field '{}': expected {:?}, found {:?}",
                        field_name, expected_type, actual_type
                    ),
                    value.span(),
                    self.file.clone(),
                )
                .with_help(format!("Field '{}' must be of type {:?}", field_name, expected_type)),
            );
        }

        // Additional validations
        if expected_type == &FieldType::Number {
            if let AstValue::Number(n, span) = value {
                if !n.is_finite() {
                    self.errors.add(DslError::new(
                        ErrorCode::InvalidNumber,
                        format!("Field '{}' must be a finite number", field_name),
                        *span,
                        self.file.clone(),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_validation() {
        assert_eq!(FieldType::Number, FieldType::Number);
        assert_ne!(FieldType::Number, FieldType::String);
    }

    #[test]
    fn test_schema_registration() {
        let validator = SchemaValidator::new(PathBuf::from("test.dsl"));
        
        assert!(validator.component_schemas.contains_key("transform"));
        assert!(validator.component_schemas.contains_key("geometry"));
        assert!(validator.component_schemas.contains_key("physical"));
        
        assert!(validator.constraint_schemas.contains_key("gear_relation"));
        assert!(validator.motion_schemas.contains_key("rotation"));
    }
}