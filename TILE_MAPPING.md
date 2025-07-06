# Tile Mapping System Documentation

## Overview

The tile mapping system provides a comprehensive solution for creating tile-based levels using a tileset with 16x16 pixel tiles. The system supports:

- **16x16 tile-based levels** loaded from simple text files
- **Automatic collision detection** based on tile indices
- **Visual debugging tools** for level editing and testing
- **Flexible tile properties** system for different tile behaviors
- **Integration with physics** for solid tiles and platforms

## Tileset Structure

### Expected Tileset Format
- **File**: `assets/scene/tileset.png`
- **Tile Size**: 16x16 pixels
- **Grid Layout**: 16 tiles per row, 16 tiles per column (256x256 total image)
- **Total Tiles**: 256 possible tile indices (0-255)

### Tile Index Calculation
```
tile_index = (row * 16) + column
```

Example tile positions:
- Index 0: Top-left corner (row 0, col 0)
- Index 15: Top-right corner (row 0, col 15)
- Index 16: Second row, first column (row 1, col 0)
- Index 255: Bottom-right corner (row 15, col 15)

## Level File Format

### File Structure
Level files use a simple comma-separated format:

```
width,height
tile_row_0
tile_row_1
...
tile_row_height-1
```

### Example Level File (`assets/levels/level1.txt`)
```
30,20
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
...
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1
```

### Special Values
- **255**: Empty/Air tile (invisible, no collision)
- **0-254**: Valid tile indices from tileset

## Tile Properties System

### Collision Mapping
Tiles are categorized into different collision types:

#### Solid Tiles (Full Collision)
```rust
// Default solid tiles (customizable in level_loader.rs)
solid_tiles.insert(0);  // Ground tile
solid_tiles.insert(1);  // Stone tile
solid_tiles.insert(2);  // Brick tile
solid_tiles.insert(16); // Second row first tile
solid_tiles.insert(17); // Second row second tile
```

#### Platform Tiles (Jump-through)
```rust
// Default platform tiles
platform_tiles.insert(32); // Third row first tile
platform_tiles.insert(33); // Third row second tile
```

#### Empty/Decorative Tiles
- All other indices (except 255) are rendered but have no collision
- Index 255 is not rendered at all

## Debug Tools

### Available Debug Commands

| Key | Function | Description |
|-----|----------|-------------|
| `F3` | Physics Debug | Toggle collision box visualization |
| `F4` | Tile Info | Show tile information under cursor |
| `F5` | Tile Grid | Toggle tile grid overlay |
| `F6` | Tile Collisions | Highlight tiles with collision |
| `F7` | Tileset Info | Show tileset information in console |

### Debug Features

#### Tile Grid (F5)
- Shows 16x16 grid overlay around camera
- Green semi-transparent rectangles
- Helps with precise tile placement

#### Collision Highlighting (F6)
- Red overlay on tiles with collision
- Useful for verifying collision setup

#### Tile Information (F4)
- Hover over tiles to see their properties
- Shows tile index and tileset coordinates
- Console output format:
  ```
  Tile at (X, Y): Index N, Tileset pos (col, row)
  ```

## Creating and Editing Levels

### Manual Level Creation

1. **Plan Your Level**
   - Decide on dimensions (width x height)
   - Map out where different tile types should go

2. **Create Level File**
   ```
   # Example: 10x5 level
   10,5
   255,255,255,255,255,255,255,255,255,255
   255,255,255,32,32,32,255,255,255,255
   255,255,255,255,255,255,255,255,255,255
   255,2,255,255,255,255,255,255,2,255
   0,0,0,0,0,0,0,0,0,0
   ```

3. **Add to Assets**
   - Save as `.txt` file in `assets/levels/`
   - Load using the level loading system

### Tile Index Reference

#### Common Tile Assignments
```
Row 0 (0-15):   Ground and basic terrain
Row 1 (16-31):  Stone and solid blocks
Row 2 (32-47):  Platforms and semi-solid
Row 3 (48-63):  Decorative elements
...
```

### Level Dimensions
- **Coordinate System**: Top-left origin (0,0)
- **World Position**: X increases right, Y increases down
- **Tile Size**: 16x16 pixels in world space
- **Recommended Size**: 30-100 width, 20-50 height

## Programming Interface

### Loading Custom Levels

