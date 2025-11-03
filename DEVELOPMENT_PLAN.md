# Nutrition Helper - Development Plan

## Project Overview

A Tauri-based desktop application for managing daily nutrition plans with meal tracking, flexible meal options, and context-aware choices (home, office, restaurant). The app uses a card-based interface where users select meal options for daily time slots and track detailed information about each meal.

---

## 1. Architecture Design

### 1.1 Technology Stack

**Frontend:**

- **Framework**: React with TypeScript (modern, component-based, great Tauri support)
- **Styling**: Tailwind CSS (rapid development, consistent design system)
- **UI Components**: shadcn/ui or similar (accessible, customizable components)
- **State Management**: Zustand or React Context (lightweight for this use case)
- **Icons**: Lucide React (consistent icon system)

**Backend:**

- **Core**: Rust (Tauri backend)
- **Database**: SQLite (embedded, perfect for desktop apps, no server needed)
- **ORM**: Diesel or sqlx (type-safe database interactions)
- **API Layer**: Tauri Commands (IPC between frontend and Rust)

**Build & Dev Tools:**

- Tauri CLI
- Vite (fast development server)
- ESLint + Prettier (code quality)

### 1.2 Architecture Layers

```
┌─────────────────────────────────────────────┐
│           Frontend (React/TS)                │
│  ┌────────────────────────────────────────┐ │
│  │  UI Layer (Components)                 │ │
│  │  - MealSlot, MealCard, Calendar View   │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  State Management (Zustand/Context)    │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  Tauri API Client                      │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
                    ↕ (IPC)
┌─────────────────────────────────────────────┐
│           Backend (Rust/Tauri)               │
│  ┌────────────────────────────────────────┐ │
│  │  Command Handlers (Tauri Commands)     │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  Business Logic Layer                  │ │
│  │  - Meal validation                     │ │
│  │  - Weekly limits tracking              │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  Data Access Layer (Repository)        │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  SQLite Database                       │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
```

---

## 2. Data Model

### 2.1 Core Entities

#### MealTemplate

Represents the blueprint for a meal option that can be selected.

```rust
struct MealTemplate {
    id: i32,
    name: String,
    description: Option<String>,
    category: MealCategory, // Breakfast, Lunch, Dinner, Snack
    location_type: LocationType, // Home, Office, Restaurant, Any
    weekly_limit: Option<i32>, // null = unlimited, 1 = once per week, etc.
    nutritional_notes: Option<String>,
    tags: Vec<String>, // e.g., ["protein-rich", "vegetarian", "quick"]
    created_at: DateTime,
    updated_at: DateTime,
}
```

#### MealSlot

A specific time slot in a day that needs to be filled.

```rust
struct MealSlot {
    id: i32,
    date: Date,
    slot_type: SlotType, // Breakfast, MorningSnack, Lunch, AfternoonSnack, Dinner
    order: i32, // Display order in the day
}
```

#### MealEntry

An actual meal that was consumed/planned (links a template to a slot).

```rust
struct MealEntry {
    id: i32,
    meal_slot_id: i32,
    meal_template_id: i32,
    location: LocationType,
    portion_size: Option<f32>, // e.g., 1.0 = normal, 1.5 = large
    portion_unit: Option<String>, // e.g., "grams", "cups", "servings"
    notes: Option<String>,
    timestamp: DateTime,
    completed: bool, // planned vs actually eaten
}
```

#### Enums

```rust
enum MealCategory {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

enum LocationType {
    Home,
    Office,
    Restaurant,
    Any,
}

enum SlotType {
    Breakfast,
    MorningSnack,
    Lunch,
    AfternoonSnack,
    Dinner,
}
```

### 2.2 Database Schema (SQLite)

