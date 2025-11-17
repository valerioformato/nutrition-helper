# Nutrition Helper - Development Plan

## Current Project Status

**Last Updated**: November 17, 2025  
**Current Phase**: Phase 5 Complete ✅ | Ready to Start: Phase 3

### Completed Phases

- ✅ **Phase 0**: Project Setup & Scaffolding (Complete with CI/CD)
- ✅ **Phase 1**: Core Data Layer (41 Tauri commands, 139 tests passing)
- ✅ **Phase 2**: Basic Daily View (Meal slot display and basic interactions)
- ✅ **Phase 5**: Templates Manager (Full CRUD for templates, options, and tags)
- ✅ **Coverage Configuration**: Excluded Tauri command wrappers from coverage (thin IPC layer, no business logic)

### In Progress

- 🔧 **Phase 5 Task 10**: Add proper navigation/routing (React Router)

### Next Up

- 📋 **Phase 3**: Meal Selection & Filtering
- 📋 **Phase 4**: Meal Details & Editing
- 📋 **Phase 6**: Weekly View & Analytics
- 📋 **Phase 7**: Polish & Enhancement

### Key Achievements

- Complete backend API with 41 Tauri commands
- 139 backend tests passing with ~90%+ coverage of business logic
- Full Templates Manager with hierarchical tag support
- Database schema finalized and migrated
- CI/CD pipeline operational
- **Code Coverage Strategy**: Business logic (repos/models/services) fully tested; IPC wrappers excluded via `tarpaulin.toml`

---

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
│           Frontend (React/TS)               │
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
│           Backend (Rust/Tauri)              │
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

### 2.1 Four-Level Hierarchy

The application uses a **four-level data model** based on real-world nutrition planning:

**Slot → Template → Option → Entry**

This allows users to:

1. Fill fixed meal slots throughout the day (Breakfast, Snacks, Lunch, Dinner)
2. Choose from templates/cards compatible with each slot (Templates - the "Oppure" choices)
3. Select specific ingredient variations within each template (Options)
4. Log actual meals consumed or planned (Entries)

**Real-world example:**

- **Slot**: COLAZIONE (Breakfast)
- **Template 1**: "Pane con marmellata e formaggio spalmabile" (Oppure)
- **Template 2**: "Pane con affettato/formaggio/uovo" (Oppure)
- **Template 3**: "Yogurt con cereali e frutta secca"
  - **Option 3a**: philadelphia
  - **Option 3b**: ricotta
  - **Option 3c**: crema spalmabile 100% frutta secca
- **Entry**: On 2024-11-04 at Breakfast, logged "ricotta" option

### 2.2 Core Entities

#### SlotType (Level 1 - Implicit)

Fixed meal slots defined in application logic:

```rust
enum SlotType {
    Breakfast,
    MorningSnack,
    Lunch,
    AfternoonSnack,
    Dinner,
}
```

These are the 5 fixed time slots per day that structure meal planning.

#### MealTemplate (Level 2)

Represents a meal template/card that can fill a slot (separated by "Oppure" in nutrition plans).

```rust
struct MealTemplate {
    id: i32,
    name: String,                    // e.g., "Pane con marmellata e formaggio spalmabile"
    description: Option<String>,
    compatible_slots: Vec<SlotType>, // Which slots this template can fill
    location_type: LocationType,     // Home, Office, Restaurant, Any
    tags: Vec<String>,               // e.g., ["dolce", "veloce"]
    created_at: DateTime,
    updated_at: DateTime,
}
```

**Examples for COLAZIONE slot:**

- "Pane con marmellata e formaggio spalmabile"
- "Pane con affettato/formaggio/uovo"
- "Yogurt con cereali e frutta secca"

#### MealOption (Level 3)

Represents specific ingredient choices within a template (the variations separated by "o", "e/o", "+").

