# DSL Compiler Architecture

## Overview

This document describes the internal architecture of the DSL compiler. The compiler transforms DSL source code into validated intermediate representation through a series of ordered passes.

## Compilation Pipeline

```
┌─────────────┐
│ Source Text │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Lexer     │ → Tokens
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Parser    │ → AST (Abstract Syntax Tree)
└──────┬──────┘
       │
       ▼
┌─────────────────────────────┐
│   Validation Pipeline       │
│  ┌────────────────────────┐ │
│  │ 1. Syntax Validation   │ │
│  └────────────────────────┘ │
│  ┌────────────────────────┐ │
│  │ 2. Schema Validation   │ │
│  └────────────────────────┘ │
│  ┌────────────────────────┐ │
│  │ 3. Reference Resolution│ │
│  └────────────────────────┘ │
│  ┌────────────────────────┐ │
│  │ 4. Unit Validation     │ │
│  └────────────────────────┘ │
│  ┌────────────────────────┐ │
│  │ 5. Library Check       │ │
│  └────────────────────────┘ │
└──────────┬──────────────────┘
           │
           ▼
    ┌─────────────┐
    │ IR Lowering │ → Intermediate Representation
    └─────────────┘
```

## Module Responsibilities

### `errors.rs`
**Purpose:** Central error handling system

**Responsibilities:**
- Define error codes (E001-E599)
- Source span tracking
- Error formatting with context
- Error collection for batch reporting

**Key Types:**
- `ErrorCode` - Machine-readable error categories
- `DslError` - Complete error with location and message
- `SourceSpan` - Precise source location tracking
- `ErrorCollector` - Accumulate multiple errors

**Design Principles:**
- Every error must be actionable
- Errors include source context
- Machine-readable codes for tooling
- Human-readable messages for developers

### `lexer.rs`
**Purpose:** Tokenize source text

**Responsibilities:**
- Character-by-character scanning
- Token recognition (keywords, identifiers, literals)
- Comment handling
- Position tracking

**Key Types:**
- `Token` - A single lexical unit
- `TokenKind` - Type of token
- `Lexer` - Tokenization state machine

