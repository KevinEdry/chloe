# Architecture Documentation

## System Overview

Chloe is built using a modular architecture with clear separation of concerns between state management, event handling, and UI rendering.

## Safety Policy

**STRICT NO UNSAFE CODE POLICY**

This codebase maintains the highest safety standards:

### Enforced Rules
1. **No `unsafe` blocks** - `#![forbid(unsafe_code)]` at crate root
2. **No unsafe threading** - All concurrency uses safe primitives (channels, Arc, Mutex)
3. **Safe dependencies** - All external crates must expose safe APIs
4. **Static verification** - Enforced by Rust compiler and clippy

### Why This Matters
- **Eliminates memory bugs**: No use-after-free, buffer overflows, or dangling pointers
- **Prevents data races**: Thread safety guaranteed by type system
- **Audit simplicity**: No need to review unsafe blocks
- **Production confidence**: Entire classes of CVEs impossible

### Dependency Safety Audit
All dependencies use safe Rust:
- ✅ `ratatui` - Safe terminal UI framework
- ✅ `crossterm` - Safe terminal backend
- ✅ `portable-pty` - Safe PTY abstraction (encapsulates OS unsafe code)
- ✅ `vt100` - Safe terminal parser
- ✅ `serde` - Safe serialization
- ✅ `uuid`, `chrono`, `dirs` - All safe utilities

**Note**: While some dependencies internally use `unsafe` (like `portable-pty` for OS syscalls), their **public APIs are safe** and we never call unsafe functions directly.

## Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER INPUT                                │
│                  (Keyboard, Mouse Events)                        │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
              ┌────────────────────────┐
              │   main.rs: Event Loop   │
              │   - Poll crossterm      │
              │   - Route to app        │
              └────────────┬────────────┘
                           │
                           ▼
              ┌────────────────────────┐
              │  app.rs: State Router   │
              │  - Check active tab     │
              │  - Delegate to feature  │
              └─────────┬───┬───────────┘
                        │   │
               ┌────────┘   └────────┐
               │                     │
               ▼                     ▼
     ┌─────────────────┐   ┌──────────────────┐
     │ Kanban Events   │   │ Terminal Events  │
     │ - Handle keys   │   │ - Route to PTY   │
     │ - Mutate state  │   │ - Mutate state   │
     └────────┬────────┘   └────────┬─────────┘
              │                     │
              ▼                     ▼
     ┌─────────────────┐   ┌──────────────────┐
     │ Kanban State    │   │ Terminal State   │
     │ - Tasks         │   │ - PTY sessions   │
     │ - Selection     │   │ - VT100 parsers  │
     │ - Mode          │   │ - Layout         │
     └────────┬────────┘   └────────┬─────────┘
              │                     │
              └──────────┬──────────┘
                         │
                         ▼
           ┌─────────────────────────┐
           │   ui/mod.rs: Dispatcher  │
           │   - Render tab bar       │
           │   - Route to feature UI  │
           └──────────┬───────────────┘
                      │
                      ▼
         ┌────────────────────────────┐
         │   Feature UI Renderers      │
         │   - kanban/ui.rs            │
         │   - terminal/ui.rs          │
         └──────────┬──────────────────┘
                    │
                    ▼
         ┌────────────────────────────┐
         │    ratatui: Frame Buffer    │
         │    crossterm: Terminal      │
         └─────────────────────────────┘
```

## State Management

### Root State

```rust
// app.rs
pub struct App {
    pub active_tab: Tab,
    pub kanban: KanbanState,
    pub terminals: TerminalState,
    pub config: Config,
    pub error: Option<String>,
}
```

### Kanban State

```rust
// kanban/state.rs
pub struct KanbanState {
    columns: Vec<Column>,      // ["To Do", "In Progress", "Done"]
    selected_column: usize,
    selected_task: Option<usize>,
    mode: KanbanMode,
}

pub enum KanbanMode {
    Normal,                    // Navigation mode
    AddingTask(String),        // Adding new task (partial input)
    EditingTask(usize, String),// Editing existing task
}
```

### Terminal State

```rust
// terminal/state.rs
pub struct TerminalState {
    panes: Vec<TerminalPane>,
    active_pane: usize,
    layout: LayoutMode,
}

pub struct TerminalPane {
    id: Uuid,
    pty: PtySession,           // Live PTY session
    parser: vt100::Parser,     // VT100 screen parser
    working_dir: PathBuf,
    scroll_offset: usize,
}

