<p align="center">
  <a href="https://getchloe.sh">
    <img src="docs/public/logos/logo-with-name.svg" alt="Chloe" width="340">
  </a>
</p>

<h3 align="center">
  The terminal-native AI agent orchestrator.<br>
  <em>Manage multiple AI coding sessions. Zero bloat.</em>
</h3>

<p align="center">
  <a href="https://github.com/KevinEdry/chloe/actions/workflows/ci.yml">
    <img src="https://github.com/KevinEdry/chloe/actions/workflows/ci.yml/badge.svg" alt="CI">
  </a>
  <a href="https://github.com/KevinEdry/chloe/releases">
    <img src="https://img.shields.io/github/v/release/KevinEdry/chloe?label=version" alt="Version">
  </a>
  <a href="https://github.com/KevinEdry/chloe/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  </a>
  <a href="https://discord.gg/Pqdb9ZGvVV">
    <img src="https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white" alt="Discord">
  </a>
  <a href="https://github.com/KevinEdry/chloe">
    <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg" alt="Unsafe Forbidden">
  </a>
  <a href="https://github.com/KevinEdry/chloe">
    <img src="https://img.shields.io/badge/rust-stable-orange.svg" alt="Rust">
  </a>
</p>

<p align="center">
  <img src="docs/public/demo.gif" alt="Chloe Demo" width="800">
</p>

---

## Why Chloe?

AI coding agents are powerful, but managing multiple sessions is chaos. You're juggling terminal tabs, losing context, and watching your system slow to a crawl under Electron-based IDEs.

**Chloe fixes this.**

| | Chloe | Electron-based IDEs |
|---|:---:|:---:|
| **Memory footprint** | ~15 MB | 500+ MB |
| **Startup time** | Instant (<100ms) | 3-10 seconds |
| **UI latency** | Sub-millisecond | Noticeable lag |
| **Distribution** | Single 5MB binary | Hundreds of MB |
| **Dependencies** | None | Node.js, Chromium |
| **Offline-first** | Yes | Varies |

### Built for Power Users

- **Terminal-native**: Stays in your workflow. No context switching.
- **Multi-agent orchestration**: Run Claude Code, Gemini CLI, Amp, or OpenCode in parallel panes.
- **Kanban + terminals**: See what each agent is working on while watching their output.
- **Git & Jujutsu support**: Each task gets its own worktree (Git) or workspace (Jujutsu), isolated and ready.
- **100% safe Rust**: Memory safety guaranteed. No undefined behavior. Ever.

---

## Features

### Multi-Provider Support

Works with the AI coding agents you already use:

| Provider | Status |
|----------|--------|
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | Supported |
| [Gemini CLI](https://github.com/google-gemini/gemini-cli) | Supported |
| [Amp](https://ampcode.com/) | Supported |
| [OpenCode](https://opencode.ai/) | Supported |

Chloe auto-detects installed providers and lets you choose which one to use for each task.

### Task Management

Macro-level visibility into your work:
- **Kanban board**: To Do, In Progress, Done columns
- **Task list view**: Dense view for many tasks
- **Persistent state**: Pick up where you left off

### Interactive Terminal Panes

Full PTY support means real terminal emulation:
- Split panes horizontally or vertically
- Keyboard-driven navigation (vim-style)
- Watch agent output in real-time

### Roadmap View

Plan work across milestones. Visualize what's coming, what's blocked, and what's done.

### Git Worktrees & Jujutsu Workspaces

Parallel development without branch switching:
- Each task can have its own worktree (Git) or workspace (Jujutsu)
- Choose your preferred version control system in Settings
- Isolated environments for each agent
- No stash/checkout dance
- Chloe adapts UI terminology based on your VCS choice

---

## Quick Start

### Install

```bash
curl -fsSL getchloe.sh/install | bash
```

Or specify a version:

```bash
curl -fsSL getchloe.sh/install | bash -s v0.1.1
```

### Build from Source

```bash
git clone https://github.com/KevinEdry/chloe.git
cd chloe
cargo build --release --locked
./target/release/chloe
```

### Run

```bash
chloe
```

That's it. No configuration required.

---

## How It Compares

| Feature | Chloe | Cursor | Windsurf | Terminal + tmux |
|---------|:-----:|:------:|:--------:|:---------------:|
| Multi-agent orchestration | Yes | No | No | Manual |
| Task tracking built-in | Yes | No | No | No |
| Memory usage | ~15 MB | ~500 MB | ~400 MB | ~5 MB |
| Terminal-native | Yes | No | No | Yes |
| Single binary | Yes | No | No | Yes |
| Provider agnostic | Yes | No | No | Yes |
| Git & Jujutsu support | Yes | No | No | Manual |
| Offline-capable | Yes | Partial | Partial | Yes |

**Chloe fills a gap**: It's like tmux met a kanban board and learned to speak to AI agents.

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1-4` | Switch tabs |
| `j/k` | Navigate up/down |
| `h/l` | Navigate left/right |
| `Enter` | Select/confirm |
| `a` | Add task |
| `d` | Delete |
| `?` | Help |
| `q` | Quit |

---

## Data Storage

State is stored in `.chloe/state.json` in your project directory:
- Tasks and their status
- Instance configurations
- Roadmap items
- Settings

All data stays local. No cloud sync. No telemetry.

---

## Contributing

Contributions welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

This project uses:
- [Conventional Commits](https://www.conventionalcommits.org/)
- CI checks for formatting (`cargo fmt`), linting (`cargo clippy`), and tests

### Code Standards

- **100% safe Rust**: `#![forbid(unsafe_code)]` enforced
- **No abbreviations**: Full words in identifiers
- **No magic numbers**: Named constants only
- **Max 2 levels of nesting**: Early returns required

---

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/KevinEdry"><img src="https://avatars.githubusercontent.com/KevinEdry?v=4&s=100" width="100px;" alt="Kevin Edry"/><br /><sub><b>Kevin Edry</b></sub></a><br /><a href="https://github.com/KevinEdry/chloe/commits?author=KevinEdry" title="Code">💻</a> <a href="https://github.com/KevinEdry/chloe/commits?author=KevinEdry" title="Documentation">📖</a> <a href="#maintenance-KevinEdry" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://allcontributors.org) specification.

---

## Acknowledgements

- [Auto-Claude](https://github.com/AndyMik90/Auto-Claude) by [@AndyMik90](https://github.com/AndyMik90) - Inspiration
- [Ratatui](https://github.com/ratatui/ratatui) - Terminal UI framework

---

## License

MIT License - see [LICENSE](./LICENSE) for details.
