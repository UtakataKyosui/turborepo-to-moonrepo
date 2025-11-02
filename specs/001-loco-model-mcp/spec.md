# Feature Specification: Loco Model MCP Tool

**Feature Branch**: `001-loco-model-mcp`
**Created**: 2025-10-19
**Status**: Draft
**Input**: User description: "LoconâÇë’\Y‹MCPÄüë’‹z"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generate Basic Loco Model (Priority: P1)

A developer working on a Loco application needs to create a new model for their application. They use an AI assistant integrated with this MCP tool to generate a complete Loco model definition with proper schema, validations, and relationships.

**Why this priority**: This is the core functionality that delivers immediate value - the ability to scaffold Loco models through conversational AI interface, saving development time and ensuring consistency with Loco framework patterns.

**Independent Test**: Can be fully tested by requesting "create a User model with email and password fields" through the MCP interface and verifying that a valid Loco model file is generated with correct syntax and structure.

**Acceptance Scenarios**:

1. **Given** a Loco project exists, **When** developer requests creation of a model with basic fields (e.g., "create User model with name:string, email:string, age:integer"), **Then** the tool generates a complete Loco model file with proper struct definition, field types, and basic validation attributes

2. **Given** a developer specifies field constraints, **When** requesting model creation with validation rules (e.g., "email should be unique and required"), **Then** the generated model includes appropriate Loco validation macros and database constraints

3. **Given** a model creation request, **When** the tool generates the model code, **Then** all generated code follows Loco framework conventions including proper imports, derive macros, and struct organization

---

### User Story 2 - Define Model Relationships (Priority: P2)

A developer needs to create models with relationships to other entities in their application. They use the MCP tool to define belongs_to, has_many, and has_one relationships between models.

**Why this priority**: Relationships are essential for most non-trivial applications, but a basic model without relationships is still functional, making this a valuable enhancement to the core functionality.

**Independent Test**: Can be tested by requesting "create a Post model that belongs to User and has many Comments" and verifying that the generated model includes proper relationship definitions and foreign key references.

**Acceptance Scenarios**:

1. **Given** existing models in the project, **When** developer specifies a belongs_to relationship (e.g., "Post belongs to User"), **Then** the tool generates a model with the appropriate foreign key field and relationship macro

2. **Given** a one-to-many relationship specification, **When** requesting a has_many relationship (e.g., "User has many Posts"), **Then** the tool adds the relationship configuration to both models with proper conventions

3. **Given** relationship definitions, **When** the model is generated, **Then** the tool ensures referential integrity constraints are properly defined in the model schema

---

### User Story 3 - Add Custom Validations and Hooks (Priority: P3)

A developer needs to add custom business logic validations and lifecycle hooks to their models. They use the MCP tool to generate models with before_save, after_create callbacks and custom validation methods.

**Why this priority**: Custom validations and hooks are important for complex business logic but aren't required for basic CRUD operations, making them valuable enhancements that can be added after core functionality is working.

**Independent Test**: Can be tested by requesting "add before_save hook that normalizes email to lowercase" and verifying the generated model includes the hook implementation with correct Loco lifecycle callback syntax.

**Acceptance Scenarios**:

1. **Given** a model with custom validation requirements, **When** developer specifies custom validation logic (e.g., "validate that price is positive"), **Then** the tool generates a custom validation method following Loco patterns

2. **Given** lifecycle hook requirements, **When** developer requests before/after hooks (e.g., "before_save, set updated_at timestamp"), **Then** the tool generates the appropriate hook implementation with correct Loco trait methods

3. **Given** complex validation rules, **When** multiple validations are specified, **Then** the tool generates all validation methods and properly integrates them into the model's validation chain

---

### User Story 4 - Generate Migration Files (Priority: P4)

A developer wants to automatically generate database migration files alongside model creation. The MCP tool generates both the model code and corresponding migration that creates the database table with proper schema.

**Why this priority**: While migrations are important, developers can manually create them after the model is generated. This enhancement improves workflow efficiency but isn't critical for the tool's core value proposition.

