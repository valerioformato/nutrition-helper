# GitHub Copilot Instructions for Nutrition Helper

## Current Status

**Phase: 0 Complete ✅ | Ready to Start: Phase 1**

- Database schema finalized (5 tables, 9 indexes, 2 views)
- All 5 database tests passing
- Schema reviewed with user and refined based on real nutrition plan
- Key decisions documented (servings-based portions, planning+logging workflow, Monday weeks)
- Next: Build core data layer (models, repositories, Tauri commands)

**Recent Changes:**

- Commit 2486bce (2024-11-04): Schema refinements (servings field, planning/logging clarification)
- Commit 18fb35a (2024-11-04): Tags system with hierarchies
- Commit db5e4c8: 4-level hierarchy correction

## Project Context

This is a Tauri-based desktop application for managing daily nutrition plans with meal tracking. Always refer to `DEVELOPMENT_PLAN.md` for complete architecture details and development phases.

## Core Architecture Decisions

### Technology Stack

- **Frontend**: React + TypeScript + Tailwind CSS + Vite
- **Backend**: Rust (Tauri)
- **Database**: SQLite initially, designed to migrate to PostgreSQL/MySQL on NAS later
- **State Management**: Zustand
- **ORM**: sqlx (database-agnostic)

### Key Design Patterns

1. **Repository Pattern**: Abstract database operations to support multiple DB backends
2. **Card-Based UI**: Meal templates are visual cards that fill meal slots
3. **Four-Level Hierarchy**: Slot → Template → Option → Entry
   - **Slots**: 5 fixed meal slots per day (Breakfast, Morning Snack, Lunch, Afternoon Snack, Dinner)
   - **Templates**: Meal cards that can fill slots (the "Oppure" choices in nutrition plans)
   - **Options**: Ingredient/variation choices within templates (e.g., "philadelphia", "ricotta", "crema spalmabile")
   - **Entries**: Actual logged meal options (snapshot of which option was chosen at specific date/slot)
4. **Real-world example**:
   - Slot: COLAZIONE → Template: "Pane con marmellata" → Options: ["philadelphia", "ricotta", "crema spalmabile"] → Entry: logged "ricotta" on 2024-11-04
5. **Dual Navigation**: Slot-first (plan by slot) and template-first (search for specific template)

### Important Rules

- **Weeks start on Monday** for weekly limit calculations
- **Entries are snapshots**: Editing a template does NOT affect past meal entries
- **Weekly frequency suggestions**: Soft recommendations (show warnings), not hard limits
- **Tags system**: Relational tags with hierarchies (e.g., pasta_integrale → pasta)
  - Tags must exist in database before use (no typos)
  - Multiple tags per meal option
  - Parent/child relationships for ingredient hierarchies
  - Weekly suggestions tracked per tag
- **Location-aware**: Meals have different options for Home/Office/Restaurant
- **Database agnostic**: Write queries that work with both SQLite and PostgreSQL

## Development Phases

Current Phase: **Phase 0 - Project Setup & Scaffolding**

Follow the phased approach in DEVELOPMENT_PLAN.md:

- Phase 0: Project initialization (current)
- Phase 1: Core data layer
- Phase 2: Basic daily view
- Phase 3: Meal selection & filtering
- Phase 4: Meal details & editing
- Phase 5: Templates manager
- Phase 6: Weekly view & analytics
- Phase 7: Polish & enhancements (including remote DB support)

## Coding Standards

### Rust Backend

- Use Tauri commands for IPC
- Implement proper error handling with Result<T, E>
- Keep business logic separate from Tauri command handlers
- Use repository pattern for all database operations
- Add #[derive(Serialize, Deserialize)] to models that cross IPC boundary
- **Write comprehensive tests for all code** (unit + integration)
- **Target 85%+ test coverage** for backend code
- Include test modules inline with `#[cfg(test)]`

### TypeScript Frontend

- Use TypeScript strictly (no `any` types)
- Components in PascalCase, files match component names
- Use Tailwind utility classes for styling
- Wrap Tauri commands in typed API functions (in `lib/api.ts`)
- Keep components small and focused
- **Write tests for utilities and components** (Phase 7)
- **Target 70%+ test coverage** for frontend code

### Database

