# Loco Model MCP - Quick Start Guide

## Overview

The Loco Model MCP tool is a Model Context Protocol server that enables AI assistants to generate Loco framework models through natural language interaction. It provides tools for creating models with fields, validations, relationships, lifecycle hooks, and database migrations.

## Prerequisites

- Rust 1.75 or later
- Loco framework project (initialized with `loco new`)
- Claude Desktop or any MCP-compatible client

## Installation

### 1. Install the MCP Server

```bash
# Clone or navigate to the tool location
cd tools/mcps/loco-model-mcp

# Build the server
cargo build --release

# The binary will be available at:
# target/release/loco-model-mcp
```

### 2. Configure Claude Desktop

Add the following to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "loco-model": {
      "command": "/path/to/moonrepo-shadcnui/tools/mcps/loco-model-mcp/target/release/loco-model-mcp",
      "args": [],
      "env": {
        "LOCO_PROJECT_ROOT": "/path/to/your/loco/project"
      }
    }
  }
}
```

### 3. Restart Claude Desktop

After adding the configuration, restart Claude Desktop to load the MCP server.

## Quick Examples

### Example 1: Create a Basic User Model

```plaintext
User: Create a User model with email, username, and password fields

AI will use: generate_model
{
  "name": "User",
  "fields": [
    {
      "name": "email",
      "type": "string",
      "nullable": false,
      "unique": true,
      "validations": ["email"]
    },
    {
      "name": "username",
      "type": "string",
      "nullable": false,
      "unique": true
    },
    {
      "name": "password_hash",
      "type": "string",
      "nullable": false
    }
  ],
  "timestamps": true
}
```

**Generated Output**: Complete Loco model code ready to use in `src/models/user.rs`

### Example 2: Create a Blog Post Model with Relationships

```plaintext
User: Create a BlogPost model that belongs to User, with title, content, and published_at fields

AI will use: generate_model
{
  "name": "BlogPost",
  "fields": [
    {
      "name": "title",
      "type": "string",
      "nullable": false,
      "validations": ["length"]
    },
    {
      "name": "content",
      "type": "text",
      "nullable": false
    },
    {
      "name": "published_at",
      "type": "timestamp",
      "nullable": true
    }
  ],
  "relationships": [
    {
      "type": "belongs_to",
      "model": "User",
      "foreign_key": "user_id",
      "on_delete": "cascade"
    }
  ],
  "timestamps": true
}
```

### Example 3: Add Validation to Existing Model

```plaintext
User: Add email validation to the email field in the User model

AI will use: add_validation
{
  "model": "User",
  "validations": [
    {
      "field": "email",
      "rule": "email",
      "message": "Invalid email format"
    }
  ]
}
```

### Example 4: Add Lifecycle Hook

```plaintext
User: Add a hook to hash the password before saving the User model

AI will use: add_hook
{
  "model": "User",
  "hook_type": "before_save",
  "implementation": {
    "preset": "hash_password",
    "field": "password"
  }
}
```

### Example 5: Generate Database Migration

```plaintext
User: Generate a migration for the User model

AI will use: generate_migration
{
  "model": "User",
  "migration_type": "create_table"
}
```

## Available Tools

### Core Tools

1. **generate_model** - Generate complete models with all features
   - Fields, validations, relationships, hooks
   - Automatic timestamp fields (created_at, updated_at)
   - Optional migration generation

2. **add_field** - Add new field to existing model
   - Supports all Loco field types
   - Optional validation rules
   - Optional migration generation

3. **add_relationship** - Add model relationships
   - belongs_to, has_many, has_one, many_to_many
   - Configurable cascade actions
   - Foreign key customization

4. **add_validation** - Add validation rules
   - Email, URL, length, range, regex
   - Custom validation functions
   - Custom error messages

5. **add_hook** - Add lifecycle hooks
   - before/after save, create, update, delete
   - Preset hooks (normalize_email, hash_password, etc.)
   - Custom hook code

6. **generate_migration** - Generate database migrations
   - create_table, add_column, add_index, add_foreign_key
   - Compatible with Loco migration system

### Utility Tools

7. **validate_model** - Validate specification without generating code
   - Check for errors before generation
   - Verify field types and relationships

8. **list_field_types** - List all supported field types
   - Shows Loco → Rust → Database type mappings
   - Helpful for discovering available types

## Supported Field Types

- **Strings**: `string`, `text`
- **Numbers**: `int`, `small_int`, `big_int`, `float`, `double`, `decimal`
- **Identifiers**: `uuid`
- **Booleans**: `bool`
- **Time**: `timestamp`, `date`, `time`
- **JSON**: `json`, `jsonb`
- **Binary**: `binary`

## Validation Rules

- **email** - Email format validation
- **url** - URL format validation
- **length** - String length (min/max)
- **range** - Numeric range (min/max)
- **regex** - Custom regex pattern
- **custom** - Custom validation function

## Lifecycle Hooks

### Preset Hooks
- **normalize_email** - Convert email to lowercase and trim
- **generate_uuid** - Auto-generate UUID for field
- **set_timestamp** - Set current timestamp
- **trim_whitespace** - Trim whitespace from fields
- **hash_password** - Hash password using bcrypt

### Hook Types
- **before_save** / **after_save**
- **before_create** / **after_create**
- **before_update** / **after_update**
- **before_delete** / **after_delete**

## Relationship Types

### belongs_to
```json
{
  "type": "belongs_to",
  "model": "User",
  "foreign_key": "user_id",
  "on_delete": "cascade"
}
```

### has_many
```json
{
  "type": "has_many",
  "model": "Posts",
  "foreign_key": "user_id"
}
```

### has_one
```json
{
  "type": "has_one",
  "model": "Profile",
  "foreign_key": "user_id"
}
```

### many_to_many
```json
{
  "type": "many_to_many",
  "model": "Tag",
  "join_table": "post_tags"
}
```

## Testing the Server

### Manual Testing with MCP Inspector

```bash
# Install MCP Inspector
npm install -g @modelcontextprotocol/inspector

