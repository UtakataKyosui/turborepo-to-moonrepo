# Tasks: Loco Model MCP Tool

**Feature**: 001-loco-model-mcp
**Input**: Design documents from `/specs/001-loco-model-mcp/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Tests are included per plan.md requirement for Test-Driven Development (NON-NEGOTIABLE)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions
- **Project root**: `/Users/utakatakyosui/Documents/moonrepo-shadcnui/tools/mcps/loco-model-mcp/`
- **Source**: `src/`
- **Templates**: `templates/`
- **Tests**: `tests/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure per plan.md

- [ ] T001 Create Rust project structure with Cargo.toml at tools/mcps/loco-model-mcp/
- [ ] T002 Add dependencies to Cargo.toml: rmcp 0.8.0, tokio, serde, tera, quote, validator, thiserror, lru, chrono
- [ ] T003 [P] Add dev dependencies to Cargo.toml: insta 1.34, tokio-test, tempfile
- [ ] T004 [P] Create src/ module structure: main.rs, lib.rs, server.rs, models/, generator/, validation/, error.rs, utils/
- [ ] T005 [P] Create templates/ directory for Tera templates
- [ ] T006 [P] Create tests/ directory structure: unit/, integration/, snapshots/
- [ ] T007 [P] Configure rustfmt.toml and clippy.toml for code quality standards
- [ ] T008 [P] Create README.md with project overview and installation instructions

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T009 Define error types in src/error.rs using thiserror (ValidationError, GenerationError, FileSystemError, TemplateError)
- [ ] T010 Implement McpError conversion in src/error.rs for MCP protocol compliance
- [ ] T011 [P] Create ModelSpecification struct in src/models/specification.rs with all attributes from data-model.md
- [ ] T012 [P] Create FieldDefinition struct and LocoFieldType enum in src/models/field.rs
- [ ] T013 [P] Create RelationshipDefinition enum in src/models/relationship.rs (BelongsTo, HasMany, HasOne, ManyToMany)
- [ ] T014 [P] Create ValidationRule struct and ValidationType enum in src/models/validation.rs
- [ ] T015 [P] Create LifecycleHook struct and HookType/HookImplementation enums in src/models/hook.rs
- [ ] T016 [P] Create GeneratedModel struct and GenerationMetadata in src/models/generated.rs
- [ ] T017 [P] Create MigrationDefinition struct and enums in src/models/migration.rs
- [ ] T018 Implement naming utilities in src/utils/naming.rs (snake_case, PascalCase conversion, keyword checking)
- [ ] T019 [P] Implement code formatting utilities in src/utils/formatting.rs (rustfmt integration)
- [ ] T020 Setup MCP server skeleton in src/server.rs with #[tool_router] macro
- [ ] T021 Implement stdio transport initialization in src/main.rs
- [ ] T022 Create template cache with LRU in src/generator/cache.rs (100-item capacity)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Generate Basic Loco Model (Priority: P1) 🎯 MVP

**Goal**: Enable developers to generate a complete Loco model with basic fields through the MCP interface

**Independent Test**: Request "create a User model with email and password fields" and verify valid Loco model file is generated

### Tests for User Story 1 (Test-Driven Development)

**NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T023 [P] [US1] Create snapshot test for basic model generation in tests/unit/model_generation.rs
- [ ] T024 [P] [US1] Create integration test for generate_model tool in tests/integration/generate_model_test.rs
- [ ] T025 [P] [US1] Create test fixtures for basic model specs in tests/fixtures/

### Implementation for User Story 1

- [ ] T026 [P] [US1] Create model.tera template in templates/ with basic struct, fields, derives
- [ ] T027 [P] [US1] Implement parse validation in src/validation/parse.rs (JSON Schema, required fields, format validation)
- [ ] T028 [P] [US1] Implement semantic validation in src/validation/semantic.rs (name conflicts, type compatibility, keyword checking)
- [ ] T029 [US1] Implement template manager in src/generator/templates.rs (Tera setup, template loading, caching integration)
- [ ] T030 [US1] Implement model code generator in src/generator/model.rs (render template, format code, handle timestamps)
- [ ] T031 [US1] Implement generate_model tool in src/server.rs using #[tool] macro
- [ ] T032 [US1] Add field type mapping logic in src/generator/model.rs (LocoFieldType → Rust type → SQL type)
- [ ] T033 [US1] Implement validation error messages in src/validation/mod.rs with actionable feedback
- [ ] T034 [US1] Add comprehensive field documentation generation in src/generator/model.rs

**Checkpoint**: At this point, User Story 1 should be fully functional - basic model generation works end-to-end

---

## Phase 4: User Story 2 - Define Model Relationships (Priority: P2)

**Goal**: Enable developers to create models with relationships (belongs_to, has_many, has_one, many_to_many)

**Independent Test**: Request "create a Post model that belongs to User and has many Comments" and verify relationship definitions are correct

