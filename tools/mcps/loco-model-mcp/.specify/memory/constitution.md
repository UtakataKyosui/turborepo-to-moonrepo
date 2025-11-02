<!--
SYNC IMPACT REPORT - Constitution Update
Version: 1.0.0 (Initial ratification)
Changes:
  - Initial constitution created for loco-model-mcp project
  - Established 7 core principles for MCP server development
  - Defined development workflow and governance rules

Templates Requiring Updates:
  ✅ plan-template.md - Constitution Check gate already present
  ✅ spec-template.md - User stories structure aligns with principle-driven development
  ✅ tasks-template.md - Task organization supports testing and deployment principles

Follow-up TODOs: None
-->

# Loco Model MCP Constitution

## Core Principles

### I. MCP Protocol Compliance

All features MUST adhere to Model Context Protocol (MCP) specifications. Tools and resources exposed through the MCP server must follow the rmcp framework conventions. Protocol compatibility is non-negotiable - breaking changes require explicit versioning and migration paths.

**Rationale**: MCP is the foundational contract with clients. Protocol violations break integrations and erode trust in the server's reliability.

### II. Type Safety First

Leverage Rust's type system to enforce correctness at compile time. All data models MUST use strongly-typed structures with `serde` for serialization. Avoid `String` typing where domain-specific types provide better safety (e.g., `ModelId` vs `String`).

**Rationale**: Type safety catches errors early, documents intent, and provides free validation through the type system.

### III. Test-Driven Development (NON-NEGOTIABLE)

Tests MUST be written before implementation. The cycle is: Write test → User approval → Test fails → Implement → Test passes. All MCP tools must have unit tests, and complex workflows require integration tests.

**Rationale**: TDD ensures features meet requirements, prevents regressions, and serves as living documentation.

### IV. Resource-Oriented Design

Model MCP server capabilities as resources (nouns) and tools (verbs). Resources represent queryable data (model schemas, configurations), while tools represent actions (generate code, validate models). Clear separation prevents confusion.

**Rationale**: Resource/tool distinction aligns with REST principles and makes the MCP interface intuitive.

### V. Error Handling Excellence

All error paths MUST be explicit and actionable. Use Rust's `Result<T, E>` type with detailed error enums. Errors surfaced through MCP MUST include context helpful for client debugging (not internal stack traces).

**Rationale**: Clear errors reduce support burden and enable clients to handle failures gracefully.

### VI. Performance Consciousness

MCP servers are synchronous by nature - slow operations block clients. Operations MUST complete in <2 seconds for typical inputs. For long-running tasks, provide progress mechanisms or async patterns.

**Rationale**: Client UX depends on server responsiveness. Slow servers degrade the AI assistant experience.

### VII. Observability & Debugging

All MCP tool invocations MUST be logged with structured data (tool name, parameters, duration, outcome). Support debug modes that emit detailed traces without compromising security.

**Rationale**: Debugging MCP integrations is challenging. Comprehensive logging is the primary troubleshooting mechanism.

## Development Workflow

### Code Review Requirements

- All changes require review before merge
- Constitution compliance must be verified in PR description
- Breaking changes require explicit CHANGELOG entries with migration guide

### Testing Gates

- Unit tests MUST pass before PR approval
- Integration tests MUST pass for MCP protocol interactions
- Manual testing MUST verify tool behavior through MCP client (e.g., Claude Desktop)

### Deployment Standards

- Semantic versioning (MAJOR.MINOR.PATCH)
- MAJOR: Breaking MCP protocol or tool signature changes
- MINOR: New tools or resources added
- PATCH: Bug fixes, internal improvements

## Governance

This constitution supersedes all other development practices. Changes to the constitution require:

1. Documentation of the change rationale
2. Review by project maintainers
3. Migration plan for affected features
4. Update to this document with version bump

All pull requests and code reviews MUST verify compliance with these principles. Complexity that violates principles must be explicitly justified in the implementation plan.

For runtime development guidance specific to MCP servers, see `.specify/memory/` for additional context and patterns.

**Version**: 1.0.0 | **Ratified**: 2025-10-18 | **Last Amended**: 2025-10-18
