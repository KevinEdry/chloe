# Chloe - Auto Claude CLI

## Project Overview

Chloe is a terminal-based CLI application that mimics Auto Claude functionality, providing:
- **Kanban Board**: Task management with To Do, In Progress, and Done columns
- **Interactive Terminals**: Multiple terminal panes running actual shell sessions
- **Persistent State**: Tasks and terminal configurations saved across sessions

## Safety Guarantees

**This project maintains a STRICT NO UNSAFE CODE policy:**

- ✅ **100% Safe Rust** - No `unsafe` blocks anywhere in the codebase
- ✅ **Thread Safety** - All concurrency patterns use safe Rust primitives
- ✅ **Memory Safety** - Compiler-enforced memory safety guarantees
- ✅ **Static Enforcement** - `#![forbid(unsafe_code)]` prevents any unsafe code
- ✅ **Dependency Audit** - Only dependencies with safe public APIs

This eliminates entire classes of bugs:
- No use-after-free
- No buffer overflows
- No data races
- No null pointer dereferences
- No undefined behavior

All dependencies (ratatui, crossterm, portable-pty, vt100) use safe Rust interfaces.

**For complete safety policy details, see [docs/safety.md](./docs/safety.md)**

## Code Quality Standards

### Comments: Only "Why", Never "How" or "What"

**Golden Rule**: Code should be self-documenting. Comments explain *why*, never *what* or *how*.

```rust
// ❌ BAD: "What" comment - code already says this
// Increment the counter
counter += 1;

// ❌ BAD: "How" comment - code already shows how
// Loop through items and find matching ID
for item in items {
    if item.id == target_id {
        return Some(item);
    }
}

// ✅ GOOD: "Why" comment - explains business logic
// Task must move to previous column before deletion to maintain audit trail
move_task_previous();
delete_task();
```

**If you need a "how" or "what" comment, REFACTOR instead:**

```rust
// ❌ BAD: Needs comment to explain
// Parse the timestamp and convert to local timezone
let dt = chrono::DateTime::parse_from_rfc3339(&s)
    .map(|d| d.with_timezone(&chrono::Local))?;

// ✅ GOOD: Function name makes it clear
let dt = parse_timestamp_as_local(&s)?;
```

**When "why" comments are appropriate:**
- Business logic rationale: "We use base64 to avoid URL encoding issues"
- Performance trade-offs: "Caching here reduces API calls by 90%"
- Non-obvious algorithms: "Binary search chosen for O(log n) lookup on sorted data"
- Workarounds: "Clippy false positive on this pattern, see issue #123"
- Safety invariants: "SAFETY: This assumes the buffer is always initialized"

### No Abbreviations

**All identifiers must use full words, not abbreviations.**

```rust
// ❌ BAD: Abbreviations
let cfg = Config::default();
let msg = "Hello";
let idx = 0;
let btn = Button::new();
let ctx = AppContext::new();

// ✅ GOOD: Full words
let config = Config::default();
let message = "Hello";
let index = 0;
let button = Button::new();
let context = AppContext::new();
```

**Exceptions (industry standard abbreviations only):**
- `id` (identifier) - universally understood
- `url` (Uniform Resource Locator) - more common than "address"
- `html`, `json`, `xml` - file format names
- `io` (input/output) - standard library convention
- `uuid` (Universally Unique Identifier) - standard acronym
- `pty` (pseudo-terminal) - standard Unix term

**Common violations to avoid:**
- ❌ `num` → ✅ `number` or `count`
- ❌ `str` → ✅ `string` (except `&str` type)
- ❌ `arr` → ✅ `array`
- ❌ `btn` → ✅ `button`
- ❌ `msg` → ✅ `message`
- ❌ `tmp` → ✅ `temporary`
- ❌ `val` → ✅ `value`
- ❌ `cfg` → ✅ `config`
- ❌ `ctx` → ✅ `context`
- ❌ `doc` → ✅ `document`
- ❌ `img` → ✅ `image`