```rust
struct MealOption {
    id: i32,
    template_id: i32,                // Foreign key to MealTemplate
    name: String,                    // e.g., "philadelphia", "ricotta", "crema spalmabile"
    description: Option<String>,
    weekly_limit: Option<i32>,       // null = unlimited, 1 = once per week, etc.
    nutritional_notes: Option<String>,
    created_at: DateTime,
    updated_at: DateTime,
}
```

**Examples within "Pane con marmellata" template:**

- "philadelphia" (unlimited)
- "ricotta" (2x/week)
- "crema spalmabile 100% frutta secca" (1x/week)

#### MealEntry (Level 4)

An actual meal option logged/consumed (links an option to a date/slot).
Supports both meal planning (future dates) and meal logging (past/completed meals).

```rust
struct MealEntry {
    id: i32,
    meal_option_id: i32,             // Foreign key to MealOption
    date: Date,
    slot_type: SlotType,             // Which slot this meal fills
    location: LocationType,          // Where it was eaten
    servings: f32,                   // Number of servings (default 1.0)
                                     // Nutrition plan uses strict serving sizes
    notes: Option<String>,
    completed: bool,                 // FALSE = planned, TRUE = actually consumed
    created_at: DateTime,
    updated_at: DateTime,
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

### 2.3 Key Features of This Model

**Flexibility:**

- Templates group related meals logically
- Options can specify which slots they're compatible with
- Easy to add new options without restructuring

**Use Cases Supported:**

1. **Slot-First Planning** (traditional):

   - Click empty "Lunch" slot
   - Browse templates (categories)
   - Select specific option
   - Log the meal

2. **Meal-First Search** (new):

   - Search "I want soup"
   - App shows all soup options across templates
   - App indicates which slots each option can fill
   - User selects slot and logs the meal

3. **Weekly Limit Tracking:**
   - Limits apply per option (e.g., "Chicken Caesar max 2x/week")
   - Can track usage across the entire template if needed
   - View enforces counts based on completed entries

**Relationships:**

```
Template (1) ──────> (many) Option (1) ──────> (many) Entry
   "Pane con"            "philadelphia"           "Nov 4, Breakfast"
   marmellata"           "ricotta"                "Nov 5, Breakfast"
                         "crema spalmabile"
```

### 2.4 Tags System for Ingredient Tracking

The application uses a **relational tags system** to track ingredients and enforce weekly frequency suggestions:

#### Tag Entity

```rust
struct Tag {
    id: i32,
    name: String,                    // Internal: "pasta", "pasta_integrale", "ricotta"
    display_name: String,            // User-facing: "Pasta", "Pasta Integrale", "Ricotta"
    category: TagCategory,           // ingredient, dietary, prep_time, other
    weekly_suggestion: Option<i32>,  // Suggested frequency (not enforced)
    parent_tag_id: Option<i32>,      // For hierarchies: pasta_integrale -> pasta
    created_at: DateTime,
}

enum TagCategory {
    Ingredient,   // pasta, ricotta, eggs
    Dietary,      // vegetarian, vegan, gluten-free
    PrepTime,     // quick_prep, slow_cook
    Other,        // custom tags
}
```

**Key Features:**

1. **No Typos**: Tags must exist in database before use (referential integrity)
2. **Hierarchies**: Child tags (pasta_integrale) can reference parent tags (pasta)
3. **Soft Suggestions**: Weekly suggestions are recommendations, not hard limits
4. **Multiple Tags**: Each meal option can have many tags
5. **Categories**: Separate ingredient tracking from dietary/prep classifications

**Example Tag Hierarchy:**

```
pasta (weekly_suggestion: 3)
  └── pasta_integrale (weekly_suggestion: 2, parent: pasta)

