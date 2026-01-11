# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-10

### Bug Fixes

- Resolve clippy pedantic and nursery warnings by @KevinEdry
- Resolve Rust 1.92 clippy warnings and add docs site by @KevinEdry
- Correct committed.toml config format by @KevinEdry
- Defer MDX theme components initialization for Vercel build by @KevinEdry
- Update release workflow for macos-15 and remove musl target by @KevinEdry
- Checkout main branch in changelog workflow by @KevinEdry
- Update docs links to point to overview page by @KevinEdry
- Disable nextra git timestamp for vercel compatibility by @KevinEdry
- Revert invalid nextra config option by @KevinEdry
- Use newline instead of carriage return when submitting prompts by @KevinEdry
- Allow typing 'q' in dialog input fields by @KevinEdry
- Store worktrees inside repository at .chloe/worktrees/ by @KevinEdry

### Build

- **deps:** Bump crossterm from 0.28.1 to 0.29.0 by @dependabot[bot]
- **deps:** Bump git2 from 0.18.3 to 0.20.3 by @dependabot[bot]
- **deps:** Bump portable-pty from 0.8.1 to 0.9.0 by @dependabot[bot]
- **deps:** Bump ratatui from 0.29.0 to 0.30.0 by @dependabot[bot]

### CI/CD

- Add GitHub workflows, issue templates, and PR template by @KevinEdry
- Add crates.io auto-publish to release workflow by @KevinEdry
- Use macos-13 for x86_64 builds to avoid cross-compilation by @KevinEdry
- Remove redundant unsafe grep check (compiler lint handles this) by @KevinEdry
- Use macos-13 for x86_64 builds in CI workflow by @KevinEdry
- Bump actions/download-artifact from 4 to 7 by @dependabot[bot]
- Bump actions/upload-artifact from 4 to 6 by @dependabot[bot]
- Bump stefanzweifel/git-auto-commit-action from 5 to 7 by @dependabot[bot]
- Bump actions/checkout from 4 to 6 by @dependabot[bot]
- Use macos-latest for x86_64 builds (macos-13 retired) by @KevinEdry
- Use macos-15-intel for x86_64 builds (macos-13 retired) by @KevinEdry
- Checkout PR head to avoid merge commit in lint by @KevinEdry

### Documentation

- Update README and add CONTRIBUTING and CHANGELOG by @KevinEdry
- Update CHANGELOG.md for v0.1.0 by @KevinEdry
- Add tailwind css configuration by @KevinEdry
- Add landing page components by @KevinEdry
- Add demo video and logo assets by @KevinEdry
- Add demo gif to README by @KevinEdry
- Add Vercel configuration by @KevinEdry
- Update CHANGELOG.md for v0.1.1 by @KevinEdry
- Add installation one-liner and remove usage section by @KevinEdry
- Add installation command to landing page hero by @KevinEdry
- Simplify hero CTA to Install button with copy + Get started link by @KevinEdry
- Swap Install/Get started button styles by @KevinEdry
- Show install command instead of Install button by @KevinEdry
- Restructure documentation with Nextra built-in components by @KevinEdry
- Add demo recordings for documentation by @KevinEdry
- Add feature documentation pages by @KevinEdry
- Update documentation navigation and content by @KevinEdry
- Update persistence paths and remove completed enhancement by @KevinEdry

### Features

- Add Pull Requests tab for PR management by @KevinEdry
- Add robust installation script with checksum verification by @KevinEdry
- Add getchloe.sh/install.sh redirect by @KevinEdry
- Add sitemap and robots.txt generation by @KevinEdry
- Add open graph image by @KevinEdry
- Add faq section by @KevinEdry
- Add comprehensive seo metadata and open graph tags by @KevinEdry
- Add json-ld structured data and faq section to landing page by @KevinEdry
- Add vercel analytics by @KevinEdry
- Track install command copy events by @KevinEdry
- Add --version flag and dynamic version display by @KevinEdry
- Enhanced review workflow with status display and flexible merge target by @KevinEdry
- Auto-grow task input dialog for longer text by @KevinEdry
- Run AI classification in background for task creation by @KevinEdry
- Map Shift+Esc to Esc in terminal panes by @KevinEdry
- Instruct AI to discover and follow repo commit standards by @KevinEdry
- Replace scrollwheel with vim-style scroll in terminal panes by @KevinEdry
- Add Settings tab with JSON persistence by @KevinEdry
- Prevent unclassified tasks from moving to In Progress by @KevinEdry
- Implement dynamic terminal pane splitting with tree-based layout by @KevinEdry
- **terminal:** Add scroll offset support to Screen trait by @KevinEdry
- **tasks:** Add terminal scroll mode by @KevinEdry
- **tasks:** Add scroll mode UI indicators by @KevinEdry

### Miscellaneous

- Add git-cliff, committed, and all-contributors configs by @KevinEdry
- Add docs/.next/ to gitignore by @KevinEdry
- Increase commit subject max length to 100 by @KevinEdry
- Add VHS demo recording script by @KevinEdry
- Update demo recording script timing by @KevinEdry
- Bump version to 0.1.1 and remove crates.io publish by @KevinEdry
- Add vhs tape scripts for demo recordings by @KevinEdry
- Add next-sitemap dependency by @KevinEdry
- Bump version to v0.2.0 and fix clippy warnings by @KevinEdry

### Refactoring

- Simplify tab bar to show numbered tabs by @KevinEdry
- Split hero into smaller components by @KevinEdry
- Remove unused code to fix compiler warnings by @KevinEdry
- Fix clippy warnings by @KevinEdry
- **settings:** Remove theme configuration option by @KevinEdry
- **terminal:** Add alacritty Screen/Cell implementation by @KevinEdry
- **pty:** Rewrite using alacritty_terminal tty module by @KevinEdry
- **instances:** Remove custom scrollback buffer by @KevinEdry
- **instances:** Update rendering for alacritty_terminal by @KevinEdry
- **app:** Update for alacritty_terminal changes by @KevinEdry

### Styling

- Update sidebar and callout styles by @KevinEdry
- Apply rustfmt formatting fixes by @KevinEdry
- Apply cargo fmt formatting by @KevinEdry

### Deps

- Replace vt100 and portable-pty with alacritty_terminal by @KevinEdry

<!-- generated by git-cliff -->
