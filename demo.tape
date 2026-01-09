# Chloe Demo - Auto Claude CLI
# A comprehensive demo showcasing the TUI features

Output demo.gif
Output demo.webm

Set FontSize 16
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set Padding 10
Set Framerate 30
Set PlaybackSpeed 1

# Start the application
Type "chloe"
Enter
Sleep 3s

# ============================================
# TASKS TAB - Kanban Board
# ============================================

# Show we're on the Tasks tab (default)
Sleep 1s

# Add a new task
Type "a"
Sleep 500ms
Type "Implement user authentication"
Enter
Sleep 8s

# Add another task
Type "a"
Sleep 500ms
Type "Write API documentation"
Enter
Sleep 8s

# Add a third task
Type "a"
Sleep 500ms
Type "Fix login bug"
Enter
Sleep 8s

# Navigate tasks with vim keys
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "k"
Sleep 500ms

# Move a task to In Progress (Enter)
Enter
Sleep 1s
Enter
Sleep 1s

# Navigate to the task in In Progress column
Type "l"
Sleep 500ms

# ============================================
# INSTANCES TAB - Terminal Panes
# ============================================

# Switch to Instances tab
Type "2"
Sleep 1s

# Create a new terminal pane
Type "c"
Sleep 1s

# Focus the pane to type
Enter
Sleep 1s

# Run a simple command
Type "echo 'Hello from Chloe!'"
Enter
Sleep 1s

Type "ls -la"
Enter
Sleep 1s

# Exit focus mode
Escape
Sleep 500ms

# Create another pane
Type "c"
Sleep 1s

# Navigate between panes
Right
Sleep 300ms
Left
Sleep 500ms

# ============================================
# ROADMAP TAB - Feature Planning
# ============================================

# Switch to Roadmap tab
Type "3"
Sleep 1s

# Add a roadmap item
Type "a"
Sleep 500ms
Type "Multi-user support"
Enter
Sleep 800ms

# Add another item
Type "a"
Sleep 500ms
Type "Cloud sync feature"
Enter
Sleep 800ms

# Navigate and change priority
Type "j"
Sleep 300ms
Type "p"
Sleep 500ms
Type "p"
Sleep 500ms

# ============================================
# WORKTREE TAB - Git Worktrees
# ============================================

# Switch to Worktree tab
Type "4"
Sleep 1.5s

# Navigate worktrees (if any)
Type "j"
Sleep 300ms
Type "k"
Sleep 500ms

# ============================================
# TAB NAVIGATION DEMO
# ============================================

# Quick tab switching with numbers
Type "1"
Sleep 800ms
Type "2"
Sleep 800ms
Type "3"
Sleep 800ms
Type "4"
Sleep 800ms

# Cycle through tabs with Tab key
Tab
Sleep 500ms
Tab
Sleep 500ms
Tab
Sleep 500ms
Tab
Sleep 500ms

# ============================================
# EXIT
# ============================================

# Return to Tasks
Type "1"
Sleep 1s

# Quit the application
Type "q"
Sleep 500ms
Type "y"
Sleep 1s