**Why this matters:**
- Code is read 10x more than written
- Abbreviations are ambiguous (`msg` = message or messages?)
- IDEs have autocomplete - no typing savings
- Newcomers understand full words instantly
- Consistency > brevity

**Refactoring over Comments:**
If you're writing a comment to explain what code does, the code is too complex. Refactor by:
1. Extract to well-named functions
2. Use descriptive variable names
3. Simplify complex expressions
4. Break long functions into smaller pieces

### Never Nester: Return Early

Avoid deep nesting - return early and keep the happy path at the lowest indentation:

```rust
// ❌ BAD: Deep nesting
if let Some(task) = task {
    if task.is_valid() {
        if let Some(assignee) = task.assignee {
            // do work
        }
    }
}

// ✅ GOOD: Early returns
let task = task.ok_or("No task")?;
if !task.is_valid() {
    return Err("Invalid task".into());
}
let assignee = task.assignee.ok_or("No assignee")?;
// do work - happy path at lowest indentation
```

**Rule: Maximum nesting depth is 2 levels.**

### Functional Core, Imperative Shell

Separate pure logic from side effects:

```rust
// FUNCTIONAL CORE: Pure business logic
fn calculate_discount(user: &User, amount: f64) -> f64 {
    // Pure function - no I/O, no mutation
    if user.is_premium && amount > 100.0 {
        amount * 0.2
    } else {
        0.0
    }
}

// IMPERATIVE SHELL: Side effects at edges
fn apply_discount_and_save(user_id: Uuid, amount: f64) -> Result<()> {
    let user = database.load(user_id)?;          // I/O
    let discount = calculate_discount(&user, amount); // Pure
    database.save_transaction(discount)?;         // I/O
    Ok(())
}
```

**Benefits:**
- Pure functions are easy to test (no mocks)
- Logic is reusable across different contexts
- Side effects are isolated and explicit
- Easier to reason about code behavior

See [docs/code-style.md](./docs/code-style.md) for complete guidelines.

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────┐
│  main.rs - Entry point & event loop            │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  app.rs - Root state coordinator               │
│  - Active tab tracking                          │
│  - State aggregation                            │
│  - Event routing                                │
└──────┬────────────────────────┬─────────────────┘
       │                        │
