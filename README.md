# Nutrition Helper

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

**Phase 0**: Project Setup & Scaffolding (95% Complete) 🔧

### Completed

- [x] Architecture planning
- [x] Technology stack decisions
- [x] Development plan documentation
- [x] Tauri 2.0 project initialization
- [x] React + TypeScript + Vite setup
- [x] Tailwind CSS v3 configuration
- [x] Rust dependencies (sqlx, serde, tokio, chrono)
- [x] Project structure (commands, models, db, repository, services)
- [x] IPC communication tested and working
- [x] App icons generated for all platforms
- [x] Linux graphics workaround documented
- [x] Git repository initialized with commits

### In Progress

- [ ] Database schema creation (SQL migrations)
- [ ] Database connection test

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

## 🎯 Development Workflow

1. **Check Current Phase**: See DEVELOPMENT_PLAN.md for current phase objectives
2. **Backend First**: Implement Rust backend (models, repository, Tauri commands)
3. **Frontend Integration**: Build React components that use Tauri commands
4. **Test**: Manual testing in dev mode
5. **Update Plan**: Mark tasks complete in DEVELOPMENT_PLAN.md

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

## 🤝 Contributing

This is a personal project, but the development plan and architecture can serve as a reference for similar applications.

## 📄 License

TBD

---

**Need help?** Check [DEVELOPMENT_PLAN.md](./DEVELOPMENT_PLAN.md) for complete documentation.
