# 3D Tree Species Guide

This document details the botanical characteristics and L-system implementations of each tree species in the collection.

## Tree Species Overview

| Key | Species | Characteristics | Complexity | Best Features |
|-----|---------|-----------------|------------|---------------|
| 3 | Oak Tree | Wide canopy, thick trunk | Medium | Realistic branching |
| 4 | Pine Tree | Coniferous, upward growth | Medium | Radial symmetry |
| 5 | Cherry Blossom | Delicate flowering | High | Pink color palette |
| 6 | Autumn Maple | Fall colors | Medium | Color transitions |
| 7 | Weeping Willow | Drooping branches | High | Graceful form |
| 8 | Baobab Tree | Thick trunk, sparse branches | Low | Dramatic proportions |
| 9 | Spiral Eucalyptus | Spiraling growth | High | Unique geometry |

## Detailed Species Descriptions

### 🌳 Oak Tree (Quercus)
**Key: 3** | **Complexity: Medium** | **Characters: ~2,027**

```json
"rules": { "A": "!F[&+A][\\/A][^-A]", "F": "FF" }
```

**Botanical Features:**
- Wide, spreading canopy typical of mature oaks
- Thick trunk (!) with substantial branching
- Balanced growth in all directions

**L-System Features:**
- `&` pitch-down for horizontal branch spread
- `\\/` roll commands for radial distribution
- `^-` pitch-up with turn for crown formation
- Depth-based coloring: brown trunk to green leaves

**Real-world Inspiration:** English Oak (Quercus robur)

---

### 🌲 Pine Tree (Pinus)
**Key: 4** | **Complexity: Medium** | **Characters: Variable**

```json
"rules": { "T": "F[&+T][&-T][^\\T][^\\/T]T", "F": "F" }
```

**Botanical Features:**
- Coniferous structure with upward-directed growth
- Whorled branching pattern
- Maintains apical dominance (central leader)

**L-System Features:**
- `&` pitch-down for lateral branches
- `^\\` and `^\\/` for spiral arrangement
- Recursive terminal growth with `T`
- Dark green coloring throughout

**Real-world Inspiration:** Scots Pine (Pinus sylvestris)

---

### 🌸 Cherry Blossom (Prunus) - DEFAULT
**Key: 5** | **Complexity: High** | **Characters: ~30,484**

```json
"rules": { "C": "F[\\^+C][/^-C][&+C][&-C]", "F": "F#" }
```

**Botanical Features:**
- Delicate, flowering branches with abundant blooms
- Graceful, somewhat horizontal branching
- Dense flower clusters on branch tips

**L-System Features:**
- Complex 3D branching with all rotation axes
- `#` color increments for flower progression
- Pink palette: brown trunk → pink branches → light pink flowers
- High iteration count for detailed structure

**Real-world Inspiration:** Japanese Cherry (Prunus serrulata)

---

### 🍁 Autumn Maple (Acer)
**Key: 6** | **Complexity: Medium** | **Characters: Variable**

```json
"rules": { "M": "F[\\+M#][/-M#][&+M#][^-M#]", "F": "FF#" }
```

**Botanical Features:**
- Deciduous tree with spectacular fall coloration
- Opposite branching pattern
- Palmate leaf arrangement (simulated through coloring)

**L-System Features:**
- `#` color progression with every branch and segment
- Custom palette: brown → orange → yellow → red
- Balanced branching in all directions
- Color transitions simulate seasonal change

**Real-world Inspiration:** Sugar Maple (Acer saccharum)

---

### 🌿 Weeping Willow (Salix)
**Key: 7** | **Complexity: High** | **Characters: Variable**

```json
"rules": { "W": "F[&+W][&-W][\\&W][\\/&W]", "F": "F[&f]" }
```

**Botanical Features:**
- Characteristic drooping branches (pendulous habit)
- Fine, hanging branchlets
- Graceful, flowing form

**L-System Features:**
- Extensive use of `&` (pitch-down) for drooping effect
- `f` (forward without drawing) for fine branch details
- `\\&` and `/&` for spiral drooping pattern
- Light green coloration throughout

