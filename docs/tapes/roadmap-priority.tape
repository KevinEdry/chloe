# Roadmap - Change Priority
# Shows cycling through priorities

Output ../public/docs/roadmap-priority.gif
Output ../public/docs/roadmap-priority.webm

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

# Switch to Roadmap tab
Type "3"
Sleep 1s

# Add an item
Type "a"
Sleep 500ms
Type "Performance optimization"
Enter
Sleep 1s

# Cycle priority: Medium -> Low
Type "p"
Sleep 800ms

# Cycle priority: Low -> High
Type "p"
Sleep 800ms

# Cycle priority: High -> Medium
Type "p"
Sleep 800ms