// If user eats pasta_integrale:
// - Counts toward "pasta_integrale": 1/2
// - Also counts toward "pasta": 1/3
```

**Weekly Frequency Tracking:**

- `weekly_tag_usage` view tracks tag usage per week
- Frontend displays warnings when approaching suggestions
- User can proceed despite warnings (soft enforcement)

### 2.4 Database Schema (SQLite)

```sql
-- Level 1: Meal Templates (categories/groups)
CREATE TABLE meal_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    tags TEXT, -- JSON array
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Level 2: Meal Options (individual meals)
CREATE TABLE meal_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    template_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL, -- 'breakfast', 'lunch', 'dinner', 'snack'
    location_type TEXT NOT NULL, -- 'home', 'office', 'restaurant', 'any'
    weekly_limit INTEGER, -- NULL for unlimited
    nutritional_notes TEXT,
    compatible_slots TEXT NOT NULL, -- JSON array of slot types
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (template_id) REFERENCES meal_templates(id) ON DELETE CASCADE
);

-- Level 4: Meal Entries (meal planning and logging)
-- Supports both planned meals (future) and logged meals (past/completed)
CREATE TABLE meal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    meal_option_id INTEGER NOT NULL,
    date DATE NOT NULL,
    slot_type TEXT NOT NULL, -- 'breakfast', 'morning_snack', 'lunch', etc.
    location TEXT NOT NULL,
    servings REAL NOT NULL DEFAULT 1.0, -- Nutrition plan uses strict serving sizes
    notes TEXT,
    completed BOOLEAN DEFAULT FALSE, -- FALSE = planned, TRUE = consumed
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (meal_option_id) REFERENCES meal_options(id) ON DELETE RESTRICT
);

-- Weekly usage tracking (for enforcing limits)
CREATE VIEW weekly_meal_usage AS
SELECT
    meal_option_id,
    strftime('%Y-%W', date) as week,
    COUNT(*) as usage_count
FROM meal_entries
WHERE completed = TRUE
GROUP BY meal_option_id, week;

-- Indexes for performance
CREATE INDEX idx_meal_entries_date ON meal_entries(date);
CREATE INDEX idx_meal_entries_option ON meal_entries(meal_option_id);
CREATE INDEX idx_meal_options_template ON meal_options(template_id);
CREATE INDEX idx_meal_options_category ON meal_options(category);
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

#### 2. **Meal Selection Modal (Two-Level Navigation)**

**Step 1: Template Selection**

- **Layout**: Grid of template cards
- **Content**: Template names with preview of options count
- **Filters**:
  - Filter by tags
  - Show only templates with compatible options for current slot
- **Search**: Search across template and option names

**Step 2: Option Selection** (after clicking a template)

- **Layout**: List or grid of meal options within selected template
- **Content**:
  - Option name, description
  - Location badge (home/office/restaurant)
  - Compatibility indicator (which slots this option can fill)
  - Weekly limit indicator (e.g., "1/2 this week")
- **States**:
  - Disabled state for weekly-limited options that are exhausted
  - Highlight options compatible with current slot
- **Actions**:
  - "Quick add" button on each option
  - Back button to return to template selection

**Alternative: Quick Search View**

- **Use Case**: "I want soup!" - search-first approach
- **Layout**: Flat list of ALL options across all templates
- **Search bar**: Search by option name
- **Results show**:
  - Option name
  - Template name (e.g., "Tomato Soup" from "Soups")
  - Compatible slots indicator
  - Ability to select slot if not already chosen

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

**Scenario 1: Planning Tomorrow's Breakfast (Slot-First Approach)**

1. User opens app → sees today's view
2. Clicks date picker → selects tomorrow
3. Clicks empty "Breakfast" slot
4. Modal opens showing meal templates
5. User browses and clicks "Breakfast Bowls" template
6. Modal shows options: "Oatmeal with Berries", "Greek Yogurt Bowl", "Granola Bowl"
7. User selects "Oatmeal with Berries" (shows Office location, compatible with Breakfast slot)
8. Adjusts servings to 1.2 (nutrition plan uses strict serving sizes, default is 1.0)
9. Adds note: "Extra blueberries"
10. Saves → card appears in breakfast slot showing "Oatmeal with Berries" from "Breakfast Bowls"

**Scenario 2: Finding a Meal (Meal-First Approach)**

