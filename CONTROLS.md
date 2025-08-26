# 3D L-Systems Controls & Usage Guide

Complete reference for all controls, features, and usage patterns.

## Quick Start

1. **Launch**: `cargo run --release`
2. **Default View**: Cherry Blossom tree loads automatically
3. **Rotate**: Click and drag mouse to rotate camera
4. **Zoom**: Scroll mouse wheel to zoom in/out
5. **Switch Trees**: Press number keys 1-9 for instant tree selection

## Mouse Controls

### Camera Manipulation
| Action | Control | Description |
|--------|---------|-------------|
| **Rotate Camera** | Mouse Drag | Click and drag to orbit around the L-system |
| **Zoom In/Out** | Mouse Wheel | Scroll up to zoom in, down to zoom out |
| **Start Rotation** | Left Click + Hold | Begin camera rotation mode |
| **Stop Rotation** | Release Left Click | End camera rotation mode |

### Camera Behavior
- **Orbital Motion**: Camera orbits around the center of the L-system
- **Smooth Rotation**: Continuous mouse movement for smooth camera control
- **Zoom Limits**: Zoom constrained between 1.0 and 100.0 distance units
- **Auto-Center**: Camera always looks at the L-system center

## Keyboard Controls

### Tree Selection (Instant Load)
| Key | Tree Species | Characteristics |
|-----|--------------|-----------------|
| **1** | Sierpinski Triangle | Classic mathematical fractal |
| **2** | 3D Plant | General plant structure |
| **3** | Oak Tree | Wide, spreading branches |
| **4** | Pine Tree | Upward coniferous growth |
| **5** | Cherry Blossom | Pink flowering (default) |
| **6** | Autumn Maple | Fall color transitions |
| **7** | Weeping Willow | Graceful drooping branches |
| **8** | Baobab Tree | Thick trunk, sparse branches |
| **9** | Spiral Eucalyptus | Unique spiraling pattern |

### System Controls
| Key | Action | Description |
|-----|--------|-------------|
| **Tab** | Toggle Menu | Show/hide the L-system selection menu |
| **E** | Edit L-system | Open current L-system file in vim |
| **R** | Reload | Reload current L-system from disk |
| **Escape** | Exit | Close the application |

## Menu System

### Navigation
- **Tab**: Toggle menu visibility
- **Up/Down Arrows**: Navigate menu items
- **Enter**: Select highlighted item
- **Tab** (again): Close menu

### Menu Features
- **Visual Indicators**: Selected item highlighted in green
- **Hotkey Display**: Shows number keys for quick access
- **File Status**: Displays which L-systems are available
- **Instructions**: Built-in help text at bottom

### Menu Layout
```
┌─────────────────────────────────────┐
│ L-System Menu                       │
├─────────────────────────────────────┤
│ > Cherry Blossom (5)                │
│   Autumn Maple (6)                  │
│   Oak Tree (3)                      │
│   Pine Tree (4)                     │
│   ...                               │
├─────────────────────────────────────┤
│ Arrow keys: Navigate | Enter: Select│
│ Tab: Toggle Menu | E: Edit          │
└─────────────────────────────────────┘
```

## Editor Integration

### Vim Editing Workflow
1. **Open Editor**: Press `E` to edit current L-system
2. **Edit Rules**: Modify JSON file in vim
3. **Save & Exit**: `:wq` in vim to save and exit
4. **Auto-Reload**: Application automatically reloads the L-system
5. **View Changes**: See your edits instantly in 3D

### Editor Features
- **Template Creation**: Auto-generates template for new files
- **Syntax**: Standard JSON editing in vim
- **Error Handling**: Displays errors if JSON is invalid
- **Hot Reload**: Instant feedback on changes

### Example Edit Session
```bash
# Press E key in application
# Vim opens with current L-system file:
{
  "name": "My Custom Tree",
  "axiom": "T", 
  "angle": 25.0,
  "rules": {
    "T": "F[+T][-T]"  # ← Edit this line
  }
}
# Save and exit vim (:wq)
# Application instantly shows your changes
```

## File Management

### L-System Files Location
- **Directory**: `rules/` folder in project root
- **Format**: JSON files (`.json` extension)
- **Auto-Discovery**: All JSON files automatically loaded
- **Custom Files**: Add your own files to `rules/` directory

