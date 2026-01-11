<p align="center">
  <a href="https://getchloe.sh">
    <img src="docs/public/logos/logo-with-name.svg" alt="Chloe" width="340">
  </a>
</p>

<p align="center">
  Chloe (a wordplay on "Claude TUI") is a terminal-based task management application built with Rust. It combines a kanban board for tracking work with integrated terminal instances, letting you manage multiple Claude Code sessions in parallel while maintaining visibility and control over what each instance is doing.
</p>

<p align="center">
  <a href="https://github.com/KevinEdry/chloe/actions/workflows/ci.yml">
    <img src="https://github.com/KevinEdry/chloe/actions/workflows/ci.yml/badge.svg" alt="CI">
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
</p>

<p align="center">
  <img src="docs/public/demo.gif" alt="Chloe Demo" width="800">
</p>

<details>
<summary>Table of Contents</summary>

- [Features](#features)
- [Installation](#installation)
- [Contributing](#contributing)
- [Acknowledgements](#acknowledgements)
- [License](#license)

</details>

## Features

**Task Management** - Macro-level task viewer with kanban board and task list views. Track work across To Do, In Progress, and Done states with persistent storage.

**Interactive Instances** - Multiple terminal panes with full PTY support. Run any command, switch between panes with keyboard shortcuts.

**Roadmap View** - Visualize project milestones and plan work across time.

**Worktrees** - Manage git worktrees for parallel development on multiple branches.

## Installation

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
```

The binary will be at `target/release/chloe`.

## Data Storage

State is stored in `.chloe/state.json` in your project directory, including tasks, instances, roadmap items, and settings.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

This project uses [Conventional Commits](https://www.conventionalcommits.org/) and includes CI checks for formatting, linting, and tests.

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

## Acknowledgements

- [Auto-Claude](https://github.com/AndyMik90/Auto-Claude) by [@AndyMik90](https://github.com/AndyMik90) - Inspiration and implementation guidelines
- [Ratatui](https://github.com/ratatui/ratatui) - Terminal UI framework

## License

MIT License - see [LICENSE](./LICENSE) for details.