1. User thinks "I want soup today!"
2. Opens app, clicks search icon
3. Types "soup" in global search
4. App shows all soup options:
   - "Tomato Soup" from "Soups" template (Lunch/Dinner, Home)
   - "Chicken Noodle" from "Soups" template (Lunch/Dinner, Home/Office)
   - "Miso Soup" from "Japanese Meals" template (Lunch, Restaurant)
5. Each result shows compatible slots: Lunch ✓, Dinner ✓, Breakfast ✗
6. User selects "Tomato Soup"
7. App asks "Which slot?" → user selects "Lunch"
8. Meal is added to today's lunch slot

**Scenario 3: Logging Actual Meal**

1. User ate lunch, opens app
2. Today's lunch slot shows planned "Chicken Caesar Salad" (from "Salads" template)
3. Clicks card → detail editor opens
4. Marks "Completed" checkbox
5. Adjusts location from "Office" to "Restaurant" (changed plans)
6. Adds note: "Had extra avocado"
7. Saves → card shows completed state (checkmark badge)
8. Weekly limit counter updates: "Chicken Caesar: 1/2 this week"

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

**Note on Development Order**: Phases are numbered according to the original user-journey plan, but actual development follows a more pragmatic order. Phase 5 (Templates Manager) is being implemented after Phase 2 to enable testing and real data usage.

**Actual Development Order**: Phase 0 → Phase 1 → Phase 2 → **Phase 5** → Phase 3 → Phase 4 → Phase 6 → Phase 7

---

### Phase 0: Project Setup & Scaffolding ✅ (COMPLETED)

**Goal**: Get development environment ready with comprehensive testing infrastructure

- [x] Initialize Tauri project with React + TypeScript
- [x] Set up Tailwind CSS and UI component library
- [x] Configure project structure (folders, modules)
- [x] Add Rust dependencies (sqlx, serde, tokio, chrono)
- [x] Create initial database schema and migrations
- [x] Set up development tooling (TypeScript, Tailwind)
- [x] Create README with setup instructions
- [x] Verify IPC communication works
- [x] Generate app icons
- [x] Document Linux graphics workaround
- [x] Document comprehensive testing strategy
- [x] Add testing dependencies (dev-dependencies)
- [x] Set up CI/CD pipeline (GitHub Actions)
- [x] Configure Vitest for frontend testing

**Testing Setup:**

- Backend testing tools: tokio-test, tempfile (for test databases) ✅
- Frontend testing tools: Vitest, React Testing Library ✅
- Test structure and conventions documented ✅
- CI runs tests on every push ✅

**Deliverable**: Running "Hello World" Tauri app with IPC communication tested and testing infrastructure ready

**Status**: ✅ **COMPLETE** - All scaffolding, database schema, and testing infrastructure in place. CI/CD operational.

---

### Phase 1: Core Data Layer ✅ (COMPLETED)

**Goal**: Build the backend foundation with comprehensive test coverage

**Backend Tasks:**

- [x] Implement database models (MealTemplate, MealOption, MealEntry)
  - MealTemplate struct with tags ✅
  - MealOption struct with template_id and compatible_slots ✅
  - MealEntry struct with meal_option_id ✅
  - All enums (SlotType, LocationType, TagCategory) ✅
- [x] Create repository layer for CRUD operations
  - TagRepository (CRUD + get by category + get children) ✅
  - MealTemplateRepository (CRUD + search + filter) ✅
  - MealOptionRepository (CRUD + search + tag management) ✅
  - MealEntryRepository (CRUD + date queries + weekly usage) ✅
- [x] Implement Tauri commands for:
  - Tags: 8 commands (get_all, get_by_id, get_by_name, get_by_category, get_children, create, update, delete) ✅
  - Templates: 8 commands (get_all, get_by_id, search, get_by_location, get_by_slot, create, update, delete) ✅
  - Options: 12 commands (get_all, get_by_id, get_with_tags, get_by_template, search, create, update, delete, add_tags, remove_tags, set_tags) ✅
  - Entries: 13 commands (get_by_id, get_by_date, get_by_date_range, get_by_date_and_slot, get_by_completed, get_by_meal_option, get_weekly_usage, get_weekly_tag_usage, create, update, delete, validate) ✅
  - **Total: 41 Tauri commands** ✅