### Tests for User Story 2 (Test-Driven Development)

- [ ] T035 [P] [US2] Create snapshot tests for each relationship type in tests/unit/relationships.rs
- [ ] T036 [P] [US2] Create integration test for complex relationship scenarios in tests/integration/relationship_test.rs

### Implementation for User Story 2

- [ ] T037 [P] [US2] Create relationship.tera template snippet in templates/
- [ ] T038 [US2] Implement relationship validation in src/validation/semantic.rs (foreign key validation, model existence checking)
- [ ] T039 [US2] Implement relationship code generation in src/generator/model.rs (Relation enum, impl blocks)
- [ ] T040 [US2] Implement add_relationship tool in src/server.rs using #[tool] macro
- [ ] T041 [US2] Add cascade action handling (Cascade, Restrict, SetNull, NoAction) in src/generator/model.rs
- [ ] T042 [US2] Implement foreign key field auto-generation in src/generator/model.rs
- [ ] T043 [US2] Add relationship documentation generation in src/generator/model.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - models with relationships can be generated

---

## Phase 5: User Story 3 - Add Custom Validations and Hooks (Priority: P3)

**Goal**: Enable developers to add custom validations and lifecycle hooks to models

**Independent Test**: Request "add before_save hook that normalizes email to lowercase" and verify hook implementation is generated

### Tests for User Story 3 (Test-Driven Development)

- [ ] T044 [P] [US3] Create snapshot tests for validation rules in tests/unit/validations.rs
- [ ] T045 [P] [US3] Create snapshot tests for lifecycle hooks in tests/unit/hooks.rs
- [ ] T046 [P] [US3] Create integration test for validation+hook combination in tests/integration/validation_hook_test.rs

### Implementation for User Story 3

- [ ] T047 [P] [US3] Create validation.tera template snippet in templates/
- [ ] T048 [P] [US3] Create hook.tera template snippet in templates/
- [ ] T049 [US3] Implement validation rule generation in src/generator/model.rs (email, url, length, range, regex, custom)
- [ ] T050 [US3] Implement lifecycle hook generation in src/generator/model.rs (ActiveModelBehavior trait methods)
- [ ] T051 [US3] Implement preset hook templates in src/models/hook.rs (normalize_email, generate_uuid, hash_password, etc.)
- [ ] T052 [US3] Implement add_validation tool in src/server.rs using #[tool] macro
- [ ] T053 [US3] Implement add_hook tool in src/server.rs using #[tool] macro
- [ ] T054 [US3] Add validation compatibility checking in src/validation/semantic.rs (field type vs validation rule)

**Checkpoint**: All three core user stories (US1, US2, US3) should now be independently functional

---

## Phase 6: User Story 4 - Generate Migration Files (Priority: P4)

**Goal**: Enable automatic generation of database migration files alongside model creation

**Independent Test**: Request model creation with "also generate migration" and verify both model and migration files are created with matching schemas

### Tests for User Story 4 (Test-Driven Development)

- [ ] T055 [P] [US4] Create snapshot tests for migration generation in tests/unit/migration_generation.rs
- [ ] T056 [P] [US4] Create integration test for model+migration workflow in tests/integration/migration_test.rs

### Implementation for User Story 4

- [ ] T057 [P] [US4] Create migration.tera template in templates/ (create_table, add_column, add_index, add_foreign_key)
- [ ] T058 [US4] Implement migration code generator in src/generator/migration.rs (table creation, columns, indexes, foreign keys)
- [ ] T059 [US4] Implement migration file naming/timestamping in src/generator/migration.rs
- [ ] T060 [US4] Implement generate_migration tool in src/server.rs using #[tool] macro
- [ ] T061 [US4] Add migration schema validation in src/validation/generation.rs (ensure migration matches model)
- [ ] T062 [US4] Integrate migration generation into generate_model tool in src/server.rs (when generate_migration flag is true)

**Checkpoint**: All user stories should now be independently functional - complete workflow from spec to model+migration

---

## Phase 7: Supporting Tools & Polish

**Purpose**: Complete the MCP tool set and add cross-cutting improvements