**Independent Test**: Can be tested by requesting model creation with "also generate migration" flag and verifying both model file and migration file are created with matching schemas.

**Acceptance Scenarios**:

1. **Given** a model creation request, **When** developer includes migration generation option, **Then** the tool creates both a model file and a timestamped migration file in the appropriate directories

2. **Given** model field specifications, **When** migration is generated, **Then** the migration includes all table columns with correct data types, constraints, and indexes matching the model definition

3. **Given** model relationships, **When** migration is generated, **Then** foreign key constraints and relationship indexes are properly defined in the migration

---

### Edge Cases

- What happens when requesting a model name that already exists in the project?
- How does the system handle invalid field type specifications (e.g., unsupported data types)?
- What happens when relationship targets reference models that don't exist yet?
- How does the tool handle field name conflicts with Rust reserved keywords?
- What happens when validation rules conflict with database constraints?
- How does the tool handle generation requests for projects that aren't properly initialized Loco applications?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Tool MUST provide an MCP interface that accepts natural language model specifications and returns generated Loco model code
- **FR-002**: Tool MUST support all standard Loco field types including String, Integer, Float, Boolean, DateTime, and custom types
- **FR-003**: Tool MUST generate valid Rust code that compiles without errors and follows Loco framework conventions
- **FR-004**: Tool MUST support field-level validations including required, unique, length constraints, and format validations
- **FR-005**: Tool MUST generate proper derive macros for Loco models including Model, Serialize, Deserialize, and Clone
- **FR-006**: Tool MUST support relationship definitions including belongs_to, has_many, has_one, and many_to_many
- **FR-007**: Tool MUST validate that generated field names comply with Rust naming conventions and don't conflict with reserved keywords
- **FR-008**: Tool MUST generate models with proper module structure and imports required by Loco framework
- **FR-009**: Tool MUST support generation of custom validation methods with proper Loco validator trait implementation
- **FR-010**: Tool MUST support lifecycle hooks including before_save, after_save, before_create, after_create, before_update, after_update
- **FR-011**: Tool MUST optionally generate corresponding database migration files with table schema matching the model definition
- **FR-012**: Tool MUST detect and warn about potential conflicts with existing model files before generation
- **FR-013**: Tool MUST provide clear error messages when model specifications are invalid or incomplete
- **FR-014**: Tool MUST support generation of indexed fields with appropriate database index definitions
- **FR-015**: Tool MUST generate models with proper handling of nullable and non-nullable fields

### Key Entities *(include if feature involves data)*

- **Model Specification**: Represents the user's input describing the desired model including name, fields, relationships, validations, and hooks. Attributes include model name, field definitions (name, type, constraints), relationship definitions, validation rules, and lifecycle hooks.

- **Generated Model**: The output Rust source code file containing the complete Loco model implementation. Includes struct definition, derive macros, field declarations, relationship configurations, validation methods, and lifecycle hook implementations.

- **Field Definition**: Describes a single field in the model including its name, data type, database constraints (unique, required, default value), validation rules, and whether it's indexed. Related to the Model Specification.

- **Relationship Definition**: Describes associations between models including type (belongs_to, has_many, etc.), target model name, foreign key field, and any join table configuration for many-to-many relationships.

- **Migration File**: Optional output that creates the database schema matching the model definition. Includes table creation, column definitions, indexes, and foreign key constraints.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can generate a complete, compilable Loco model from a natural language specification in under 30 seconds
- **SC-002**: 95% of generated models compile successfully without requiring manual code fixes
- **SC-003**: Generated model code passes Loco framework validation and follows official Loco model patterns and conventions
- **SC-004**: Tool successfully handles model specifications with up to 20 fields and 5 relationships without errors
- **SC-005**: Generated models include comprehensive documentation comments explaining field purposes and constraints
- **SC-006**: Tool reduces model creation time by 70% compared to manual coding (measured from specification to working model)
- **SC-007**: Error messages for invalid specifications are clear enough that developers can fix issues without external documentation
- **SC-008**: Generated migration files successfully execute against supported databases (PostgreSQL, MySQL, SQLite) without errors
