# Documentation Tape Files

These tape files are used with [VHS](https://github.com/charmbracelet/vhs) to generate GIFs and WebM videos for the documentation.

## Prerequisites

Install VHS:

```bash
# macOS
brew install vhs

# Other platforms
go install github.com/charmbracelet/vhs@latest
```

## Generate All GIFs

Run all tape files to generate the documentation assets:

```bash
# From the docs/tapes directory
for tape in *.tape; do
  echo "Processing $tape..."
  vhs "$tape"
done
```

Or generate a specific GIF:

```bash
vhs tasks-create.tape
```

## Output Location

GIFs and WebM files are output to `../public/docs/`.

## Tape Files

### Tasks Tab
- `tasks-create.tape` - Creating a new task
- `tasks-start.tape` - Starting a task (moving to In Progress)
- `tasks-terminal-focus.tape` - Using terminal focus mode
- `tasks-view-toggle.tape` - Switching between Focus and Kanban views

### Instances Tab
- `instances-create.tape` - Creating a new terminal pane
- `instances-focused.tape` - Entering focused mode and typing
- `instances-layouts.tape` - Different pane layouts

### Roadmap Tab
- `roadmap-create.tape` - Creating roadmap items
- `roadmap-priority.tape` - Changing item priorities
- `roadmap-to-task.tape` - Converting roadmap item to task
- `roadmap-generate.tape` - AI roadmap generation

### Worktree Tab
- `worktree-navigate.tape` - Navigating worktrees
- `worktree-open.tape` - Opening worktree in IDE

### Pull Requests Tab
- `pull-requests-navigate.tape` - Navigating PRs
- `pull-requests-refresh.tape` - Refreshing PR list
- `pull-requests-open.tape` - Opening PR in browser

## Customization

Each tape file uses these common settings:

```tape
Set FontSize 16
Set Width 1000
Set Height 600
Set Theme "Catppuccin Mocha"
Set Padding 10
Set Framerate 30
Set PlaybackSpeed 1
```

Adjust these values if needed for consistency with the main demo.