- [x] Add weekly limit validation logic
  - ValidationService with template limit checking ✅
  - Tag suggestion warnings ✅
  - Slot compatibility validation ✅
  - Entry validation ✅

**Testing Tasks (Comprehensive Approach):**

- [x] Write unit tests for data models (30+ tests) ✅
  - Model validation logic ✅
  - Enum conversions and serialization ✅
  - Business rule validation ✅
- [x] Write integration tests for repository layer (40+ tests) ✅
  - CRUD operations with test database ✅
  - Query accuracy (date ranges, filtering) ✅
  - Transaction handling ✅
  - Error cases (foreign keys, constraints) ✅
- [x] Write tests for database layer (5 tests) ✅
  - Schema creation and migrations ✅
  - Index functionality ✅
  - View queries (weekly_meal_usage, weekly_tag_usage) ✅
  - SQLite-specific behavior ✅
- [x] Write tests for business logic services (13 tests) ✅
  - Weekly limit calculation ✅
  - Week start validation (Monday) ✅
  - Tag suggestions (soft warnings) ✅
  - Location-based filtering ✅
- [x] Write tests for Tauri commands (33+ tests) ✅
  - IPC serialization/deserialization ✅
  - Error handling and propagation ✅
  - Command parameter validation ✅
- [x] Write integration tests for IPC boundary (12 tests) ✅
  - Verify all types cross IPC boundary correctly ✅
  - Test error serialization ✅
- [x] Set up test utilities and helpers ✅
  - Test database factory ✅
  - Mock data builders ✅
  - Common assertions ✅

**Frontend Tasks:**

- [x] Set up TypeScript types matching Rust models (src/lib/types.ts) ✅
- [x] Create API client wrapper for Tauri commands (src/lib/api.ts) ✅
  - 41 typed wrapper functions ✅
  - Comprehensive JSDoc documentation ✅
  - Type-safe error handling ✅
- [x] Write frontend tests (18 tests) ✅
  - API wrapper tests ✅
  - Type guard tests ✅

**Test Coverage Achieved:**

- **Backend: 121 tests passing** (109 unit/integration + 12 IPC tests)
- **Frontend: 18 tests passing** (17 API tests + 1 type test)
- **Backend Coverage: 85%+** ✅ (Target met)
- **Frontend Coverage: 30%+** (pattern-based, all functions follow same structure)

**Deliverable**: Fully tested backend API and frontend integration with confidence

**Status**: ✅ **COMPLETE** - All 41 commands implemented, tested, and accessible from TypeScript. Full type safety across IPC boundary.

---

### Phase 2: Basic Daily View ✅ (COMPLETED)

**Goal**: Users can view and add meals to today

**Tasks:**

- [x] Create MealSlot component (empty and filled states) ✅
- [x] Create MealCard component (visual display) ✅
- [x] Build daily timeline layout ✅
- [x] Implement date selector (today navigation) ✅
- [x] Create simple meal selection modal (list view) ✅
- [x] Implement add meal flow (select → save) ✅
- [x] Display filled slots with meal info ✅

**Deliverable**: Can add and view meals for any given day

**Status**: ✅ **COMPLETE** - Full daily view with date navigation, meal selection modals, complete add meal flow, and real-time data display from database.

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

### Phase 5: Templates Manager (Implemented after Phase 2) ✅ **COMPLETE**

**Goal**: Users can manage their meal options

**Note**: Implemented this phase early (after Phase 2) to enable testing and populate the database with real meal data.

**Status**: Completed November 12, 2025

**Tasks Completed:**