```sql
-- Meal Templates (meal options available to choose from)
CREATE TABLE meal_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL, -- 'breakfast', 'lunch', 'dinner', 'snack'
    location_type TEXT NOT NULL, -- 'home', 'office', 'restaurant', 'any'
    weekly_limit INTEGER, -- NULL for unlimited
    nutritional_notes TEXT,
    tags TEXT, -- JSON array
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Meal Entries (actual meals consumed/planned)
CREATE TABLE meal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL,
    slot_type TEXT NOT NULL, -- 'breakfast', 'morning_snack', 'lunch', etc.
    meal_template_id INTEGER NOT NULL,
    location TEXT NOT NULL,
    portion_size REAL,
    portion_unit TEXT,
    notes TEXT,
    completed BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (meal_template_id) REFERENCES meal_templates(id)
);

-- Weekly usage tracking (for enforcing limits)
CREATE VIEW weekly_meal_usage AS
SELECT
    meal_template_id,
    strftime('%Y-%W', date) as week,
    COUNT(*) as usage_count
FROM meal_entries
WHERE completed = TRUE
GROUP BY meal_template_id, week;

-- Indexes for performance
CREATE INDEX idx_meal_entries_date ON meal_entries(date);
CREATE INDEX idx_meal_entries_template ON meal_entries(meal_template_id);
CREATE INDEX idx_meal_templates_category ON meal_templates(category);
```

---

## 3. UX/UI Design

### 3.1 Main Views

#### 1. **Daily View (Primary Interface)**

- **Layout**: Calendar header + horizontal timeline of meal slots
- **Interaction**:
  - Each slot shows either an empty state or a filled meal card
  - Click empty slot → opens meal selection modal
  - Click filled slot → opens meal details editor
- **Visual Design**:
  - Cards with meal images/icons
  - Color coding by meal type (breakfast = yellow, lunch = blue, dinner = purple)
  - Location badges (home icon, office icon, restaurant icon)
  - Weekly limit indicator (e.g., "1/2 this week" badge)

#### 2. **Meal Selection Modal**

- **Search & Filter**:
  - Search bar (by meal name)
  - Filter by location type
  - Filter by category
  - Filter by tags
- **Card Grid**:
  - Visual cards with meal name, description, tags
  - Disabled state for weekly-limited meals that are exhausted
  - "Quick add" button on each card
- **Smart Suggestions**:
  - Prioritize meals that fit current slot type and haven't been used recently
  - Show "Frequently chosen" section

#### 3. **Meal Detail Editor**

- **Form Fields**:
  - Location dropdown (auto-filled from template but editable)
  - Portion size slider/input
  - Notes textarea
  - Completed checkbox
  - Timestamp display
- **Actions**:
  - Save, Delete, Cancel buttons
  - "Copy to tomorrow" quick action

#### 4. **Weekly Overview**

- **Calendar Grid**: 7 days × meal slots
- **Analytics Panel**:
  - Meals completed this week
  - Weekly limited meals usage
  - Variety score (different meals chosen)
- **Quick Actions**:
  - "Copy previous week"
  - "Template this week"

#### 5. **Meal Templates Manager**

- **List View**: All available meal templates
- **CRUD Operations**:
  - Add new template
  - Edit existing
  - Delete (with warning if used in entries)
- **Bulk Import**: CSV import for initial setup

#### 6. **Analytics/History View** (Phase 2+)

- Meal frequency charts
- Location distribution
- Weekly compliance tracking
- Export data to CSV

### 3.2 User Flow Examples

**Scenario 1: Planning Tomorrow's Breakfast**

1. User opens app → sees today's view
2. Clicks date picker → selects tomorrow
3. Clicks empty "Breakfast" slot
4. Modal opens with breakfast-appropriate templates
5. Filters by "Office" location
6. Selects "Oatmeal with Berries" card
7. Adjusts portion to 1.2 servings
8. Adds note: "Extra blueberries"
9. Saves → card appears in breakfast slot

**Scenario 2: Logging Actual Meal**

1. User ate lunch, opens app
2. Today's lunch slot shows planned "Chicken Salad"
3. Clicks card → detail editor opens
4. Marks "Completed" checkbox
5. Adjusts location from "Office" to "Restaurant" (changed plans)
6. Adds note: "Had extra avocado"
7. Saves → card shows completed state (checkmark badge)

### 3.3 Design System

**Colors:**

