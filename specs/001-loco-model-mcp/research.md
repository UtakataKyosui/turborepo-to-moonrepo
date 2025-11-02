# Research Report: Loco Model MCP Tool

**Date**: 2025-10-19
**Feature**: 001-loco-model-mcp
**Phase**: Phase 0 - Technical Research

## Executive Summary

This research establishes the technical foundation for building a Model Context Protocol (MCP) server that generates Loco framework models through natural language interaction. The tool will leverage the `rmcp` Rust framework for MCP protocol compliance and SeaORM/Loco patterns for code generation.

**Key Findings**:
- MCP protocol provides standardized tool/resource interface for AI assistants
- rmcp 0.8.0+ offers production-ready Rust SDK with macros for rapid development
- Loco uses SeaORM for ORM with migration-first workflow
- Performance target: <2s tool execution (ideally <200ms)
- Type-safe code generation possible using `quote` and template engines

---

## 1. MCP Protocol & rmcp Framework

### Decision: Use rmcp 0.8.0 with stdio transport

**Rationale**:
- Official Rust SDK from Anthropic with active maintenance
- Macro-based tool definition reduces boilerplate by ~70%
- stdio transport offers lowest latency (<50ms overhead)
- Full async/await support with Tokio integration
- Production-ready features: error handling, parameter validation, content types

**Alternatives Considered**:
1. **Custom JSON-RPC implementation**: Rejected due to maintenance burden and protocol compliance risk
2. **HTTP/SSE transport**: Better for remote services, but unnecessary overhead for same-machine operation
3. **Python MCP SDK**: Rejected due to performance concerns and type safety requirements

**Implementation Approach**:
```toml
[dependencies]
rmcp = { version = "0.8.0", features = ["server", "transport-io", "macros"] }
tokio = { version = "1", features = ["full"] }
```

**Key Capabilities**:
- `#[tool_router]` - Automatic tool registration
- `#[tool(description = "...")]` - Tool metadata with schema validation
- `#[tool(aggr)]` - Parameter aggregation into structured types
- Built-in error types conforming to JSON-RPC 2.0

---

## 2. Loco Framework Model Patterns

### Decision: Target Loco 0.8+ with SeaORM code generation

**Rationale**:
- Loco uses SeaORM as ORM layer (not custom implementation)
- Migration-first workflow aligns with database-driven development
- SeaORM provides ActiveModelBehavior trait for lifecycle hooks
- Standard field types with clear syntax (! = required, ^ = unique)

**Loco Model Anatomy**:
```rust
// Generated entity (src/models/_entities/users.rs)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,              // string!
    pub name: Option<String>,       // string
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

// Custom model logic (src/models/users.rs)
impl Validatable for _entities::users::ActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(UserValidator { /* ... */ })
    }
}
```

**Field Type Mapping**:
| Loco Syntax | Database Type | Rust Type | Notes |
|-------------|---------------|-----------|-------|
| `string` | VARCHAR(255) NULL | `Option<String>` | Nullable |
| `string!` | VARCHAR(255) NOT NULL | `String` | Required |
| `string^` | VARCHAR(255) UNIQUE | `String` | Unique constraint |
| `uuid!` | UUID NOT NULL | `Uuid` | Primary key candidate |
| `timestamp` | TIMESTAMP NULL | `Option<DateTime>` | With timezone |
| `int!` | INTEGER NOT NULL | `i32` | Standard integer |
| `bool` | BOOLEAN NULL | `Option<bool>` | Three-state logic |
| `json` | JSON/JSONB NULL | `Option<serde_json::Value>` | Structured data |

**Validation Patterns**:
```rust
use validator::{Validate, ValidationError};

#[derive(Debug, Validate)]
pub struct ModelValidator {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(range(min = 18, max = 120))]
    pub age: Option<i32>,
}
```

**Relationship Syntax**:
```rust
// belongs_to
#[sea_orm(
    belongs_to = "super::users::Entity",
    from = "Column::UserId",
    to = "super::users::Column::Id"
)]

// has_many
#[sea_orm(has_many = "super::posts::Entity")]

// many_to_many (via join table)
fn via() -> Option<RelationDef> {
    Some(super::join_table::Relation::Model.def().rev())
}
```

---

## 3. Code Generation Strategy

### Decision: Template-based generation with Tera + quote for type safety

**Rationale**:
- Tera provides flexible text templating for boilerplate code
- quote crate ensures generated Rust is syntactically valid
- Hybrid approach: templates for structure, quote for complex logic
- Easier to maintain than pure string concatenation
- Supports code formatting through rustfmt integration