**Real-world Inspiration:** Weeping Willow (Salix babylonica)

---

### 🌴 Baobab Tree (Adansonia)
**Key: 8** | **Complexity: Low** | **Characters: Variable**

```json
"rules": { "B": "!!!F[^+B][^-B][&\\B][&\\/B][+B][-B]", "F": "FF!" }
```

**Botanical Features:**
- Massive, bottle-shaped trunk
- Sparse, upward-reaching branches
- Distinctive African savanna profile

**L-System Features:**
- `!!!` triple line-width increase for massive trunk
- `^` pitch-up dominant for upward branching
- Minimal branching iterations for sparse crown
- Brown/tan coloring throughout

**Real-world Inspiration:** African Baobab (Adansonia digitata)

---

### 🌿 Spiral Eucalyptus (Eucalyptus)
**Key: 9** | **Complexity: High** | **Characters: Variable**

```json
"rules": { "E": "F[\\E][/E]E", "F": "F\\#" }
```

**Botanical Features:**
- Unique spiraling growth pattern
- Continuous helical branching
- Australian native characteristics

**L-System Features:**
- `\\` continuous roll for spiral effect
- `#` color progression along spiral
- Simplified branching focused on spiral geometry
- Green gradient coloring

**Real-world Inspiration:** Spiral-leaved Eucalyptus varieties

## L-System Turtle Commands Reference

### Movement Commands
- `F`, `G` - Move forward and draw line
- `f`, `g` - Move forward without drawing

### 3D Rotation Commands
- `+` / `-` - Yaw (turn left/right around up-axis)
- `^` / `&` - Pitch (rotate up/down around right-axis)  
- `\\` / `/` - Roll (rotate left/right around forward-axis)
- `|` - Turn around 180° (reverse direction)

### State Management
- `[` - Push current state (position, orientation, color)
- `]` - Pop and restore saved state

### Rendering Controls
- `#` - Advance to next color in palette
- `!` - Increase line width (for thick trunks)
- `'` - Decrease line width

## Color System Types

### Depth-based Coloring
```json
"colors": { "depth_based": true }
```
- Automatically colors based on Y-position (height)
- Brown at bottom (trunk) → Green at top (leaves)
- Realistic for most tree species

### Palette-based Coloring
```json
"colors": {
  "depth_based": false,
  "palette": [[r1,g1,b1], [r2,g2,b2], ...]
}
```
- Uses `#` command to advance through colors
- Perfect for seasonal effects and flowers
- Allows precise color control

## Performance Notes

| Species | Generation Time | Rendering Complexity | Memory Usage |
|---------|----------------|---------------------|--------------|
| Baobab | Fast | Low | Light |
| Oak | Medium | Medium | Medium |
| Pine | Medium | Medium | Medium |
| Maple | Medium | Medium | Medium |
| Cherry Blossom | Slow | High | Heavy |
| Weeping Willow | Slow | High | Heavy |
| Spiral Eucalyptus | Medium | High | Medium |

## Customization Tips

### Creating New Species
1. **Study Real Trees**: Observe branching patterns in nature
2. **Start Simple**: Begin with basic rules, add complexity gradually
3. **Use 3D Wisely**: Don't use all rotation commands unless needed
4. **Color Thoughtfully**: Choose botanically appropriate palettes
5. **Test Iterations**: Start with low iterations, increase gradually
6. **Performance Balance**: Complex rules grow exponentially

### Tuning Parameters
- **Angle**: Smaller angles (15-20°) for delicate trees, larger (25-35°) for robust trees
- **Step Length**: Shorter (0.5-0.8) for fine detail, longer (1.0-2.0) for bold structure  
- **Iterations**: 4-5 for simple trees, 6-8 for complex flowering trees
- **Start Position**: Adjust Y-coordinate for proper ground placement

### Advanced Techniques
- **Conditional Growth**: Use different rules for different parts
- **Stochastic Elements**: Consider probability-based rule selection
- **Seasonal Variants**: Create multiple versions with different coloring
- **Environmental Response**: Simulate wind, light direction, or terrain effects