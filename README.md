# Chloe - Auto Claude CLI

A powerful terminal-based CLI application built with Rust that provides kanban task management and multiple interactive instance panes.

## Features

### 📋 Kanban Board
- Traditional task board with **To Do**, **In Progress**, and **Done** columns
- Add, edit, delete, and move tasks between columns
- Persistent task storage across sessions
- Task metadata (creation timestamps, descriptions)

### 💻 Interactive Instances
- Multiple instance panes in split view
- Full interactive PTY sessions - run any command
- **Advanced Session Management** with tmux backend:
  - Sessions persist across app restarts
  - Detach/reattach sessions without losing state
  - Background processes survive app closure
  - Resume exactly where you left off
- Horizontal and vertical layouts
- Switch between panes with keyboard shortcuts
- Session naming and metadata support

### 🎨 Modern TUI Interface
- Tab-based navigation
- Context-sensitive help overlay
- Color-coded UI with visual feedback
- Status bar showing current mode and keybindings

### 🔒 Safety Guarantees
- **100% Safe Rust** - Zero unsafe code (`#![forbid(unsafe_code)]`)
- **Memory Safe** - Compiler-enforced memory safety
- **Thread Safe** - No data races or unsafe threading
- **Statically Verified** - All safety properties checked at compile time

## Installation

### Prerequisites
- Rust 1.70+ (2024 edition support)
- tmux 2.0+ (optional, required for persistent sessions)

### Build from Source

```bash
git clone https://github.com/yourusername/chloe.git
cd chloe
cargo build --release
```

The binary will be available at `target/release/chloe`.

### Run Directly

```bash
cargo run
```

## Usage

### Basic Navigation

- **Tab** or **1-2**: Switch between Kanban and Instances tabs
- **q** or **Ctrl+C**: Quit application
- **?**: Show help overlay

### Kanban Tab

**Navigation:**
- **←/→**: Switch between columns
- **↑/↓**: Select task within column

**Actions:**
- **a**: Add new task
- **e**: Edit selected task
- **d**: Delete selected task
- **Enter**: Move task to next column
- **Backspace**: Move task to previous column

**Text Entry Mode:**
- Type to add text
- **Enter**: Save
- **Esc**: Cancel

### Instances Tab

**Pane Management:**
- **c**: Create new direct PTY pane (not persistent)
- **t**: Create new tmux-backed pane (persistent)
- **x**: Close active pane
- **Arrow Keys**: Navigate between panes
- **Enter**: Focus on pane (all keys sent to instance)
- **Esc**: Exit focus mode (return to navigation)

**Session Management:**
- **d**: Detach from current tmux session (keeps it running)
- **a**: Reattach to detached tmux session
- Session status indicators:
  - **[📎]**: Attached to tmux session
  - **[💤]**: Detached (session running in background)
  - **[💀]**: Session terminated

**Instance Input:**
- When focused, all keyboard input is forwarded to the instance
- Special key sequences (Ctrl+C, arrow keys, etc.) work correctly

## Configuration

Configuration file: `~/.config/chloe/config.toml`

```toml
theme = "Dark"  # or "Light"
default_shell = "/bin/zsh"  # or your preferred shell
auto_save_interval_secs = 30
```

State file: `~/.config/chloe/state.json` (auto-generated)

## Architecture

Chloe follows a modular architecture with code locality principles:

```
src/
├── types/         # Shared types (errors, config)
├── kanban/        # Kanban feature (logic + UI)
├── instance/      # Instance feature (logic + UI)
├── ui/            # Shared UI components
├── persistence/   # State serialization
└── common/        # Shared utilities
```

See [claude.md](./claude.md) for detailed architecture documentation, safety policy, and code quality standards.

## Development

### Format Code

```bash
cargo fmt
```

### Run Linter

```bash
cargo clippy
```

### Run Tests

```bash
cargo test
```

## Known Limitations

- Terminal emulation doesn't support full VT100 spec (no images, limited Unicode)
- Some TUI programs (vim, emacs) may not render perfectly in direct PTY mode
- Direct PTY sessions don't persist across app restarts (use tmux-backed panes for persistence)
- Maximum practical instance panes: 4-6 before UI becomes crowded
- tmux must be installed for session persistence features

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal backend
- [portable-pty](https://github.com/wez/wezterm/tree/main/pty) - PTY support
- [vt100](https://github.com/doy/vt100-rust) - VT100 terminal emulator
- [serde](https://github.com/serde-rs/serde) - Serialization

## License

MIT License - see LICENSE file for details

## Contributing

Contributions welcome! Please see [docs/development.md](./docs/development.md) for guidelines.

## Author

Built with ❤️ using Rust and Claude Code
