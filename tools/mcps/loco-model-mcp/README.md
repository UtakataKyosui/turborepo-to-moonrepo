# Loco Model MCP

An MCP (Model Context Protocol) server that enables AI assistants to generate Loco framework models through natural language interaction.

## Overview

This tool provides 8 MCP tools for generating and managing Loco framework models:

1. **generate_model** - Generate complete Loco models with fields and validations
2. **add_field** - Add fields to existing models
3. **add_relationship** - Define model relationships (belongs_to, has_many, etc.)
4. **add_validation** - Add validation rules to model fields
5. **add_hook** - Add lifecycle hooks (before_save, after_create, etc.)
6. **generate_migration** - Generate SeaORM migration files
7. **validate_model** - Validate model specifications before generation
8. **list_field_types** - List available field types and their Rust/SQL mappings

## Installation

### Prerequisites

- Rust 2021 edition or later
- Cargo package manager

### Building from Source

```bash
cd tools/mcps/loco-model-mcp
cargo build --release
```

## Usage

### As an MCP Server

The tool runs as an MCP server using stdio transport:

```bash
cargo run
```

### Configuration for Claude Desktop

Add to your Claude Desktop MCP configuration:

```json
{
  "mcpServers": {
    "loco-model": {
      "command": "/path/to/loco-model-mcp/target/release/loco-model-mcp"
    }
  }
}
```

## Features

### Supported Field Types

- **String types**: String, Text
- **Numeric types**: Integer, SmallInt, BigInt, Float, Double, Decimal
- **Date/Time**: Timestamp, Date, Time
- **Special**: Uuid, Boolean, Json, Jsonb, Binary
- **References**: Foreign key relationships

### Relationship Types

- **belongs_to**: Many-to-one relationship
- **has_many**: One-to-many relationship
- **has_one**: One-to-one relationship
- **many_to_many**: Many-to-many with join table

### Validation Rules

- Email format validation
- URL validation
- Length constraints (min/max)
- Range validation (numeric)
- Regular expression matching
- Custom validation functions

### Lifecycle Hooks

- **before_save** - Execute before saving
- **after_save** - Execute after saving
- **before_create** - Execute before creating
- **after_create** - Execute after creating
- **before_update** - Execute before updating
- **after_update** - Execute after updating
- **before_delete** - Execute before deleting
- **after_delete** - Execute after deleting

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --test unit

# Run integration tests
cargo test --test integration

# Update snapshot tests
cargo insta review
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check for unused dependencies
cargo machete
```

## Architecture

```
src/
├── main.rs              # MCP server entry point
├── lib.rs               # Library root
├── server.rs            # MCP tool implementations
├── error.rs             # Error types
├── models/              # Data structures
│   ├── specification.rs # Model specification
│   ├── field.rs         # Field definitions
│   ├── relationship.rs  # Relationship types
│   ├── validation.rs    # Validation rules
│   ├── hook.rs          # Lifecycle hooks
│   ├── generated.rs     # Generated model output
│   └── migration.rs     # Migration definitions
├── generator/           # Code generation
│   ├── model.rs         # Model code generator
│   ├── migration.rs     # Migration generator
│   ├── templates.rs     # Template manager
│   └── cache.rs         # LRU template cache
├── validation/          # Validation logic
│   ├── parse.rs         # Input validation
│   ├── semantic.rs      # Semantic validation
│   └── generation.rs    # Output validation
└── utils/               # Utilities
    ├── naming.rs        # Naming conventions
    └── formatting.rs    # Code formatting
```

## License

MIT

## Contributing

Contributions are welcome! Please ensure all tests pass and code is formatted before submitting PRs.