**Design Principles:**
- Single-pass tokenization
- Preserve source locations
- Fail on first lexical error
- Support single-line comments (//)

### `ast.rs`
**Purpose:** Abstract Syntax Tree definitions

**Responsibilities:**
- Mirror DSL structure exactly
- Preserve all source information
- Provide accessor helpers
- No semantic interpretation

**Key Types:**
- `AstFile` - Complete DSL file
- `AstScene` - Scene metadata
- `AstEntity` - Entity definition
- `AstComponent` - Component within entity
- `AstConstraint` - Constraint definition
- `AstMotion` - Motion definition
- `AstTimeline` - Timeline with events
- `AstValue` - Value types (number, string, identifier, vector)

**Design Principles:**
- 1:1 mapping to DSL syntax
- Preserve source spans on every node
- No validation at this layer
- Helper methods for common queries

### `parser.rs`
**Purpose:** Build AST from token stream

**Responsibilities:**
- Enforce mandatory section ordering
- Build AST nodes
- Track source spans
- Syntax error reporting

**Key Types:**
- `Parser` - Parsing state and logic

**Design Principles:**
- Recursive descent parsing
- Fail on first syntax error
- Preserve all source information
- No backtracking

**Grammar Enforcement:**
```
file := scene library_imports entity* constraint* motion* timeline*
```
Order is **mandatory** and **not reorderable**.

### `validator/syntax.rs`
**Purpose:** Pass 1 - Structural validation

**Responsibilities:**
- Check required fields present
- Detect duplicate names
- Validate value ranges
- Check vector lengths
- Verify version formats

**Validations:**
- Scene version ≥ 1
- IR version is valid semver
- Unit system in {SI, Imperial}
- Entity names unique
- Component types unique within entity
- Constraint names unique
- Motion names unique
- Timeline names unique
- Vectors have exactly 3 components
- No NaN or Infinity values

**Design Principles:**
- Pure validation (no mutation)
- Fail-fast on critical errors
- Collect multiple errors when safe
- Clear error messages

### `validator/schema.rs`
**Purpose:** Pass 2 - Schema conformance

**Responsibilities:**
- Validate component schemas
- Check constraint type schemas
- Validate motion type schemas
- Enforce field types
- Check required fields

**Schema Registry:**
- Transform component schema
- Geometry component schema
- Physical component schema
- Gear relation constraint schema
- Fixed joint constraint schema
- Rotation motion schema
- Translation motion schema

**Design Principles:**
- Extensible schema system
- Type-safe field validation
- Helpful error messages
- Suggest correct types

### `validator/references.rs`
**Purpose:** Pass 3 - Reference resolution

**Responsibilities:**
- Resolve entity references
- Resolve motion references
- Detect undefined references
- Detect circular dependencies
- Check timeline event overlaps

**Algorithms:**
- Build symbol tables
- DFS for cycle detection
- Interval overlap detection

**Design Principles:**
- Complete symbol tables first
- Then validate references
- Detect all undefined refs
- Suggest fixes when possible

### `validator/units.rs`
**Purpose:** Pass 4 - Physical unit validation

**Responsibilities:**
- Validate rotation units (radians)
- Check mass values (positive, reasonable)
- Validate vector magnitudes
- Warn on suspicious values
- Normalize axes

**Unit Systems:**
- SI: meters, kilograms, seconds, radians
- Imperial: feet, pounds, seconds, radians

**Validations:**
- Rotation in radians (warn if >100)
- Mass > 0
- Mass in reasonable range
- Speed finite
- Axis normalized for rotation

**Design Principles:**
- Unit system aware
- Helpful warnings
- Suggest conversions
- Don't block on warnings

### `validator/library.rs`
**Purpose:** Pass 5 - Library compatibility

**Responsibilities:**
- Validate imported libraries exist
- Check construct availability
- Suggest missing imports
- Version compatibility

**Default Libraries:**
1. `core_mechanics` - transform, physical, rotations
2. `basic_solids` - geometry primitives
3. `gear_systems` - gear constraints
4. `advanced_physics` - collisions, springs

**Design Principles:**
- Explicit dependency graph
- No implicit imports
- Extensible library system
- Helpful suggestions

### `lower_to_ir.rs`
**Purpose:** Transform validated AST to IR

**Responsibilities:**
- 1:1 DSL→IR mapping
- Flatten structure
- Resolve identifiers to IDs
- Normalize values

**IR Structure:**
- `IrScene` - Complete scene
- `IrMetadata` - Scene metadata
- `IrEntity` - Entity with ID
- `IrComponent` - Component with properties
- `IrConstraint` - Constraint with parameters
- `IrMotion` - Motion with target ID
- `IrTimeline` - Timeline with events
- `IrValue` - Normalized value types

**Design Principles:**
- Pure function (AST → IR)
- No validation here
- Preserve semantics exactly
- JSON serializable output

### `lib.rs`
**Purpose:** Public API and orchestration

**Responsibilities:**
- Compiler orchestration
- Pipeline execution
- Public interface
- Convenience functions

**Public API:**
```rust
pub struct Compiler { /* ... */ }

impl Compiler {
    pub fn new() -> Self;
    pub fn compile(&self, source: String, file: PathBuf) 
        -> Result<IrScene, Vec<DslError>>;
    pub fn parse_only(&self, source: String, file: PathBuf) 
        -> DslResult<AstFile>;
    pub fn validate_only(&self, ast: &AstFile, file: &PathBuf) 
        -> Result<(), Vec<DslError>>;
}

pub fn compile_file(path: PathBuf) -> Result<IrScene, Vec<DslError>>;
pub fn compile_source(source: String) -> Result<IrScene, Vec<DslError>>;
```

## Data Flow

### 1. Lexing Phase
```
"scene { name: \"Test\" }"
         ↓
[Token::Scene, Token::LeftBrace, Token::Identifier("name"), 
 Token::Colon, Token::String("Test"), Token::RightBrace, Token::Eof]
```

### 2. Parsing Phase
```
Tokens
  ↓
AstFile {
  scene: AstScene { name: "Test", ... },
  library_imports: AstLibraryImports { imports: [...] },
  entities: [...],
  ...
}
```

### 3. Validation Phases
```
AST → [Syntax Check] → [Schema Check] → [Reference Check] 
    → [Unit Check] → [Library Check] → Validated AST
```

Each validator:
- Takes `&AstFile` (immutable)
- Returns `Result<(), Vec<DslError>>`
- Is completely independent
- Can run in isolation for testing

### 4. Lowering Phase
```
Validated AST
     ↓
IrScene {
  metadata: { name: "Test", ... },
  entities: [
    IrEntity {
      id: "cube1",
      components: {
        "transform": IrComponent { ... }
      }
    }
  ],
  ...
}
```

## Error Handling Strategy

### Error Codes
- **E001-E099:** Lexical (unexpected character, unterminated string)
- **E100-E199:** Syntax (unexpected token, missing field)
- **E200-E299:** Schema (unknown component, invalid type)
- **E300-E399:** Reference (undefined entity, circular dependency)
- **E400-E499:** Unit (invalid mass, wrong units)
- **E500-E599:** Library (missing import, incompatible version)

### Error Flow
```
Lexer Error → Immediate Return
Parser Error → Immediate Return
Validator Error → Collect All → Return Batch
Lowering Error → Should Never Happen (validated AST)
```

### Error Quality
Each error must include:
1. Error code (machine-readable)
2. Message (human-readable)
3. Source span (line, column, offset)
4. File path
5. Help text (optional, encouraged)

Example:
```
E300: Undefined entity 'cube2'
 --> scene.dsl:42:12
  |
42|   target: cube2
  |           ^^^^^ Reference Error
  |
help: Entity 'cube2' must be defined before it can be referenced
```

## Testing Strategy

### Unit Tests
Each module has inline tests:
- `lexer.rs` - Token recognition
- `parser.rs` - AST construction
- `ast.rs` - Helper methods
- `validator/*.rs` - Validation rules
- `lower_to_ir.rs` - IR transformation

### Integration Tests
- Complete compilation pipeline
- Error reporting accuracy
- Multi-file scenarios
- Version compatibility

### Test Organization
```
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_specific_feature() {
        // Arrange
        // Act
        // Assert
    }
}
```

## Performance Considerations

### Memory
- Single-pass lexing (no backtracking)
- AST owns all strings (no lifetimes)
- Validators borrow AST (no cloning)
- IR owns all data (can be serialized)

### Speed
- Fail-fast on errors
- No unnecessary allocations
- Efficient symbol tables (HashMap)
- No regex in hot paths

### Scalability
- Linear time complexity O(n)
- Space complexity O(n)
- Parallel validation possible (independent passes)

## Extensibility Points

### Adding Components
1. Update schema in `validator/schema.rs`
2. Register in library
3. No code changes elsewhere

### Adding Constraints
1. Define schema
2. Add to library
3. Update validator

### Adding Motions
1. Define schema
2. Add to library
3. Update validator

### Adding Libraries
```rust
let library = Library {
    name: "my_library".to_string(),
    version: "1.0.0".to_string(),
    provides_components: vec!["custom_comp".to_string()],
    provides_constraints: vec![],
    provides_motions: vec![],
};

validator.add_library(library);
```

## Future Enhancements

### Possible Improvements
- [ ] Incremental compilation
- [ ] Better error recovery
- [ ] Language server protocol (LSP)
- [ ] Auto-formatting
- [ ] Documentation generation
- [ ] Visualization tools

### Explicitly Not Planned
- ❌ Control flow
- ❌ Runtime evaluation
- ❌ Inline expressions
- ❌ Implicit conversions
- ❌ Type inference
- ❌ Macros or metaprogramming

## Maintenance Guidelines

### Adding Features
1. Update `grammar.md` specification
2. Implement in appropriate module
3. Add comprehensive tests
4. Update this document
5. Update README

### Modifying Validators
- Each validator is independent
- Changes don't affect other validators
- Test in isolation first
- Then test full pipeline

### Changing Grammar
- Requires version bump
- Update all affected modules
- Maintain backward compatibility
- Document migration path

## Questions & Answers

**Q: Why is the order mandatory?**
A: Deterministic parsing. Simple compiler passes. No forward references.

**Q: Why no expressions?**
A: This is a data format, not a programming language. Compute in your tools.

**Q: Why so many validators?**
A: Separation of concerns. Each does one thing well. Easy to test and extend.

**Q: Can I skip validators?**
A: Technically yes (for testing), but never in production. All are required.

**Q: How do I add custom components?**
A: Create a library with schema. Register it. Import it in DSL files.

**Q: What about performance?**
A: Fast enough. Typical files compile in <10ms. Larger files scale linearly.

**Q: Why Rust?**
A: Type safety. Memory safety. Great error handling. Fast. Good for compilers.

---

**This architecture is designed to be:**
- ✅ Simple to understand
- ✅ Easy to extend
- ✅ Hard to misuse
- ✅ Fast to compile
- ✅ Pleasant to debug