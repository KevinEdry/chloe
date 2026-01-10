# Tasks - Terminal Focus Mode
# Shows entering terminal focus and interacting

Output ../public/docs/tasks-terminal-focus.gif
Output ../public/docs/tasks-terminal-focus.webm

Set FontSize 16
Set Width 1000
Set Height 600
Set Theme "Catppuccin Mocha"
Set Padding 10
Set Framerate 30
Set PlaybackSpeed 1

# Start the application
Type "chloe"
Enter
Sleep 2s

# Create and start a task
Type "a"
Sleep 300ms
Type "Test terminal focus"
Enter
Sleep 2s

Type "s"
Sleep 500ms
Enter
Sleep 2s

# Navigate to In Progress task
Type "l"
Sleep 500ms

# Enter terminal focus mode
Enter
Sleep 1s

# Type some commands in the terminal
Type "echo 'Hello from terminal focus mode!'"
Enter
Sleep 1s

Type "pwd"
Enter
Sleep 1s

# Exit focus mode with Escape
Escape
Sleep 1s

# Show we're back to navigation
Type "h"
Sleep 500ms