**Generation Pipeline**:
```
User Input (JSON)
  → Validate & Parse (serde)
  → Transform to Internal Model
  → Template Rendering (Tera)
  → Syntax Validation (syn)
  → Format (rustfmt)
  → Output (String)
```

**Template Example**:
```rust
// templates/model_entity.tera
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "{{ table_name }}")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    {% for field in fields %}
    pub {{ field.name }}: {{ field.rust_type }},
    {% endfor %}
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}
```

**Alternatives Considered**:
1. **String concatenation**: Too error-prone, no syntax checking
2. **AST manipulation with syn/quote**: Overkill for templates, harder to read
3. **Handlebars**: Less Rust-idiomatic than Tera, similar capabilities

---

## 4. Performance Optimization

### Decision: LRU cache + lazy generation + connection pooling

**Rationale**:
- LRU cache prevents redundant generation of identical models
- Lazy generation defers work until necessary
- Connection pooling reduces database overhead for validation checks
- Monitoring ensures <2s performance requirement is met

**Caching Strategy**:
```rust
use lru::LruCache;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CachedGenerator {
    cache: Arc<RwLock<LruCache<String, GeneratedCode>>>,
    generator: ModelGenerator,
}

impl CachedGenerator {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity.try_into().unwrap()))),
            generator: ModelGenerator::new(),
        }
    }

    pub async fn generate(&self, params: &ModelParams) -> Result<String> {
        let key = params.cache_key();

        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.peek(&key) {
                return Ok(cached.code.clone());
            }
        }

        // Generate and cache
        let code = self.generator.generate(params)?;
        {
            let mut cache = self.cache.write().await;
            cache.put(key, GeneratedCode { code: code.clone() });
        }

        Ok(code)
    }
}
```

**Performance Targets**:
- ✅ Simple model (1-5 fields): <50ms
- ✅ Complex model (6-20 fields): <100ms
- ✅ With relationships (1-5): <150ms
- ✅ With validations: +20ms overhead
- ⚠️ Budget: 200ms (ideal), 500ms (acceptable), 2000ms (maximum)

---

## 5. Error Handling Architecture

### Decision: thiserror for domain errors + MCP error mapping

**Rationale**:
- thiserror provides clean enum-based error types with automatic Display
- Easy conversion to MCP ErrorData format
- Structured errors enable better client-side handling
- Preserves error context through the stack

**Error Type Hierarchy**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelGeneratorError {
    #[error("Invalid model name '{0}': must be PascalCase")]
    InvalidModelName(String),

    #[error("Invalid field type '{field_type}' for field '{field_name}'")]
    InvalidFieldType { field_name: String, field_type: String },

    #[error("Duplicate field name: {0}")]
    DuplicateField(String),

    #[error("Invalid relationship: {0}")]
    InvalidRelationship(String),

    #[error("Template rendering failed: {0}")]
    TemplateError(#[from] tera::Error),

    #[error("Syntax validation failed: {0}")]
    SyntaxError(String),
}

// MCP error mapping
impl From<ModelGeneratorError> for rmcp::ErrorData {
    fn from(err: ModelGeneratorError) -> Self {
        rmcp::ErrorData {
            code: -32000,  // Application error
            message: err.to_string(),
            data: Some(serde_json::json!({
                "error_type": std::any::type_name_of_val(&err),
            })),
        }
    }
}
```

**Client-Facing Error Messages**:
```rust
#[tool(description = "Generate Loco model")]
async fn generate_model(&self, params: ModelParams) -> Result<CallToolResult, McpError> {
    match self.generator.generate(&params).await {
        Ok(code) => Ok(CallToolResult::success(vec![Content::text(code)])),
        Err(e) => Err(McpError {
            code: -32000,
            message: format!("Model generation failed: {}", e),
            data: Some(serde_json::json!({
                "model_name": params.name,
                "suggestion": suggest_fix(&e),
            })),
        }),
    }
}

fn suggest_fix(error: &ModelGeneratorError) -> String {
    match error {
        ModelGeneratorError::InvalidModelName(_) =>
            "Use PascalCase (e.g., UserProfile, BlogPost)".to_string(),
        ModelGeneratorError::InvalidFieldType { .. } =>
            "Supported types: string, text, int, uuid, bool, timestamp, json".to_string(),
        _ => "Check input parameters and try again".to_string(),
    }
}
```

---

## 6. Testing Strategy

### Decision: Multi-layered testing with insta snapshots

**Rationale**:
- Snapshot testing ideal for code generation (captures output changes)
- Property testing validates invariants across input space
- Integration tests verify MCP protocol compliance
- Unit tests cover individual components

**Test Pyramid**:
```
     E2E Tests (1-2)
    ↗              ↖
  Integration Tests (5-10)
 ↗                        ↖
Unit Tests (50+) + Property Tests (10+)
```

**Snapshot Testing Example**:
```rust
#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    #[test]
    fn test_generate_user_model() {
        let params = ModelParams {
            name: "User".to_string(),
            fields: vec![
                field("email", "string!", &["email"], true),
                field("name", "string!", &[], false),
            ],
            timestamps: true,
        };

        let code = ModelGenerator::new().generate(&params).unwrap();
        assert_snapshot!(code);  // Auto-updates on first run
    }
}
```

**Test Dependencies**:
```toml
[dev-dependencies]
insta = { version = "1.34", features = ["yaml"] }
proptest = "1.4"
tokio-test = "0.4"
```

---

## 7. Project Dependencies

### Core Dependencies (Confirmed)

```toml
[dependencies]
# MCP Server Framework
rmcp = { version = "0.8.0", features = ["server", "transport-io", "macros"] }

