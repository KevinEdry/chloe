# Tasks - Toggle View Modes
# Shows switching between Focus and Kanban views

Output ../public/docs/tasks-view-toggle.gif
Output ../public/docs/tasks-view-toggle.webm

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

# Add some tasks first
Type "a"
Sleep 300ms
Type "First task"
Enter
Sleep 2s

Type "a"
Sleep 300ms
Type "Second task"
Enter
Sleep 2s

# Show Focus view (default)
Sleep 1s

# Toggle to Kanban view
Type "/"
Sleep 2s

# Toggle back to Focus view
Type "/"
Sleep 2s
