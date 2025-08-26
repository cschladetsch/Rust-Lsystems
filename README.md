# 3D L-Systems Tree Generator

A beautiful, interactive 3D L-Systems visualization tool featuring realistic tree generation with full camera controls, live editing, and botanical species simulation.

## Features

### 🌳 **Realistic 3D Trees**
- **7 Tree Species** with botanically-inspired characteristics:
  - **Oak Tree** - Thick trunk with wide, spreading branches
  - **Pine Tree** - Coniferous upward growth with radial symmetry
  - **Cherry Blossom** - Delicate pink flowering branches (default)
  - **Autumn Maple** - Vibrant fall colors (brown→orange→red→yellow)
  - **Weeping Willow** - Graceful drooping branches
  - **Baobab Tree** - Thick African trunk with dramatic branching
  - **Spiral Eucalyptus** - Unique spiraling growth pattern

### 🎮 **Interactive 3D Graphics**
- **Full 3D Rendering** with perspective projection and depth buffering
- **Camera Controls**: Mouse drag to rotate, mouse wheel to zoom
- **Real-time Visualization** of complex L-system structures
- **Smooth Performance** with optimized software rasterization

### 🎨 **Advanced Coloring Systems**
- **Depth-based Coloring**: Automatic gradients from brown trunks to green leaves
- **Species-specific Palettes**: Custom color schemes for each tree type
- **Seasonal Themes**: Autumn colors, cherry blossoms, etc.
- **Dynamic Color Changes**: Using L-system color increment commands

### ⚙️ **Live Editing & Development**
- **Vim Integration**: Edit L-system rules in real-time
- **Hot Reload**: Press 'R' to instantly see your changes
- **Template System**: Auto-creates template files for new L-systems
- **Interactive Menu**: Browse and switch between L-systems

### 📐 **Advanced L-System Features**
- **Full 6DOF Turtle Graphics**: 
  - `F/G` - Move forward (with/without drawing)
  - `+/-` - Yaw (turn left/right)
  - `^/&` - Pitch (up/down)
  - `\\/` - Roll (rotate around forward axis)
  - `|` - Turn around 180°
- **3D Branching**: `[` push state, `]` pop state in full 3D space
- **Line Width Control**: `!` for thick trunks and branches
- **Color Control**: `#` for color palette progression

## Installation

### Prerequisites
- **Rust** (latest stable)
- **Git**
- **Vim/Neovim** (for editing L-systems)

### Build from Source
```bash
git clone https://github.com/cschladetsch/Rust-Lsystems.git
cd Rust-Lsystems
cargo build --release
cargo run --release
```

## Usage

### Basic Controls
- **Mouse + Drag**: Rotate camera around the L-system
- **Mouse Wheel**: Zoom in/out
- **M**: Toggle main menu (central navigation hub)
- **Tab**: Toggle tree selection menu
- **G**: Toggle GUI parameter controls for real-time editing
- **Escape**: Exit application

### Quick Tree Selection
- **1**: Sierpinski Triangle (classic fractal)
- **2**: 3D Plant (branching plant structure)
- **3**: Oak Tree (wide, spreading branches)
- **4**: Pine Tree (coniferous, upward growth)
- **5**: Cherry Blossom (pink flowering - default)
- **6**: Autumn Maple (fall colors)
- **7**: Weeping Willow (drooping branches)
- **8**: Baobab Tree (thick African trunk)
- **9**: Spiral Eucalyptus (spiraling growth)

### Menu Navigation
- **Main Menu (M)**: Access all features from one central hub
  - Tree Species → Tab menu or number keys 1-9
  - Parameters → G key for real-time sliders
  - Edit L-system → E key to open in vim
  - Reload → R key to refresh from disk
  - Help → H key for controls reference
  - Exit → Escape to close