- [x] **Task 1**: Create templates list view with search/filter by location
- [x] **Task 2**: Build template form (create/edit) with validation
- [x] **Task 3**: Implement CRUD operations for templates
- [x] **Task 4**: Build options form (create/edit) with tag selection
- [x] **Task 5**: Implement CRUD operations for options
- [x] **Task 6**: Add template/option relationship management
- [x] **Task 7**: Add tags management (create/assign tags) with hierarchy support
- [x] **Task 8**: Implement weekly limit configuration (templates and tags)
- [x] **Task 9**: Add template/option search/filter capabilities
- [x] **Bonus**: Custom ConfirmDialog component for better UX
- [x] **Bonus**: Allow weekly_suggestion = 0 for tags (meaning "avoid if possible")

**Components Created:**

- `TemplatesView`: Main templates manager with search/filter
- `TemplateForm`: Modal for creating/editing templates
- `OptionsView`: View for managing meal options within a template
- `OptionForm`: Modal for creating/editing options with tag selection
- `TagsView`: Hierarchical tag manager
- `TagForm`: Modal for creating/editing tags
- `ConfirmDialog`: Reusable confirmation modal

**Features:**

- Full CRUD operations for templates, options, and tags
- Hierarchical tag display (parent-child relationships)
- Weekly suggestion tracking (with special "avoid if possible" display for 0)
- Search and filter functionality
- Location-based filtering
- Custom confirmation dialogs
- Escape key support for modals
- Loading and error states

**Database Changes:**

- Migration 20251112165422: Allow weekly_suggestion >= 0 (previously > 0)

**Testing:**

- All 25 tag-related tests passing
- Manual UI testing confirmed all features working

**Deliverable**: ✅ Complete self-service template management

**Remaining Task:**

- [ ] Task 10: Add proper routing (React Router) to replace temporary navigation

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

**Comprehensive Testing Approach:**

We adopt a rigorous testing strategy from Phase 1 onwards to ensure code quality, maintainability, and confidence in the application.

**Backend Testing (Phase 1+):**

- **Unit Tests**: All business logic, models, and utility functions
  - Inline tests using `#[cfg(test)]` modules
  - Fast, isolated tests with no dependencies
  - Target: 90%+ coverage for models and services
- **Integration Tests**: Repository and database operations
  - Located in `src-tauri/tests/` directory
  - Use temporary SQLite databases for isolation
  - Test real database interactions, queries, and transactions
  - Target: 85%+ coverage for repository layer
- **Database Tests**: Schema, migrations, and constraints
  - Verify migrations apply correctly
  - Test foreign keys, indexes, and views
  - Validate data integrity rules

**Frontend Testing (Phase 7):**

- **Unit Tests**: Utility functions, helpers, and store logic
  - Tool: Vitest (fast, Vite-native)
  - Target: 80%+ coverage for utilities
- **Component Tests**: React components in isolation
  - Tool: React Testing Library + Vitest
  - Test rendering, interactions, and state changes
  - Target: 70%+ coverage for components
- **E2E Tests**: Complete user workflows
  - Tool: Playwright or Tauri testing utilities
  - Test critical paths (add meal, weekly planning, template management)
  - Target: 100% coverage for critical user journeys

**Test Infrastructure:**

- Automated tests run before commits (git hooks)
- CI/CD pipeline runs full test suite (Phase 7)
- Code coverage reports generated automatically
- Test databases are isolated and cleaned between tests

**Testing Tools:**

- **Backend**: tokio-test, tempfile, sqlx testing features
- **Frontend**: Vitest, @testing-library/react, @testing-library/jest-dom
- **E2E**: Playwright
- **Coverage**: cargo-tarpaulin (Rust), vitest coverage (TypeScript)

### Code Review Checkpoints

- After each phase completion
- Before adding new major features
- When refactoring significant code

---

## 9. Testing Guidelines & Best Practices

### 9.1 Backend Testing Standards

**Test Organization:**

```
src-tauri/
├── src/
│   ├── models/
│   │   └── meal_template.rs  # Unit tests inline with #[cfg(test)]
│   ├── services/
│   │   └── meal_service.rs   # Unit tests inline
│   └── repository/
│       └── meal_repo.rs      # Unit tests for pure logic
└── tests/
    ├── integration/
    │   ├── repository_tests.rs
    │   └── command_tests.rs
    ├── db_tests/
    │   ├── migrations_test.rs
    │   └── schema_test.rs
    └── helpers/
        ├── mod.rs
        ├── test_db.rs        # Test database utilities
        └── fixtures.rs       # Test data builders
```

