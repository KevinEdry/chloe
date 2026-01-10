# Instances - Focused Mode
# Shows entering focused mode and typing commands

Output ../public/docs/instances-focused.gif
Output ../public/docs/instances-focused.webm

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

# Switch to Instances tab
Type "2"
Sleep 1s

# Create a pane if needed
Type "c"
Sleep 1s

# Enter focused mode
Enter
Sleep 500ms

# Type commands
Type "echo 'Now in focused mode - all input goes to terminal'"
Enter
Sleep 1s

Type "ls -la"
Enter
Sleep 1s

Type "date"
Enter
Sleep 1s

# Exit focused mode
Escape
Sleep 1s

# Show navigation works again
Right
Sleep 300ms
Left
Sleep 500ms