- Primary: Blue (#3B82F6) - actions, links
- Success: Green (#10B981) - completed meals
- Warning: Orange (#F59E0B) - weekly limit warnings
- Neutral: Gray scale for backgrounds and text
- Meal Categories:
  - Breakfast: Warm yellow (#FCD34D)
  - Lunch: Sky blue (#7DD3FC)
  - Dinner: Purple (#C4B5FD)
  - Snack: Light green (#86EFAC)

**Typography:**

- Headings: Inter or similar (clean, modern)
- Body: System font stack for performance
- Sizes: Clear hierarchy (base 16px)

**Spacing:**

- 4px base unit
- Consistent padding/margins using Tailwind scale

**Components:**

- Rounded corners (medium radius)
- Subtle shadows for cards
- Smooth transitions (200ms)
- Hover states for all interactive elements

---

## 4. Development Phases

### Phase 0: Project Setup & Scaffolding ✓ (95% COMPLETED)

**Goal**: Get development environment ready

- [x] Initialize Tauri project with React + TypeScript
- [x] Set up Tailwind CSS and UI component library
- [x] Configure project structure (folders, modules)
- [x] Add Rust dependencies (sqlx, serde, tokio, chrono)
- [ ] Create initial database schema and migrations (In Progress)
- [x] Set up development tooling (TypeScript, Tailwind)
- [x] Create README with setup instructions
- [x] Verify IPC communication works
- [x] Generate app icons
- [x] Document Linux graphics workaround

**Deliverable**: Running "Hello World" Tauri app with IPC communication tested

**Status**: Basic scaffolding complete. Database schema creation in progress.

---

### Phase 1: Core Data Layer (Week 1-2)

**Goal**: Build the backend foundation

**Backend Tasks:**

- [ ] Implement database models (MealTemplate, MealEntry)
- [ ] Create repository layer for CRUD operations
- [ ] Implement Tauri commands for:
  - Get all meal templates
  - Create/Update/Delete meal templates
  - Get meal entries by date range
  - Create/Update/Delete meal entries
- [ ] Add weekly limit validation logic
- [ ] Write unit tests for business logic

**Frontend Tasks:**

- [ ] Set up TypeScript types matching Rust models
- [ ] Create API client wrapper for Tauri commands
- [ ] Set up state management structure

**Deliverable**: Backend API that can create and retrieve meals

---

### Phase 2: Basic Daily View (Week 3-4)

**Goal**: Users can view and add meals to today

**Tasks:**

- [ ] Create MealSlot component (empty and filled states)
- [ ] Create MealCard component (visual display)
- [ ] Build daily timeline layout
- [ ] Implement date selector (today navigation)
- [ ] Create simple meal selection modal (list view)
- [ ] Implement add meal flow (select → save)
- [ ] Display filled slots with meal info

**Deliverable**: Can add and view meals for any given day

---

### Phase 3: Meal Selection & Filtering (Week 5)

**Goal**: Improve meal selection experience

**Tasks:**

- [ ] Enhanced meal selection modal with grid layout
- [ ] Add search functionality
- [ ] Implement filters (location, category, tags)
- [ ] Add meal template images/icons
- [ ] Show weekly limit indicators
- [ ] Disable exhausted weekly-limited meals
- [ ] Add "Recently Used" and "Suggested" sections

**Deliverable**: Smart, intuitive meal selection

---

### Phase 4: Meal Details & Editing (Week 6)

**Goal**: Track detailed information about meals

**Tasks:**

- [ ] Create meal detail editor component
- [ ] Implement portion size input
- [ ] Add notes field
- [ ] Add location override
- [ ] Implement completed/planned toggle
- [ ] Add delete meal functionality
- [ ] Show creation/update timestamps

**Deliverable**: Full meal tracking with details

---

### Phase 5: Templates Manager (Week 7)

**Goal**: Users can manage their meal options

**Tasks:**

- [ ] Create templates list view
- [ ] Build template form (create/edit)
- [ ] Implement CRUD operations
- [ ] Add template categories and tags management
- [ ] Implement weekly limit configuration
- [ ] Add template search/filter
- [ ] (Optional) CSV import for bulk template creation

**Deliverable**: Self-service template management

---

### Phase 6: Weekly View & Analytics (Week 8-9)

**Goal**: See patterns and plan ahead

**Tasks:**

- [ ] Create weekly calendar grid component
- [ ] Show 7-day meal overview
- [ ] Add weekly statistics panel
- [ ] Implement "Copy week" functionality
- [ ] Add meal variety tracking
- [ ] Show weekly limit usage summary
- [ ] Create basic charts (meal frequency, location distribution)

**Deliverable**: Weekly planning and insights

---

### Phase 7: Polish & Enhancement (Week 10+)

**Goal**: Production-ready experience

**Tasks:**

- [ ] Add keyboard shortcuts
- [ ] Implement drag-and-drop meal reordering
- [ ] Add data export (CSV, JSON)
- [ ] Implement data backup/restore
- [ ] Add user preferences/settings
- [ ] Improve error handling and user feedback
- [ ] Add animations and transitions
- [ ] Optimize performance (lazy loading, virtual scrolling)
- [ ] Write user documentation
- [ ] Create onboarding flow for new users

**Deliverable**: Polished, production-ready app

---

### Future Enhancements (Post-Launch)

- **Remote Database Support**: Connect to PostgreSQL/MySQL instance on NAS
  - Abstract database layer to support multiple backends
  - Configuration for connection strings
  - Migration path from SQLite to remote DB
- **Android App**: React Native version with shared business logic
  - Reuse TypeScript types and API patterns
  - Native mobile UI/UX optimizations
  - Sync with remote database
- Recipe integration (ingredient lists, instructions)
- Nutritional analysis (macros, calories)
- Shopping list generation
- Meal photos/images for templates
- Social features (share meal plans)
- AI meal suggestions based on history

---

## 5. Technical Decisions & Rationale

### Why Tauri?

- Native performance with small binary size
- Rust backend for safety and speed
- Cross-platform (Windows, macOS, Linux)
- Active community and good documentation
- No Electron overhead

### Why SQLite (Initially)?

- Perfect for desktop apps (embedded, no server)
- Excellent performance for this use case
- Zero configuration for development
- Easy backup (single file)
- Great SQL support for queries and analytics
- **Migration Path**: Using sqlx/Diesel allows easy transition to PostgreSQL/MySQL for NAS deployment
  - Same SQL syntax with minor adjustments
  - Repository pattern abstracts database implementation
  - Can support both local (SQLite) and remote (PostgreSQL) in final version

### Why React + TypeScript?

- Component-based architecture fits card UI pattern
- Strong typing helps with Tauri IPC
- Large ecosystem of UI libraries
- Fast development with modern tooling (Vite)

### State Management Choice

- **Zustand** (recommended):
  - Simple API, minimal boilerplate
  - Good TypeScript support
  - Easy to test
  - Sufficient for this app's complexity
- Alternative: React Context if we want zero dependencies

---

## 6. File Structure

```
nutrition-helper/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Tauri app entry
│   │   ├── commands/            # Tauri command handlers
│   │   │   ├── mod.rs
│   │   │   ├── meal_templates.rs
│   │   │   └── meal_entries.rs
│   │   ├── models/              # Data models
│   │   │   ├── mod.rs
│   │   │   ├── meal_template.rs
│   │   │   └── meal_entry.rs
│   │   ├── db/                  # Database layer
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   └── migrations/
│   │   ├── repository/          # Data access
│   │   │   ├── mod.rs
│   │   │   ├── meal_template_repo.rs
│   │   │   └── meal_entry_repo.rs
│   │   └── services/            # Business logic
│   │       ├── mod.rs
│   │       └── meal_service.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                          # React frontend
│   ├── main.tsx                 # App entry
│   ├── App.tsx                  # Root component
│   ├── components/              # React components
│   │   ├── common/              # Shared components
│   │   │   ├── Button.tsx
│   │   │   ├── Card.tsx
│   │   │   └── Modal.tsx
│   │   ├── meals/
│   │   │   ├── MealSlot.tsx
│   │   │   ├── MealCard.tsx
│   │   │   ├── MealSelectionModal.tsx
│   │   │   └── MealDetailEditor.tsx
│   │   ├── templates/
│   │   │   ├── TemplateList.tsx
│   │   │   └── TemplateForm.tsx
│   │   └── layout/
│   │       ├── Header.tsx
│   │       ├── Navigation.tsx
│   │       └── DatePicker.tsx
│   ├── views/                   # Page-level components
│   │   ├── DailyView.tsx
│   │   ├── WeeklyView.tsx
│   │   ├── TemplatesView.tsx
│   │   └── AnalyticsView.tsx
│   ├── lib/                     # Utilities
│   │   ├── api.ts              # Tauri command wrappers
│   │   ├── types.ts            # TypeScript types
│   │   └── utils.ts            # Helper functions
│   ├── store/                   # State management
│   │   ├── index.ts
│   │   ├── mealStore.ts
│   │   └── templateStore.ts
│   └── styles/
│       └── globals.css
│
├── public/                       # Static assets
├── package.json
├── tsconfig.json
├── tailwind.config.js
├── vite.config.ts
├── DEVELOPMENT_PLAN.md          # This file
└── README.md
```

---

## 7. Getting Started (Phase 0 Checklist)

When we initialize the project, we'll:

1. **Create Tauri App**:

   ```bash
   npm create tauri-app@latest
   # Choose: React + TypeScript
   ```

2. **Install Dependencies**:

   ```bash
   npm install zustand lucide-react date-fns
   npm install -D tailwindcss postcss autoprefixer
   npx tailwindcss init -p
   ```

3. **Set up Rust Dependencies** (Cargo.toml):

   ```toml
   [dependencies]
   tauri = "..."
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-native-tls"] }
   tokio = { version = "1", features = ["full"] }
   chrono = { version = "0.4", features = ["serde"] }
   ```

4. **Create Database Schema**: Run initial migration

5. **Set up Basic Folder Structure**: As outlined above

6. **Configure Tailwind**: Add to globals.css

7. **Create First Tauri Command**: Test IPC connection

---

## 8. Development Workflow

### Daily Development Cycle

1. **Backend First**: Implement Tauri command and business logic
2. **Test Backend**: Use Tauri dev tools or unit tests
3. **Frontend Integration**: Create UI that calls the command
4. **Manual Testing**: Verify in the running app
5. **Commit**: Small, focused commits

### Testing Strategy

- **Backend**: Unit tests for business logic, integration tests for DB operations
- **Frontend**: Component tests with React Testing Library (Phase 7)
- **E2E**: Manual testing initially, consider Playwright later

### Code Review Checkpoints

- After each phase completion
- Before adding new major features
- When refactoring significant code

---

## 9. Open Questions & Decisions Needed

### Decisions Made:

1. **Meal Slot Configuration**: Hardcoded 5 slots (Breakfast, Morning Snack, Lunch, Afternoon Snack, Dinner)

   - Can make configurable in Phase 7 if needed

2. **Image Handling**: Future enhancement (Phase 7+)

   - File system with paths in DB when implemented
   - Focus on core functionality first

3. **Weekly Limits**: Week starts on Monday ✓

4. **Data Validation**: Allow any positive number for portion sizes

   - Keep it flexible for user's needs

5. **Templates vs Entries**: Editing a template does NOT affect past entries

   - Entries are snapshots of the template at creation time

6. **First-Time Setup**: User will input their own meal templates

   - No default templates needed

7. **Database Architecture**:
   - **Phase 1-6**: SQLite for local development and testing
   - **Phase 7+**: Add remote database support (PostgreSQL/MySQL on NAS)
   - Design data layer to be database-agnostic from the start

---

## 10. Success Metrics

### Phase Completion Criteria:

- ✅ All phase tasks completed
- ✅ No critical bugs
- ✅ Code reviewed and refactored
- ✅ Basic testing passed
- ✅ Documentation updated

### Final Release Criteria:

- Can manage 50+ meal templates smoothly
- Can view/edit meals for any date
- Weekly limits enforced correctly
- Data persists between app restarts
- Intuitive UX (minimal learning curve)
- Stable (no crashes in normal usage)

---

## Next Steps

Now that we have this plan, we can:

1. **Review and refine** this document together
2. **Make decisions** on open questions
3. **Start Phase 0**: Initialize the Tauri project
4. **Create initial scaffolding**: Set up the basic project structure

Let's discuss any changes you'd like to make before we begin coding! 🚀