┌──────▼──────┐          ┌──────▼──────┐
│ Kanban      │          │ Terminals   │
│ Feature     │          │ Feature     │
├─────────────┤          ├─────────────┤
│ • state.rs  │          │ • state.rs  │
│ • events.rs │          │ • events.rs │
│ • ops.rs    │          │ • pty.rs    │
│ • ui.rs     │          │ • layout.rs │
│             │          │ • ui.rs     │
└─────────────┘          └─────────────┘
```

### Module Structure

**Core Principle**: **Code Locality** - Keep feature-specific logic and UI together in the same module.

#### `src/types/` - Shared Types
- `errors.rs`: Custom error types and Result alias
- `config.rs`: Application configuration

#### `src/kanban/` - Kanban Feature
- `state.rs`: KanbanState, Task, Column, KanbanMode enums
- `events.rs`: Keyboard event handlers for kanban interactions
- `operations.rs`: Business logic (add/edit/delete/move tasks)
- `ui.rs`: **Kanban-specific rendering** (stays with feature)

#### `src/terminal/` - Terminal Feature
- `state.rs`: TerminalState, TerminalPane, LayoutMode
- `events.rs`: Event routing to PTY sessions
- `pty.rs`: PTY lifecycle management using portable-pty
- `layout.rs`: Pane splitting algorithms
- `ui.rs`: **Terminal-specific rendering** (stays with feature)

#### `src/ui/` - Shared UI Components
- `mod.rs`: Main UI dispatcher and tab bar rendering
- `widgets/`: Reusable widgets across features
  - `input_dialog.rs`: Text input popup
  - `help_overlay.rs`: Context-sensitive help
  - `status_bar.rs`: Bottom status display
- `styles.rs`: Centralized color schemes

#### `src/persistence/` - State Management
- `storage.rs`: JSON serialization/deserialization
- `paths.rs`: Config directory resolution
- `migrations.rs`: Schema version handling

#### `src/common/` - Shared Utilities
- `input.rs`: InputMode enum (Normal, TextEntry, Terminal)
- `keybindings.rs`: Centralized keybinding configuration

## Key Design Decisions

### 1. Code Locality
Each feature module (`kanban/`, `terminal/`) contains **both** its logic AND its specific UI rendering code. This makes the codebase easier to navigate and maintain - everything related to a feature lives in one place.

### 2. Event Routing Strategy
```
User Input → main.rs → app.rs → [Kanban|Terminal]::events → State Mutation → UI Rendering
```

Events flow from the main loop to the active tab's event handler, which mutates state. The UI is then re-rendered based on the new state.

### 3. Terminal Emulation Approach
- Use `portable-pty` for cross-platform PTY support
- Use `vt100` crate for VT100 terminal parsing
- Poll PTY for output on every render cycle (not just on events)
- Cannot serialize live PTY sessions - only persist metadata

### 4. Persistence Strategy
- Use JSON for state serialization (easier for nested structures than TOML)
- Save to `~/.config/chloe/state.json`
- Auto-save on state changes + on graceful shutdown
- Kanban tasks fully serializable
- Terminal panes: only save working directory and layout

## Navigation Guide

### Finding Code

**Need to add a new keybinding?**
- Global: `src/common/keybindings.rs`
- Feature-specific: `src/kanban/events.rs` or `src/terminal/events.rs`

**Need to change kanban UI rendering?**
- `src/kanban/ui.rs` (NOT in src/ui/)

**Need to change terminal UI rendering?**
- `src/terminal/ui.rs` (NOT in src/ui/)

**Need to add a shared widget?**
- `src/ui/widgets/` (e.g., for confirmation dialogs, modals)

**Need to change color scheme?**
- `src/ui/styles.rs`

**Need to modify state persistence?**
- `src/persistence/storage.rs`

### Common Tasks

**Add a new task field in Kanban:**
1. Update `Task` struct in `src/kanban/state.rs`
2. Update serialization (add `#[serde(...)]` if needed)
3. Update UI rendering in `src/kanban/ui.rs`
4. Update add/edit logic in `src/kanban/operations.rs`

**Add a new terminal pane layout:**
1. Add variant to `LayoutMode` enum in `src/terminal/state.rs`
2. Implement layout logic in `src/terminal/layout.rs`
3. Update rendering in `src/terminal/ui.rs`
4. Add keybinding in `src/terminal/events.rs`

**Add a new tab:**
1. Add variant to `Tab` enum in `src/app.rs`
2. Create new feature module under `src/`
3. Add routing in `src/ui/mod.rs` dispatcher
4. Update keybindings in `src/main.rs`

## Dependencies

- **ratatui**: Terminal UI framework
- **crossterm**: Terminal backend and event handling
- **portable-pty**: Cross-platform PTY support
- **vt100**: VT100 terminal emulator/parser
- **serde**: Serialization framework
- **uuid**: Unique identifiers for tasks/panes
- **chrono**: Timestamp handling
- **anyhow**: Error handling
- **dirs**: Cross-platform config directory

## Development Workflow

1. **Make changes** in the appropriate feature module
2. **Run** `cargo fmt` to format code
3. **Run** `cargo clippy` to check for issues
4. **Test** the application with `cargo run`
5. **Commit** changes with descriptive messages

## Known Limitations

- Terminal emulation doesn't support full VT100 spec (advanced features like images)
- Some TUI programs (vim, emacs) may not render perfectly
- PTY sessions don't persist across app restarts (only working directory)
- Maximum practical number of terminal panes: ~4-6 (UI becomes crowded)

## Future Enhancements

- [ ] Theme customization via config file
- [ ] Task priorities and tags
- [ ] Task due dates and reminders
- [ ] Terminal scrollback buffer improvements
- [ ] Split terminal panes dynamically
- [ ] Search/filter tasks in kanban
- [ ] Export kanban board to markdown/CSV