# Run the inspector
mcp-inspector /path/to/loco-model-mcp/target/release/loco-model-mcp

# Open the web UI to test tools interactively
```

### Using with Claude Desktop

1. Open Claude Desktop
2. Start a new conversation
3. Ask Claude to create a Loco model
4. The MCP server will automatically be invoked
5. Review the generated code

## Configuration

### Environment Variables

- **LOCO_PROJECT_ROOT** (required) - Path to your Loco project root
- **RUST_LOG** (optional) - Logging level (debug, info, warn, error)

### Example Configuration

```json
{
  "mcpServers": {
    "loco-model": {
      "command": "/path/to/loco-model-mcp",
      "args": [],
      "env": {
        "LOCO_PROJECT_ROOT": "/Users/dev/my-loco-app",
        "RUST_LOG": "info"
      }
    }
  }
}
```

## Best Practices

### 1. Start with Basic Models
```plaintext
Create a simple User model first, then add relationships and validations incrementally.
```

### 2. Use Descriptive Field Names
```plaintext
Use snake_case for field names (e.g., "user_id", "published_at", "email_address")
```

### 3. Add Validations Early
```plaintext
Define validation rules when creating the model to ensure data integrity from the start.
```

### 4. Generate Migrations Together
```plaintext
Use the generate_migration: true option when creating models to keep migrations in sync.
```

### 5. Test Generated Code
```plaintext
Always review and test the generated code before committing to your repository.
```

## Troubleshooting

### Server Not Starting

**Problem**: MCP server doesn't appear in Claude Desktop

**Solutions**:
- Check the path in claude_desktop_config.json
- Ensure the binary is executable: `chmod +x /path/to/loco-model-mcp`
- Verify LOCO_PROJECT_ROOT is set correctly
- Check Claude Desktop logs: `~/Library/Logs/Claude/`

### Compilation Errors in Generated Code

**Problem**: Generated model doesn't compile

**Solutions**:
- Validate the model spec first using `validate_model` tool
- Check that all referenced models exist
- Ensure field types are supported
- Review relationship foreign keys

### Performance Issues

**Problem**: Slow model generation

**Solutions**:
- The tool uses caching - first generation may be slower
- Check RUST_LOG level (debug logging is verbose)
- Ensure Loco project dependencies are up to date

## Next Steps

1. **Explore Examples**: Try the examples above in Claude Desktop
2. **Read Documentation**: Review `/specs/001-loco-model-mcp/spec.md` for detailed requirements
3. **Check Data Model**: See `/specs/001-loco-model-mcp/data-model.md` for internal structures
4. **Review Contracts**: Examine `/specs/001-loco-model-mcp/contracts/mcp-tools.json` for tool schemas
5. **Run Tests**: Execute the test suite when implementation is complete

## Support

For issues, questions, or contributions:
- Check the specification: `/specs/001-loco-model-mcp/spec.md`
- Review data model design: `/specs/001-loco-model-mcp/data-model.md`
- Examine tool contracts: `/specs/001-loco-model-mcp/contracts/mcp-tools.json`

## Version

This guide is for Loco Model MCP v1.0.0, based on:
- rmcp 0.8.0
- Loco framework latest stable
- MCP protocol specification
