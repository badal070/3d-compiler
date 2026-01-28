# Scene Description Language (SDL) Specification v1.0

## 0. Language Classification

**IS:**
- Strict, declarative scene description language
- Serialization format for intermediate representation
- Deterministic, boring, auditable

**IS NOT:**
- Programming language
- Math language
- Scripting language
- Natural language

**Logic execution:** Not supported. No apologies.

## 1. Core Design Goals (Non-Negotiable)

1. One DSL construct → One IR construct
2. Zero ambiguity
3. No execution semantics
4. Complete static validation
5. Forward-compatible versioning
6. LLM-safe but LLM-irrelevant

## 2. File Structure

**Mandatory Order (Non-Reorderable):**
```
scene
library_imports
entities
constraints
motions
timelines
```

**Constraints:**
- No forward references
- No cross-file mutation
- Single compilation unit per file

## 3. Formal Grammar (EBNF)

```ebnf
file          ::= scene library_imports entity* constraint* motion* timeline*

scene         ::= "scene" "{" scene_fields "}"
scene_fields  ::= "name:" STRING 
                  "version:" INTEGER
                  "ir_version:" STRING
                  "unit_system:" STRING

library_imports ::= "library_imports" "{" import_pair* "}"
import_pair     ::= IDENT ":" STRING

entity        ::= "entity" IDENT "{" entity_body "}"
entity_body   ::= "kind:" IDENT "components" "{" component* "}"

component     ::= IDENT "{" field* "}"
field         ::= IDENT ":" value

constraint    ::= "constraint" IDENT "{" constraint_body "}"
constraint_body ::= "type:" IDENT field*

motion        ::= "motion" IDENT "{" motion_body "}"
motion_body   ::= "target:" IDENT "type:" IDENT field*

timeline      ::= "timeline" IDENT "{" event* "}"
event         ::= "event" "{" event_fields "}"
event_fields  ::= "motion:" IDENT "start:" NUMBER "duration:" NUMBER

value         ::= NUMBER | STRING | IDENT | vector
vector        ::= "[" NUMBER "," NUMBER "," NUMBER "]"

NUMBER        ::= [0-9]+ ("." [0-9]+)? ([eE] [+-]? [0-9]+)?
STRING        ::= '"' [^"]* '"'
IDENT         ::= [a-zA-Z_][a-zA-Z0-9_]*
```

**Forbidden:**
- Expressions
- Operators
- Conditionals
- Loops

## 4. Validation Rules

### 4.1 Scene Header
- `version`: DSL schema version (integer)
- `ir_version`: IR compatibility string (semantic versioning)
- `unit_system`: Validated against known systems (SI, Imperial)
- Mismatch → Hard error, no migration

### 4.2 Library Imports
- Explicit dependency graph only
- Libraries are read-only
- Versioned imports mandatory
- No implicit globals

### 4.3 Entities
- `kind` field required
- `components` block required
- Each component appears at most once
- No assumed defaults

**Component Validation:**
- `transform.position`: 3D vector
- `transform.rotation`: 3D vector (radians)
- `transform.scale`: 3D vector
- `physical.mass`: positive number
- `geometry.primitive`: valid identifier

### 4.4 Constraints
- Do not mutate entities
- References only
- `type` must exist in library
- Referenced entities must exist
- Parameters validated by constraint schema

### 4.5 Motions
- No timing information
- No easing curves
- Pure rate-based definition
- `axis`: normalized vector or rejected
- `speed`: finite number
- `target`: must exist

### 4.6 Timelines
- Time in seconds (float)
- Events cannot overlap for same motion
- Multiple timelines allowed
- `duration` > 0
- `start` ≥ 0
- Referenced motion must exist

## 5. Error Codes

```
E001-E099: Lexical errors
E100-E199: Syntax errors
E200-E299: Schema errors
E300-E399: Reference errors
E400-E499: Unit validation errors
E500-E599: Library compatibility errors
```

**Error Format:**
```
E023: Undefined entity 'gearB'
 --> scene.dsl:42:12
  |
42|   driven: gearB
  |           ^^^^^ entity not found in scope
```

## 6. Extensibility

**Allowed:**
- New constraint types (via library)
- New motion types (via library)
- New components (via library)

**Forbidden:**
- Control flow constructs
- Inline math expressions
- Runtime evaluation hooks

**Extension Requirements:**
- Must live in libraries
- Must be versioned
- Must be schema-defined

## 7. Version Compatibility

**DSL Version:** Increments on grammar changes
**IR Version:** Semantic versioning (MAJOR.MINOR.PATCH)

Compiler enforces strict compatibility checking.

---

**Frozen:** 2025-01-24
**Authority:** This document is the single source of truth.