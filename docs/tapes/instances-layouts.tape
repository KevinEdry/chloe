# Instances - Multiple Layouts
# Shows different pane layouts

Output ../public/docs/instances-layouts.gif
Output ../public/docs/instances-layouts.webm

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

# Create first pane
Type "c"
Sleep 1s

# Create second pane (horizontal split)
Type "c"
Sleep 1s

# Navigate between panes
Right
Sleep 500ms
Left
Sleep 500ms

# Create third pane
Type "c"
Sleep 1s

# Create fourth pane (grid)
Type "c"
Sleep 1s

# Navigate around the grid
Right
Sleep 300ms
Right
Sleep 300ms
Left
Sleep 300ms
Left
Sleep 500ms