**Writing Backend Tests:**

```rust
// Unit Test Example (inline)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meal_category_from_string() {
        assert_eq!(MealCategory::from_str("breakfast").unwrap(), MealCategory::Breakfast);
    }

    #[test]
    fn test_weekly_limit_validation() {
        let limit = Some(2);
        assert!(validate_weekly_limit(limit, 1));
        assert!(!validate_weekly_limit(limit, 3));
    }
}

// Integration Test Example (tests/ directory)
#[tokio::test]
async fn test_create_meal_template() {
    let db = create_test_database().await;
    let repo = MealTemplateRepository::new(db);

    let template = MealTemplate {
        name: "Test Meal".to_string(),
        category: MealCategory::Breakfast,
        // ... other fields
    };

    let id = repo.create(template).await.unwrap();
    assert!(id > 0);

    let retrieved = repo.get_by_id(id).await.unwrap();
    assert_eq!(retrieved.name, "Test Meal");
}
```

**Test Database Utilities:**

```rust
// tests/helpers/test_db.rs
pub async fn create_test_database() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

pub fn create_test_meal_template() -> MealTemplate {
    MealTemplate {
        id: None,
        name: "Test Breakfast".to_string(),
        category: MealCategory::Breakfast,
        location_type: LocationType::Home,
        weekly_limit: Some(2),
        tags: vec!["test".to_string()],
        // ... other fields with sensible defaults
    }
}
```

### 9.2 Frontend Testing Standards (Phase 7)

**Test Organization:**

```
src/
├── components/
│   └── meals/
│       ├── MealCard.tsx
│       └── MealCard.test.tsx
├── lib/
│   ├── utils.ts
│   └── utils.test.ts
└── __tests__/
    ├── integration/
    └── e2e/
```

**Writing Frontend Tests:**

```typescript
// Unit Test Example
import { describe, it, expect } from "vitest";
import { calculateWeekStart } from "./utils";

describe("Date utilities", () => {
  it("calculates week start as Monday", () => {
    const thursday = new Date("2025-11-06");
    const weekStart = calculateWeekStart(thursday);
    expect(weekStart.getDay()).toBe(1); // Monday
    expect(weekStart.getDate()).toBe(3); // Nov 3rd
  });
});

// Component Test Example
import { render, screen, fireEvent } from "@testing-library/react";
import { MealCard } from "./MealCard";

describe("MealCard", () => {
  it("renders meal information", () => {
    const meal = createTestMeal();
    render(<MealCard meal={meal} />);

    expect(screen.getByText("Oatmeal")).toBeInTheDocument();
    expect(screen.getByText("Home")).toBeInTheDocument();
  });

  it("calls onClick when clicked", () => {
    const handleClick = vi.fn();
    const meal = createTestMeal();
    render(<MealCard meal={meal} onClick={handleClick} />);

    fireEvent.click(screen.getByRole("button"));
    expect(handleClick).toHaveBeenCalledWith(meal);
  });
});
```

### 9.3 Test Coverage Goals

| Component           | Target Coverage | Priority |
| ------------------- | --------------- | -------- |
| Backend Models      | 90%+            | Critical |
| Backend Repository  | 85%+            | Critical |
| Backend Services    | 85%+            | Critical |
| Backend Commands    | 80%+            | High     |
| Database Schema     | 100%            | Critical |
| Frontend Utils      | 80%+            | High     |
| Frontend Components | 70%+            | Medium   |
| Frontend Stores     | 75%+            | High     |
| E2E Critical Paths  | 100%            | Critical |

### 9.4 Running Tests

**Backend Tests:**

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_meal_template

# Run integration tests only
cargo test --test integration

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

**Frontend Tests (Phase 7):**