pub enum LayoutMode {
    Single,
    Horizontal,  // Side-by-side
    Vertical,    // Stacked
}
```

## Event Handling

### Event Flow

1. **Polling**: `main.rs` polls crossterm for events every 50-100ms
2. **Global Handling**: Check for global keys (Tab switch, Quit, Help)
3. **Tab Routing**: Route event to active tab's event handler
4. **State Mutation**: Event handler mutates feature state
5. **Re-render**: UI is re-rendered based on new state

### Kanban Events

```rust
// kanban/events.rs
pub fn handle_key_event(state: &mut KanbanState, key: KeyEvent) -> Option<Action> {
    match state.mode {
        KanbanMode::Normal => {
            // Navigation: arrow keys
            // Actions: a (add), e (edit), d (delete), Enter (move)
        }
        KanbanMode::AddingTask(ref mut input) |
        KanbanMode::EditingTask(_, ref mut input) => {
            // Text input: append chars, backspace, Enter (save), Esc (cancel)
        }
    }
}
```

### Terminal Events

```rust
// terminal/events.rs
pub fn handle_key_event(state: &mut TerminalState, key: KeyEvent) -> Option<Action> {
    // Special keys: Ctrl+N (new pane), Ctrl+W (close), Ctrl+] (next)
    // All other keys: forward to active PTY session

    let pane = &mut state.panes[state.active_pane];
    pane.pty.write_input(&convert_key_to_bytes(key))?;
}
```

## UI Rendering

### Rendering Pipeline

1. **Dispatcher**: `ui/mod.rs` renders tab bar and routes to feature renderer
2. **Feature Renderer**: `kanban/ui.rs` or `terminal/ui.rs` renders feature-specific UI
3. **Widgets**: Shared widgets from `ui/widgets/` used as needed
4. **Styling**: Colors and styles from `ui/styles.rs`

### Kanban UI Layout

```
┌─────────────────────────────────────────────────┐
│         [Kanban] | Terminals                     │ ← Tab Bar
├─────────────────────────────────────────────────┤
│ To Do         │ In Progress   │ Done            │ ← Columns
│ ┌───────────┐ │ ┌───────────┐ │ ┌───────────┐ │
│ │ Task 1    │ │ │ Task 3    │ │ │ Task 5    │ │
│ └───────────┘ │ └───────────┘ │ └───────────┘ │
│ ┌───────────┐ │               │ ┌───────────┐ │
│ │ Task 2    │ │               │ │ Task 6    │ │
│ └───────────┘ │               │ └───────────┘ │
│               │               │               │
├─────────────────────────────────────────────────┤
│ [Mode: Normal] a:add e:edit d:delete ?:help     │ ← Status Bar
└─────────────────────────────────────────────────┘
```

### Terminal UI Layout

```
┌─────────────────────────────────────────────────┐
│         Kanban | [Terminals]                     │ ← Tab Bar
├───────────────────────┬─────────────────────────┤
│ Terminal 1 (active)   │ Terminal 2              │ ← Panes
│ $ ls                  │ $ top                   │
│ file1.rs             │ PID  CMD                │
│ file2.rs             │ 1234 bash               │
│ $ _                   │                         │
│                       │                         │
├───────────────────────┴─────────────────────────┤
│ ^N:new ^W:close ^]:next ^H/V:layout ?:help      │ ← Status Bar
└─────────────────────────────────────────────────┘
```

## Persistence Layer

### Serialization

```rust
// persistence/storage.rs

pub fn save_state(app: &App) -> Result<()> {
    let path = get_config_path()?;
    let serializable = SerializableApp {
        kanban: app.kanban.clone(),
        terminals: extract_terminal_metadata(&app.terminals),
    };
    let json = serde_json::to_string_pretty(&serializable)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn load_state() -> Result<App> {
    let path = get_config_path()?;
    let json = std::fs::read_to_string(path)?;
    let data: SerializableApp = serde_json::from_str(&json)?;

    // Reconstruct app with new PTY sessions
    Ok(App {
        kanban: data.kanban,
        terminals: recreate_terminals(data.terminals)?,
        ..Default::default()
    })
}
```

### What Gets Persisted

**Kanban (Full Serialization)**:
- All tasks with full details
- Column structure
- Selection state

**Terminals (Metadata Only)**:
- Number of panes
- Working directory per pane
- Layout mode

**Not Persisted**:
- Live PTY sessions (recreated on load)
- VT100 parser state (cleared on load)
- Terminal scrollback (lost on restart)

## Terminal Emulation

### PTY Management

```
┌──────────────────────────────────────┐
│     TerminalPane                     │
│  ┌────────────────────────────────┐ │
│  │  PtySession                    │ │
│  │  ┌──────────────────────────┐ │ │
│  │  │  Spawned Shell Process   │ │ │
│  │  │  (bash, zsh, etc.)       │ │ │
│  │  └───────┬──────────────────┘ │ │
│  │          │                    │ │
│  │    ┌─────▼──────┬──────┐     │ │
│  │    │ PTY Master │ Slave│     │ │
│  │    └─────┬──────┴───┬──┘     │ │
│  │          │          │        │ │
│  └──────────┼──────────┼────────┘ │
│             │          │          │
│         Write Input  Read Output  │
│             │          │          │
│  ┌──────────▼──────────▼────────┐ │
│  │   vt100::Parser              │ │
│  │   - Parse VT100 sequences    │ │
│  │   - Build screen buffer      │ │
│  │   - Track cursor position    │ │
│  └──────────────────────────────┘ │
└──────────────────────────────────────┘
```

### VT100 Parsing

```rust
// terminal/ui.rs

fn convert_vt100_row(row: &vt100::Row) -> Line {
    let cells = row.cells();
    let mut spans = Vec::new();

    for cell in cells {
        let style = Style::default()
            .fg(convert_vt100_color(cell.fgcolor()))
            .bg(convert_vt100_color(cell.bgcolor()))
            .add_modifier(if cell.bold() { Modifier::BOLD } else { Modifier::empty() });

        spans.push(Span::styled(cell.contents(), style));
    }

    Line::from(spans)
}
```

## Thread Model

Currently single-threaded with polling:
- Main thread runs event loop
- PTY output polled on every render cycle
- Works well for typical CLI programs

**Future**: Consider async/threading for:
- Heavy PTY output (build systems, logs)
- Non-blocking state saves
- Background tasks

## Error Handling

```rust
// types/errors.rs

pub enum AppError {
    Io(std::io::Error),
    Serialization(serde_json::Error),
    Pty(String),
    Config(String),
}

// Errors propagate up to main.rs
// Displayed in UI as notification bar
// Never panic in normal operation
```

## Performance Considerations

- **Render frequency**: 50-100ms poll interval balances responsiveness and CPU usage
- **PTY buffering**: Read up to 4KB per poll to avoid blocking
- **State cloning**: Minimize clones; use references where possible
- **VT100 parsing**: Happens only when terminal output changes
- **Serialization**: Auto-save debounced to avoid excessive disk I/O
