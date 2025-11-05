# Nutrition Helper

[![CI](https://github.com/valerioformato/nutrition-helper/actions/workflows/ci.yml/badge.svg)](https://github.com/valerioformato/nutrition-helper/actions/workflows/ci.yml)
[![Code Coverage](https://github.com/valerioformato/nutrition-helper/actions/workflows/coverage.yml/badge.svg)](https://github.com/valerioformato/nutrition-helper/actions/workflows/coverage.yml)

A desktop application for managing daily nutrition plans with meal tracking, built with Tauri, React, and TypeScript.

## 📋 Overview

Nutrition Helper allows you to:

- Plan meals using a card-based interface
- Track meals across different locations (home, office, restaurant)
- Enforce weekly limits on certain meal options
- Log meal details (portion sizes, notes, completion status)
- View daily and weekly meal plans
- Manage your meal template library

## 🏗️ Architecture

- **Frontend**: React + TypeScript + Tailwind CSS
- **Backend**: Rust (Tauri)
- **Database**: SQLite (local), with planned migration to PostgreSQL on NAS
- **State Management**: Zustand

## 📚 Documentation

- **[DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md)**: Complete architecture, data models, and phased development roadmap
- **[.github/copilot-instructions.md](./.github/copilot-instructions.md)**: AI assistant context and coding standards

## 🚀 Current Status

**Phase 1**: Core Data Layer ✅ Complete | **Phase 2**: Tauri Commands 🔧 In Progress

### Completed

#### Phase 0: Project Setup & Scaffolding ✅
- [x] Architecture planning
- [x] Technology stack decisions
- [x] Development plan documentation
- [x] **Comprehensive testing strategy defined**
- [x] Tauri 2.0 project initialization
- [x] React + TypeScript + Vite setup
- [x] Tailwind CSS v3 configuration
- [x] Rust dependencies (sqlx, serde, tokio, chrono)
- [x] Project structure (commands, models, db, repository, services)
- [x] IPC communication tested and working
- [x] App icons generated for all platforms
- [x] Linux graphics workaround documented
- [x] Git repository initialized with commits
- [x] Database schema creation (5 tables, 9 indexes, 2 views)
- [x] Database migrations with version control
- [x] Testing infrastructure setup

#### Phase 1: Core Data Layer ✅
- [x] **All 5 data models** (Tag, Template, Option, Entry, Enums)
- [x] **Model validation and serialization** (20 unit tests)
- [x] **TagRepository** (9 methods, 8 tests) - Hierarchical tags with weekly suggestions
- [x] **MealTemplateRepository** (9 methods, 8 tests) - Templates with slot compatibility
- [x] **MealOptionRepository** (14 methods, 10 tests) - Options with tag management
- [x] **MealEntryRepository** (16 methods, 12 tests) - Meal logging with date queries
- [x] **Database integration** (5 integration tests) - Schema, indexes, views
- [x] **63 total tests passing** (100% success rate)
- [x] **Repository pattern** fully established
- [x] **Weekly usage views** for tracking meal frequency
- [x] **ISO week calculations** (Monday-based)
- [x] **CI/CD pipeline** with GitHub Actions

### In Progress

#### Phase 2: Tauri Commands Layer 🔧
- [ ] Create IPC command handlers for all repositories
- [ ] Set up app state management (SqlitePool)
- [ ] Error serialization for frontend communication
- [ ] Business logic services (weekly limits, slot validation)

## 🛠️ Setup Instructions

### Prerequisites

- Node.js (v18+)
- Rust (latest stable)
- npm or yarn

### Installation

```bash
# Install dependencies
npm install

# Install Tauri CLI
npm install -D @tauri-apps/cli

# Run in development mode
npm run tauri dev

# Run in development mode (Linux with graphics driver issues)
npm run tauri:dev:linux

# Build for production
npm run tauri build
```

### Database Setup

The database will be automatically initialized on first run. SQLite database file will be stored in the app's data directory.

### Running Tests

**Backend Tests:**

```bash
# Run all Rust tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test meal_template

# Run integration tests only
cargo test --test integration
```

**Frontend Tests** (Phase 7):

```bash
# Run all frontend tests
npm test

# Run in watch mode
npm test -- --watch

# Run with coverage
npm test -- --coverage
```

**Test Coverage:**

```bash
# Generate Rust coverage report (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

## 🎯 Development Workflow

1. **Check Current Phase**: See DEVELOPMENT_PLAN.md for current phase objectives
2. **Backend First**: Implement Rust backend (models, repository, Tauri commands)
3. **Write Tests**: Comprehensive unit and integration tests (from Phase 1)
4. **Verify Coverage**: Ensure tests meet coverage targets
5. **Frontend Integration**: Build React components that use Tauri commands
6. **Manual Testing**: Test in dev mode for UX validation
7. **Update Plan**: Mark tasks complete in DEVELOPMENT_PLAN.md

## 🧪 Testing Strategy

We follow a **comprehensive testing approach** from Phase 1 onwards:

### Backend Testing (Phase 1+)

- ✅ **Unit Tests**: All models, business logic, and utilities (90%+ coverage)
- ✅ **Integration Tests**: Repository operations with real database (85%+ coverage)
- ✅ **Database Tests**: Schema, migrations, and constraints (100% coverage)

### Frontend Testing (Phase 7)

- ✅ **Unit Tests**: Utilities and helper functions (80%+ coverage)
- ✅ **Component Tests**: React components in isolation (70%+ coverage)
- ✅ **E2E Tests**: Critical user workflows (100% coverage)

### Test Infrastructure

- Tests use temporary SQLite databases for isolation
- Automated test data builders for consistency
- Fast execution (tests run in < 10 seconds)
- Coverage reports generated automatically

See [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md) Section 9 for detailed testing guidelines.

## 📁 Project Structure

```
nutrition-helper/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC command handlers
│   │   ├── models/         # Data models
│   │   ├── db/            # Database setup and migrations
│   │   ├── repository/     # Data access layer
│   │   └── services/       # Business logic
├── src/                    # React frontend
│   ├── components/         # React components
│   ├── views/             # Page-level components
│   ├── lib/               # Utilities and API wrappers
│   └── store/             # State management
└── DEVELOPMENT_PLAN.md     # Complete development guide
```

## 🔑 Key Concepts

### Meal Templates

Reusable meal options that define:

- Name and description
- Category (breakfast, lunch, dinner, snack)
- Location type (home, office, restaurant, any)
- Weekly limits (optional)
- Tags for filtering

### Meal Entries

Actual logged meals that include:

- Reference to a meal template
- Date and time slot
- Location where eaten
- Portion size
- Notes
- Completion status

### Meal Slots

5 daily time slots:

1. Breakfast
2. Morning Snack
3. Lunch
4. Afternoon Snack
5. Dinner

## 🎨 Design Principles

- **Card-based UI**: Visual meal cards that fill time slots
- **Context-aware**: Different options for different locations
- **Flexible tracking**: Detailed or simple, based on user needs
- **Weekly planning**: Enforce limits and view patterns
- **Offline-first**: Works without network, data stored locally

## 🗺️ Roadmap

- **Phase 1-2** (Weeks 1-4): Core functionality and daily view
- **Phase 3-4** (Weeks 5-6): Enhanced selection and detail tracking
- **Phase 5-6** (Weeks 7-9): Template management and weekly overview
- **Phase 7+** (Week 10+): Polish, remote DB support, and enhancements

See [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md) for detailed phase breakdown.

## 🔮 Future Features

- Remote database on NAS (PostgreSQL)
- Android app (React Native or Tauri Mobile)
- Meal photos
- Nutritional analysis
- Recipe integration
- Shopping lists

## 📝 Development Notes

- Weeks start on **Monday** for weekly limit calculations
- Meal entries are snapshots (editing templates doesn't affect past entries)
- Database is designed to be backend-agnostic for future PostgreSQL migration
- No default meal templates; user provides their own nutrition plan
- **Comprehensive testing required** - all code must have tests before merging
- Test coverage targets: Backend 85%+, Frontend 70%+, Critical paths 100%

## 🤝 Contributing

This is a personal project, but the development plan and architecture can serve as a reference for similar applications.

## 📄 License

TBD

---

**Need help?** Check [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md) for complete documentation.