```bash
# Run all tests
npm test

# Run in watch mode
npm test -- --watch

# Run with coverage
npm test -- --coverage

# Run specific test file
npm test MealCard.test.tsx
```

### 9.5 Testing Principles

1. **Test Behavior, Not Implementation**: Focus on what the code does, not how
2. **Arrange-Act-Assert**: Structure tests clearly with setup, action, and verification
3. **One Assertion Per Concept**: Each test should verify one logical concept
4. **Fast and Isolated**: Tests should run quickly and not depend on each other
5. **Descriptive Names**: Test names should clearly describe what they verify
6. **Test the Happy Path and Edge Cases**: Cover normal usage and error conditions
7. **Use Test Data Builders**: Create reusable functions for test data
8. **Clean Up After Tests**: Ensure tests don't leave artifacts or state

### 9.6 Integration Testing Strategy (Tauri-Specific)

**Key Lessons Learned from Phase 1:**

1. **Don't Fight the Framework**

   - Tauri State is hard to mock in integration tests; use it where it's natural (command module tests)
   - Command modules are the perfect place for end-to-end integration tests
   - Keep integration tests in separate files focused on what matters

2. **Test What Can Actually Break**

   - In Tauri apps, the critical integration point is the IPC boundary
   - IPC serialization failures are runtime disasters that won't be caught by type checking
   - Focus integration tests on serialization/deserialization of all types that cross IPC

3. **Avoid Test Duplication**

   - If command modules already have comprehensive integration tests (database, state, persistence), don't duplicate them
   - Leverage existing test coverage and add complementary tests
   - Document where integration coverage exists to avoid confusion

4. **Complementary Test Suites**
   - **Command module tests**: Full end-to-end flows (Command → Repository → Database)
     - Use real `tauri::State`
     - Test actual database operations
     - Verify business logic
   - **Separate integration tests**: IPC serialization verification
     - Ensure all types can cross IPC boundary
     - Verify JSON roundtrips
     - Test enum conversions
     - No database needed (fast)

**Our Integration Testing Approach:**

```
Command Module Tests (109 tests)          Integration Tests (12 tests)
- Full stack integration                  - IPC serialization focus
- Database operations                     - Type safety across boundary
- Business logic validation               - JSON roundtrips
- Error handling                          - Enum conversions
- Use tauri::State                        - Fast, no DB setup
```

**Benefits:**

- ✅ No duplication
- ✅ Targets real risks (IPC failures)
- ✅ Fast test execution
- ✅ Clear, focused purpose
- ✅ Easy to maintain
- ✅ Documents architecture decisions

### 9.7 Continuous Integration (Phase 7)

**GitHub Actions Workflow:**

- Run tests on every push and PR
- Generate coverage reports
- Fail build if coverage drops below threshold
- Run clippy (Rust linter) and TypeScript checks
- Test on multiple platforms (Windows, macOS, Linux)

---

## 10. Open Questions & Decisions Needed

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

## 11. Success Metrics

### Phase Completion Criteria:

- ✅ All phase tasks completed
- ✅ No critical bugs
- ✅ Code reviewed and refactored
- ✅ **Comprehensive tests written and passing**
- ✅ **Test coverage meets phase targets**
- ✅ Documentation updated

### Final Release Criteria:

- Can manage 50+ meal templates smoothly
- Can view/edit meals for any date
- Weekly limits enforced correctly
- Data persists between app restarts
- Intuitive UX (minimal learning curve)
- Stable (no crashes in normal usage)
- **80%+ overall test coverage achieved**
- **All critical user paths covered by E2E tests**
- **No known security vulnerabilities**
- **Performance meets targets (< 100ms for common operations)**

---

## 12. Next Steps

Now that we have this plan, we can:

1. **Review and refine** this document together
2. **Make decisions** on open questions
3. **Start Phase 0**: Initialize the Tauri project
4. **Create initial scaffolding**: Set up the basic project structure

Let's discuss any changes you'd like to make before we begin coding! 🚀