- [ ] T063 [P] Implement add_field tool in src/server.rs for incremental model modifications
- [ ] T064 [P] Implement validate_model tool in src/server.rs for pre-generation validation
- [ ] T065 [P] Implement list_field_types tool in src/server.rs for field type discovery
- [ ] T066 [P] Create field.tera template snippet in templates/
- [ ] T067 [P] Add generation validation in src/validation/generation.rs (template availability, code compilation)
- [ ] T068 [P] Implement file conflict detection in src/generator/mod.rs (warn about existing models)
- [ ] T069 [P] Add comprehensive documentation comments to all public APIs in src/
- [ ] T070 [P] Create examples directory with sample usage scenarios
- [ ] T071 Performance optimization: Verify LRU cache effectiveness with benchmarks
- [ ] T072 Security audit: Ensure no code injection vulnerabilities in template rendering
- [ ] T073 Run quickstart.md validation: Test all examples from quickstart.md
- [ ] T074 [P] Update README.md with complete API reference
- [ ] T075 [P] Create EXAMPLES.md with usage examples for all 8 tools

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion - Independent of US2/US3/US4
- **User Story 2 (Phase 4)**: Depends on Foundational phase completion - Independent of US1/US3/US4 (may integrate with US1 but testable alone)
- **User Story 3 (Phase 5)**: Depends on Foundational phase completion - Independent of US1/US2/US4 (may integrate with US1/US2 but testable alone)
- **User Story 4 (Phase 6)**: Depends on Foundational phase completion - Independent of US1/US2/US3 (may integrate with US1 but testable alone)
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May reference US1 models but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May enhance US1/US2 models but independently testable
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Complements US1 but independently testable

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD requirement)
- Data structures (models/) before generator logic
- Validation before generation
- Template creation before code generation
- Tool implementation after core logic is working
- Story complete before moving to next priority

### Parallel Opportunities

- **Setup (Phase 1)**: T003, T004, T005, T006, T007, T008 can run in parallel
- **Foundational (Phase 2)**: T011-T017 (model structs) can run in parallel; T018-T019 (utils) can run in parallel
- **User Story 1**: T023-T025 (tests), T026-T028 (templates+validation) can run in parallel
- **User Story 2**: T035-T036 (tests), T037-T038 (template+validation) can run in parallel
- **User Story 3**: T044-T046 (tests), T047-T048 (templates) can run in parallel
- **User Story 4**: T055-T056 (tests), T057 (template) can run in parallel
- **Polish (Phase 7)**: T063-T070, T074-T075 can run in parallel
- Once Foundational phase completes, all user stories (US1-US4) can start in parallel if team capacity allows

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task T023: "Create snapshot test for basic model generation in tests/unit/model_generation.rs"
Task T024: "Create integration test for generate_model tool in tests/integration/generate_model_test.rs"
Task T025: "Create test fixtures for basic model specs in tests/fixtures/"

# Launch templates and validation in parallel:
Task T026: "Create model.tera template in templates/"
Task T027: "Implement parse validation in src/validation/parse.rs"
Task T028: "Implement semantic validation in src/validation/semantic.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T008)
2. Complete Phase 2: Foundational (T009-T022) - CRITICAL blocker
3. Complete Phase 3: User Story 1 (T023-T034)
4. **STOP and VALIDATE**: Test User Story 1 independently with quickstart examples
5. Deploy/demo basic model generation capability

**MVP Deliverable**: Developers can generate complete, compilable Loco models with basic fields through MCP interface

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy (MVP! 🎯)
3. Add User Story 2 → Test independently → Deploy (Relationships!)
4. Add User Story 3 → Test independently → Deploy (Validations+Hooks!)
5. Add User Story 4 → Test independently → Deploy (Migrations!)
6. Add Polish (Phase 7) → Complete tool set ready

Each story adds value without breaking previous stories.

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (T001-T022)
2. Once Foundational is done:
   - Developer A: User Story 1 (T023-T034)
   - Developer B: User Story 2 (T035-T043)
   - Developer C: User Story 3 (T044-T054)
   - Developer D: User Story 4 (T055-T062)
3. Stories complete and integrate independently
4. Team converges on Polish phase (T063-T075)

---

## Task Metrics

**Total Tasks**: 75
- Phase 1 (Setup): 8 tasks
- Phase 2 (Foundational): 14 tasks
- Phase 3 (User Story 1): 12 tasks
- Phase 4 (User Story 2): 9 tasks
- Phase 5 (User Story 3): 11 tasks
- Phase 6 (User Story 4): 8 tasks
- Phase 7 (Polish): 13 tasks

**Tasks per User Story**:
- US1 (P1 - MVP): 12 tasks
- US2 (P2): 9 tasks
- US3 (P3): 11 tasks
- US4 (P4): 8 tasks

**Parallel Opportunities**: 35 tasks marked [P] can run in parallel within their phases

**Independent Test Criteria**:
- US1: Generate User model with email/password fields, verify compilation
- US2: Generate Post model with User relationship, verify relationship code
- US3: Add email normalization hook, verify hook implementation
- US4: Generate model with migration, verify migration matches schema

**Suggested MVP Scope**: Phase 1 + Phase 2 + Phase 3 (User Story 1 only) = 34 tasks

---

## Notes

- [P] tasks = different files, no dependencies within phase
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- **TDD is NON-NEGOTIABLE**: Tests must fail before implementation
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Success criteria from spec.md:
  - SC-002: 95% compilation rate (validate with snapshot tests)
  - SC-003: <30s generation time (validate with performance tests)
  - SC-004: 70% time reduction (measure before/after)
  - SC-007: Actionable error messages (validate with error handling tests)