# Async Runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# Code Generation
tera = "1.19"          # Template engine
quote = "1.0"          # Rust syntax generation
syn = "2.0"            # Rust syntax parsing

# Validation
validator = { version = "0.18", features = ["derive"] }

# Performance
lru = "0.12"           # LRU cache

# ORM (for reference/validation)
sea-orm = { version = "1.0", features = ["runtime-tokio-rustls", "sqlx-postgres"] }

[dev-dependencies]
insta = { version = "1.34", features = ["yaml"] }
proptest = "1.4"
tokio-test = "0.4"
```

---

## 8. Key Design Decisions Summary

| Decision Point | Choice | Alternatives Rejected | Rationale |
|----------------|--------|----------------------|-----------|
| **MCP Framework** | rmcp 0.8.0 | Custom JSON-RPC, Python SDK | Official support, type safety, performance |
| **Transport** | stdio | HTTP/SSE | Lowest latency, simpler deployment |
| **Code Generation** | Tera templates + quote | Pure strings, AST only | Balance of readability and safety |
| **ORM Target** | Loco/SeaORM | Direct SQL, Diesel | Matches user requirement |
| **Caching** | LRU in-memory | Redis, No cache | Simplicity, <2s requirement |
| **Error Handling** | thiserror + MCP errors | anyhow only, Panic | Structured errors, client usability |
| **Testing** | insta snapshots | Manual assertions | Code gen ideal use case |
| **Async Runtime** | Tokio | async-std | MCP SDK compatibility |

---

## 9. Risk Mitigation

### Performance Risks
**Risk**: Code generation exceeds 2s limit for complex models
**Mitigation**:
- Implement caching layer
- Profile critical path with benchmarks
- Lazy generation strategies
- Progress monitoring

### Protocol Compliance Risks
**Risk**: rmcp SDK API changes in future versions
**Mitigation**:
- Pin to specific version (0.8.0)
- Integration tests validate protocol compliance
- Monitor rmcp release notes

### Code Quality Risks
**Risk**: Generated code doesn't compile or follow Loco conventions
**Mitigation**:
- Validate syntax with `syn` before returning
- Snapshot tests capture regressions
- Golden file tests with known-good examples

---

## 10. Implementation Roadmap

**Phase 1: Foundation (P1 - Basic Model Generation)**
1. MCP server setup with rmcp
2. Parse model specification from JSON
3. Generate basic entity struct (no relationships, no validations)
4. Unit tests + snapshot tests

**Phase 2: Validations (P2 - Relationships)**
5. Add validation code generation
6. Implement relationship generation (belongs_to, has_many)
7. Foreign key handling

**Phase 3: Advanced Features (P3 - Hooks)**
8. Custom validation methods
9. Lifecycle hooks (before_save, after_create, etc.)
10. Integration tests

**Phase 4: Migrations (P4)**
11. Migration file generation
12. Index generation
13. E2E testing

---

## References

1. **MCP Specification**: https://modelcontextprotocol.io/specification
2. **rmcp GitHub**: https://github.com/modelcontextprotocol/rust-sdk
3. **rmcp Documentation**: https://docs.rs/rmcp/latest/rmcp/
4. **Loco Framework**: https://loco.rs/docs/
5. **SeaORM**: https://www.sea-ql.org/SeaORM/docs/
6. **validator crate**: https://docs.rs/validator/latest/validator/
7. **Tera templating**: https://keats.github.io/tera/

---

**Research Status**: ✅ COMPLETE
**Next Phase**: Phase 1 - Design & Contracts (data-model.md, contracts/, quickstart.md)