### Live Editing
- **E**: Edit current L-system file in vim
- **R**: Reload L-system after editing
- **G**: Real-time parameter adjustment with sliders
- **Tab → Arrow Keys → Enter**: Navigate and select L-systems

## L-System File Format

L-systems are defined in JSON format:

```json
{
  "name": "Cherry Blossom 3D",
  "axiom": "C",
  "angle": 20.0,
  "iterations": 6,
  "rules": {
    "C": "F[\\\\^+C][/^-C][&+C][&-C]",
    "F": "F#"
  },
  "step_length": 0.7,
  "start_position": [0.0, -7.0, 0.0],
  "start_direction": [0.0, 1.0, 0.0],
  "colors": {
    "depth_based": false,
    "palette": [
      [0.4, 0.2, 0.0],    // Brown trunk
      [0.8, 0.4, 0.5],    // Pink branches
      [1.0, 0.7, 0.8],    // Light pink
      [1.0, 0.9, 0.9],    // Pale pink
      [0.9, 0.2, 0.4]     // Deep pink flowers
    ]
  },
  "description": "Beautiful cherry blossom tree with pink flowering branches"
}
```

### 3D Turtle Commands
| Command | Action | Description |
|---------|--------|-------------|
| `F`, `G` | Forward + Draw | Move forward and draw a line |
| `f`, `g` | Forward | Move forward without drawing |
| `+` | Yaw Left | Turn left around up axis |
| `-` | Yaw Right | Turn right around up axis |
| `^` | Pitch Up | Rotate up around right axis |
| `&` | Pitch Down | Rotate down around right axis |
| `\\` | Roll Left | Roll left around forward axis |
| `/` | Roll Right | Roll right around forward axis |
| `|` | Turn Around | Rotate 180° around up axis |
| `[` | Push State | Save current position and orientation |
| `]` | Pop State | Restore saved position and orientation |
| `#` | Next Color | Advance to next color in palette |
| `!` | Thicker Line | Increase line width |
| `'` | Thinner Line | Decrease line width |

## Creating Custom Trees

1. **Start with Template**: The editor creates template files automatically
2. **Define Growth Pattern**: Use rules to describe how branches grow
3. **Add 3D Structure**: Use pitch (`^&`) and roll (`\\/`) for realistic branching
4. **Set Colors**: Choose depth-based or custom palette coloring
5. **Tune Parameters**: Adjust angles, step length, and iterations
6. **Test & Iterate**: Use hot reload (R key) to see changes instantly

### Example: Simple 3D Tree
```json
{
  "name": "My Tree",
  "axiom": "T",
  "angle": 25.0,
  "iterations": 4,
  "rules": {
    "T": "F[&+T][&-T][^\\\\T][^\\/T]"
  },
  "step_length": 1.0,
  "start_position": [0.0, -5.0, 0.0],
  "start_direction": [0.0, 1.0, 0.0],
  "colors": { "depth_based": true }
}
```

## Technical Details

### Architecture
- **Modular Design**: Separate modules for camera, rendering, turtle graphics, menu, and editor
- **Software Rasterization**: Custom 3D renderer with depth buffering
- **Real-time Performance**: Optimized for interactive frame rates

### 3D Mathematics
- **Perspective Projection**: Proper 3D-to-2D transformation
- **Depth Testing**: Z-buffer for correct occlusion
- **3D Rotations**: Matrix-based transformations for turtle orientation
- **Camera System**: Orbital camera with mouse control

### File Management
- **Auto-discovery**: Automatically loads L-systems from `rules/` directory
- **Hot Reload**: File watching and instant regeneration
- **Template Generation**: Creates starter files for new L-systems

## Contributing

Contributions welcome! Areas for enhancement:
- New tree species and L-system patterns
- Performance optimizations
- Additional rendering features
- UI/UX improvements
- Documentation and examples

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- L-Systems theory by Aristid Lindenmayer
- Turtle graphics concepts by Seymour Papert
- Botanical inspiration from nature's fractal patterns