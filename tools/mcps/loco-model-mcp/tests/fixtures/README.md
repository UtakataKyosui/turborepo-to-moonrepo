# Test Fixtures

This directory contains reusable test data in JSON format for testing model generation.

## Available Fixtures

### Basic Models

- **basic_user.json**: Simple User model with common fields (email, name, age)
  - Demonstrates basic field types (string, integer)
  - Shows nullable vs required fields
  - Includes unique and indexed fields

- **minimal_config.json**: Minimal configuration without timestamps
  - Shows model without timestamps
  - Demonstrates primary key configuration
  - Example of nullable text field

- **custom_table_name.json**: Model with custom table name
  - Shows how to override default table name
  - Uses "app_users" instead of "users"

### Complex Models

- **blog_post.json**: Blog post model with relationships
  - Demonstrates belongs_to relationship
  - Shows default values
  - Includes indexed foreign key

- **product_catalog.json**: E-commerce product model
  - Demonstrates decimal fields with precision/scale
  - Shows JSONB metadata field
  - Includes category indexing

- **account_with_validation.json**: Account model with validation rules
  - Demonstrates email validation
  - Shows length validation (min/max)
  - Includes regex pattern validation
  - Multiple validations per field

## Usage in Tests

```rust
use std::fs;

// Load a fixture
let fixture_path = "tests/fixtures/basic_user.json";
let json_str = fs::read_to_string(fixture_path).unwrap();
let spec: ModelSpecification = serde_json::from_str(&json_str).unwrap();

// Use in tests
let generated_code = generate_model_code(&spec)?;
assert!(generated_code.contains("email"));
```

## Field Type Reference

Common field types used in fixtures:

- `string`: VARCHAR type, Rust String
- `text`: TEXT type for longer content, Rust String
- `integer`: i32 in Rust
- `boolean`: bool in Rust
- `timestamp`: DateTime<Utc> in Rust
- `uuid`: Uuid type
- `decimal`: Decimal with precision and scale
- `jsonb`: PostgreSQL binary JSON, serde_json::Value

## Relationship Types

- `belongs_to`: Many-to-one relationship (e.g., Post belongs to User)
- `has_many`: One-to-many (not shown in basic fixtures, will be added in Phase 4)
- `has_one`: One-to-one (not shown in basic fixtures, will be added in Phase 4)
- `many_to_many`: Many-to-many with join table (Phase 4)

## Validation Rules

- `email`: Email format validation
- `length`: Min/max string length
- `regex`: Pattern matching
- `url`: URL format validation
- `range`: Numeric min/max (Phase 5)
- `custom`: Custom validation function (Phase 5)