- Use sqlx with compile-time checked queries
- Design schema to be compatible with both SQLite and PostgreSQL
- Always use parameterized queries (never string concatenation)
- Include timestamps (created_at, updated_at) on all tables
- **Write tests for all database operations** (migrations, queries, constraints)

### Testing Standards

**Backend Testing (Required from Phase 1):**

- Write unit tests for all business logic
- Write integration tests for repository operations
- Use temporary in-memory SQLite databases (`:memory:`)
- Create test data builders for consistency
- Test both happy paths and error cases
- Run tests before committing: `cargo test`

**Frontend Testing (Now Available!):**

- **Testing framework**: Vitest + React Testing Library (configured and ready)
- Write unit tests for utilities and helpers
- Write component tests for React components
- Test user interactions and state changes
- Mock Tauri commands using `vi.mock('@tauri-apps/api/core')`
- Run tests: `npm test` (watch mode) or `npm run test:coverage`
- **Target 70%+ test coverage** for frontend code
- See `TESTING_FRONTEND.md` for detailed guide
- Test user interactions and state changes
- Run tests before committing: `npm test`

**Test Organization:**

```
src-tauri/
├── src/
│   └── models/meal.rs       # Unit tests inline with #[cfg(test)]
└── tests/
    ├── integration/         # Integration tests
    ├── db_tests/           # Database tests
    └── helpers/            # Test utilities
```

**Test Naming:**

- Use descriptive test names: `test_weekly_limit_enforced_correctly`
- Follow Arrange-Act-Assert pattern
- One logical assertion per test
- Use `#[tokio::test]` for async tests

**Integration Testing Strategy (Tauri-Specific):**

Key lessons learned from Phase 1:

1. **Don't Fight the Framework**

   - Tauri State is hard to mock; use it naturally in command module tests
   - Command modules are perfect for end-to-end integration tests
   - Keep separate integration tests focused on critical concerns (IPC serialization)

2. **Test What Can Break**

   - In Tauri apps, the IPC boundary is the critical integration point
   - IPC serialization failures are runtime disasters
   - Focus integration tests on verifying all types can cross IPC boundary

3. **Avoid Duplication**

   - If command modules have comprehensive integration tests, don't duplicate
   - Leverage existing coverage and add complementary tests
   - Document where integration coverage exists

4. **Complementary Test Suites**
   - Command module tests: Full end-to-end flows (Command → Repository → Database)
   - Separate integration tests: IPC serialization verification (fast, no DB)

## File Organization

Follow the structure outlined in DEVELOPMENT_PLAN.md Section 6:

- `src-tauri/src/`: Rust backend (commands, models, db, repository, services)
- `src/`: React frontend (components, views, lib, store, styles)
- Keep concerns separated (presentation, business logic, data access)

## Current Focus

We are in Phase 0: Setting up the project scaffolding. Next steps will be building the core data layer with database models and Tauri commands.

## Future Considerations

- Mobile app support (Tauri Mobile or React Native) - Phase 8+
- Remote database on NAS (PostgreSQL) - Phase 7
- Meal images/photos - Phase 7+
- Analytics and insights - Phase 6-7

## Quick Reminders

- Always check DEVELOPMENT_PLAN.md when making architectural decisions
- User will provide their own meal templates (no default data needed)
- Portion sizes are flexible (any positive number)
- The app should work offline with local database initially

## Development Workflow

- **Test-Driven Development**: Write tests alongside or before implementation
- **Run Tests Frequently**:
  - Backend: `cargo test` after every significant change
  - Frontend: `npm test` for unit tests, `npm run test:coverage` for coverage
- **Run Clippy Before Committing**: Always run `cargo clippy --lib --tests -- -D warnings -A dead_code -A unused_imports` before committing to ensure code quality
- **Verify Coverage**: Ensure new code meets coverage targets (85% backend, 70% frontend)
- **Manual UI/UX Testing Required**: After ANY UI/UX change (components, views, styling, interactions), ALWAYS stop and ask the user to test manually before proceeding
- **Test Before Moving On**: After completing any task, ALWAYS ask the user to test the app before moving to the next task
- Build incrementally and verify each step works
- Don't proceed to new features until current ones are confirmed working
- **All code must have tests** - no exceptions for backend code from Phase 1 onwards, frontend when available

## Platform-Specific Notes

- **Linux**: On some Linux systems with graphics driver issues, run dev mode with: `WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev`
- This environment variable disables hardware acceleration to work around GBM buffer issues
