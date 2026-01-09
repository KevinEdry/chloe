# Contributing to Chloe

Thank you for your interest in contributing to Chloe! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions. We welcome contributors of all backgrounds and experience levels.

## Getting Started

### Prerequisites

- Rust 1.70+ (2024 edition)
- tmux 2.0+ (optional, for persistent sessions)

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/chloe.git
   cd chloe
   ```
3. Build and run:
   ```bash
   cargo build
   cargo run
   ```

## Development Workflow

### Before Making Changes

1. Create a new branch for your work:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Read [CLAUDE.md](./CLAUDE.md) for architecture details and code quality standards

### Code Quality Standards

This project maintains strict code quality standards:

#### Safety Policy

- **100% Safe Rust** - No `unsafe` code allowed anywhere
- The codebase uses `#![forbid(unsafe_code)]` to enforce this

#### Code Style

- **No abbreviations** - Use full words (`message` not `msg`, `configuration` not `cfg`)
- **Early returns** - Maximum 2 levels of nesting
- **No magic numbers** - All numeric literals must be named constants
- **Comments explain "why", never "what"** - Code should be self-documenting
- **Code locality** - Keep related code together in the same module

#### Running Quality Checks

```bash
# Format code
cargo fmt

# Run linter (must pass with no warnings)
cargo clippy -- -D warnings

# Run tests
cargo test

# Verify no unsafe code
grep -r "unsafe" src/
```

### Making Changes

1. Make your changes following the code quality standards
2. Add tests for new functionality
3. Ensure all checks pass:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
4. Commit your changes with a descriptive message

### Commit Messages

This project uses [Conventional Commits](https://www.conventionalcommits.org/). All commits must follow this format:

```
<type>(<optional scope>): <description>

[optional body]

[optional footer(s)]
```

**Allowed types:**

| Type       | Description                                      |
|------------|--------------------------------------------------|
| `feat`     | New feature                                      |
| `fix`      | Bug fix                                          |
| `docs`     | Documentation only changes                       |
| `style`    | Code style changes (formatting, no logic change) |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf`     | Performance improvement                          |
| `test`     | Adding or correcting tests                       |
| `build`    | Changes to build system or dependencies          |
| `ci`       | Changes to CI configuration                      |
| `chore`    | Other changes that don't modify src or test files|
| `revert`   | Reverts a previous commit                        |

**Examples:**

```
feat(kanban): add task priority field

- Add priority enum to Task struct
- Update UI to display priority indicators
- Add keybinding to cycle priority
```

```
fix(instance): correct PTY resize handling
```

```
docs: update installation instructions
```

Commits are automatically linted in CI using [committed](https://github.com/crate-ci/committed)

### Submitting Changes

1. Push your branch to your fork
2. Create a Pull Request against `main`
3. Fill out the PR template with:
   - Description of changes
   - Any breaking changes
   - Screenshots for UI changes
4. Wait for CI checks to pass
5. Address any review feedback

## Types of Contributions

### Bug Reports

- Use the GitHub issue tracker
- Include steps to reproduce
- Include expected vs actual behavior
- Include system information (OS, Rust version)

### Feature Requests

- Open an issue describing the feature
- Explain the use case and benefits
- Be open to discussion about implementation

### Code Contributions

- Bug fixes
- New features
- Documentation improvements
- Test coverage improvements
- Performance optimizations

### Documentation

- README improvements
- Code comments (remember: only "why", not "what")
- Architecture documentation

## Project Structure

```
src/
├── types/         # Shared types (errors, config)
├── kanban/        # Kanban feature (logic + UI)
├── instance/      # Instance feature (logic + UI)
├── ui/            # Shared UI components
├── persistence/   # State serialization
└── common/        # Shared utilities
```

See [CLAUDE.md](./CLAUDE.md) for detailed architecture documentation.

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Writing Tests

- Add unit tests in the same file as the code
- Use descriptive test names
- Test edge cases and error conditions

## Getting Help

- Open an issue for bugs or questions
- Check existing issues before creating new ones
- Be patient - maintainers review contributions as time allows

## Recognition

All contributors are recognized in the README using the [all-contributors](https://allcontributors.org/) specification. Contributions of all types are valued!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