```rust
// Load level from file (future feature)
use crate::systems::level_loader::load_level_from_file;

let level_data = load_level_from_file("assets/levels/my_level.txt")?;
```

### Modifying Tile Properties

```rust
// In level_loader.rs, modify these sets:
let mut solid_tiles = std::collections::HashSet::new();
solid_tiles.insert(0);   // Add tile index 0 as solid
solid_tiles.insert(16);  // Add tile index 16 as solid

let mut platform_tiles = std::collections::HashSet::new();
platform_tiles.insert(32); // Add tile index 32 as platform
```

### Runtime Tile Access

```rust
// Get tile at world position
let tile_index = get_tile_at_position(&level_data, world_pos);

// Set tile at world position
set_tile_at_position(&mut level_data, world_pos, new_tile_index);
```

## Integration with Game Systems

### Physics Integration
- Solid tiles automatically get `Collider::cuboid(8.0, 8.0)` components
- Platform tiles get `Collider::cuboid(8.0, 4.0)` for partial collision
- Colliders are added dynamically by `update_tile_collisions` system

### Rendering Integration
- Tiles use the main tileset texture atlas
- Each tile gets appropriate `TextureAtlas` index
- Sprites are positioned at world coordinates based on tile grid

### Camera Integration
- Level coordinates work with the camera follow system
- Parallax backgrounds layer behind tile level
- Debug overlays respect camera position

## Performance Considerations

### Optimization Features
- **Selective Collision**: Only tiles marked as solid get colliders
- **Dynamic Loading**: Collision components added as needed
- **Efficient Rendering**: Uses sprite batching through texture atlas
- **Memory Efficient**: Level data stored as simple 2D array

### Best Practices
- Keep level dimensions reasonable (< 100x100 for performance)
- Use index 255 for empty space to avoid unnecessary sprites
- Group similar tiles together in tileset for better batching
- Test with debug tools to verify collision setup

## Extending the System

### Adding New Tile Types

1. **Define in Collision Map**:
   ```rust
   // Add special tile behavior
   special_tiles.insert(64); // New special tile
   ```

2. **Handle in Collision System**:
   ```rust
   if special_tiles.contains(&tile_index) {
       // Custom collision logic
   }
   ```

3. **Update Documentation**: Add to tile index reference

### Custom Level Formats
The system can be extended to support:
- JSON level files
- TMX (Tiled Map Editor) format
- Binary level formats
- Compressed level data

### Advanced Features
Potential enhancements:
- **Animated Tiles**: Support for tile animations
- **Tile Variants**: Random tile variations
- **Layered Levels**: Multiple tile layers
- **Tile Entities**: Tiles with special behaviors
- **Level Editor**: In-game level editing tools

## Troubleshooting

### Common Issues

1. **Tiles Not Visible**
   - Check tileset.png is in `assets/scene/`
   - Verify tile indices are not 255
   - Use F5 to see tile grid

2. **No Collision**
   - Check if tile index is in solid_tiles set
   - Use F6 to highlight collision tiles
   - Verify collision map is loaded as resource

3. **Wrong Tile Displayed**
   - Check tile index calculation
   - Verify tileset dimensions match expectations
   - Use F4 to inspect tile properties

4. **Level Not Loading**
   - Check file format (comma-separated)
   - Verify file path in assets/levels/
   - Check console for error messages

### Debug Workflow

1. Enable tile grid (F5) for precise positioning
2. Use tile collision debug (F6) to verify physics
3. Hover with F4 to inspect individual tiles
4. Check F7 for tileset configuration
5. Use F3 for physics debug if needed

## Example Workflows

### Creating a Simple Platform Level

1. **Design Layout**:
   ```
   Air, Air, Air, Air, Air
   Air, Platform, Platform, Air, Air
   Air, Air, Air, Air, Air
   Ground, Ground, Ground, Ground, Ground
   ```

2. **Convert to Indices**:
   ```
   5,4
   255,255,255,255,255
   255,32,32,255,255
   255,255,255,255,255
   0,0,0,0,0
   ```

3. **Test and Iterate**:
   - Load level in game
   - Use debug tools to verify
   - Adjust tile indices as needed
   - Test player movement and collision

This tile mapping system provides a solid foundation for creating complex 2D levels with minimal setup while maintaining flexibility for future enhancements.