### File Structure
```
rules/
├── cherry_blossom.json    ← Default
├── oak_tree.json
├── pine_tree.json
├── autumn_maple.json
├── willow_tree.json
├── baobab_tree.json
├── spiral_eucalyptus.json
├── plant.json
├── sierpinski.json
└── your_custom_tree.json  ← Your creations
```

## Performance & Optimization

### Performance Characteristics
| Tree Type | Generation Speed | Rendering Load | Memory Usage |
|-----------|------------------|----------------|--------------|
| Baobab | ⚡ Very Fast | 🟢 Light | 💚 Low |
| Oak/Pine/Maple | ⚡ Fast | 🟡 Medium | 💛 Medium |
| Plant/Sierpinski | ⚡ Fast | 🟡 Medium | 💛 Medium |
| Eucalyptus | 🔄 Medium | 🟠 High | 🧡 Medium-High |
| Weeping Willow | 🔄 Slow | 🔴 Heavy | ❤️ High |
| Cherry Blossom | 🐌 Very Slow | 🔴 Very Heavy | ❤️ Very High |

### Optimization Tips
- **Start with Simple Trees**: Begin with Baobab or Oak for fast loading
- **Reduce Iterations**: Lower iteration counts for faster generation
- **Monitor Memory**: Cherry Blossom uses significant memory (~30K characters)
- **Close/Reopen**: Restart application to clear memory after heavy trees

## Troubleshooting

### Common Issues

#### Application Won't Start
- **Check Dependencies**: Ensure Rust is installed and up-to-date
- **Build Issues**: Run `cargo clean` then `cargo build --release`
- **Missing Files**: Verify `rules/` directory exists

#### L-System Won't Load
- **File Format**: Ensure JSON is valid (use online JSON validator)
- **File Location**: Place files in `rules/` directory
- **File Extension**: Use `.json` extension
- **Permissions**: Ensure files are readable

#### Performance Issues
- **Reduce Complexity**: Lower iteration counts in L-system files
- **Close Heavy Trees**: Switch to simpler trees to free memory
- **System Resources**: Close other applications for better performance

#### Editor Issues
- **Vim Not Found**: Set `EDITOR` environment variable to your preferred editor
- **File Permissions**: Ensure L-system files are writable
- **JSON Errors**: Check JSON syntax if reload fails

### Error Messages

| Error | Cause | Solution |
|-------|-------|----------|
| "Failed to load rule file" | Invalid JSON or missing file | Check file exists and JSON is valid |
| "Error launching editor" | Editor not found | Install vim or set EDITOR variable |
| "Failed to create file" | Permission issues | Check directory write permissions |
| Generation timeout | L-system too complex | Reduce iterations or simplify rules |

## Advanced Usage

### Command Line Options
```bash
# Default (Cherry Blossom)
cargo run --release

# Specify different default tree
cargo run --release -- -r rules/oak_tree.json

# Show help
cargo run --release -- --help
```

### Environment Variables
```bash
# Set preferred editor (default: vim)
export EDITOR=nvim
export VISUAL=code

# Then launch application
cargo run --release
```

### Custom Development
- **Add New Trees**: Create JSON files in `rules/` directory
- **Modify Existing**: Edit any existing tree files
- **Color Experiments**: Try different color palettes
- **Rule Variations**: Experiment with different L-system rules

### Keyboard Shortcuts Summary
```
Navigation:
  Mouse Drag     - Rotate camera
  Mouse Wheel    - Zoom in/out
  
Quick Selection:
  1-9           - Instant tree loading
  Tab           - Menu toggle
  
Editing:
  E             - Edit current L-system
  R             - Reload from disk
  
System:
  Escape        - Exit application
```

## Tips & Best Practices

### Viewing Tips
- **Start with Overview**: Zoom out first to see full tree structure
- **Examine Details**: Zoom in to see individual branches and colors
- **Try All Angles**: Rotate camera to see 3D structure from different viewpoints
- **Compare Species**: Switch between trees to see different growth patterns

### Editing Tips
- **Backup First**: Copy existing files before major edits
- **Small Changes**: Make incremental changes and test frequently
- **Study Examples**: Look at existing trees to understand patterns
- **Test Performance**: Monitor generation time with complex rules

### Learning Progression
1. **Explore Existing Trees**: Use keys 1-9 to see all species
2. **Edit Simple Trees**: Start with Baobab or Oak
3. **Understand 3D Commands**: Learn pitch, yaw, roll effects
4. **Create Custom Colors**: Experiment with color palettes
5. **Design New Species**: Create your own tree types
6. **Share Creations**: Add your custom trees to the collection