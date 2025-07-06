# Tile Constants System Documentation

## Overview

The tile constants system provides an organized way to manage different types of tiles in your game. Instead of remembering random numbers, you can use meaningful constant arrays that group related tiles together.

## System Architecture

### File Location
**Primary Configuration:** `src/systems/level_loader.rs` (lines 14-38)

### How It Works
- Tiles are organized into logical groups using constant arrays
- Each group represents a specific tile type or material
- The collision system automatically applies the correct physics based on these groups
- Easy to modify and extend without hunting through code

## Tile Categories

### 🌱 Terrain Tiles (Solid)
```rust
const GRASS_TILES: [u32; 4] = [0, 1, 2, 3];           // Row 0: Grass variants
const STONE_TILES: [u32; 4] = [16, 17, 18, 19];       // Row 1: Stone variants  
const BRICK_TILES: [u32; 4] = [20, 21, 22, 23];       // Row 1: Brick variants
const ROCK_TILES: [u32; 4] = [4, 5, 6, 7];            // Row 0: Rock variants
```

### 🪵 Platform Tiles (Jump-through)
```rust
const WOOD_PLATFORMS: [u32; 4] = [32, 33, 34, 35];    // Row 2: Wood platforms
const STONE_PLATFORMS: [u32; 4] = [48, 49, 50, 51];   // Row 3: Stone platforms
const METAL_PLATFORMS: [u32; 4] = [64, 65, 66, 67];   // Row 4: Metal platforms
```

### 🌸 Decorative Tiles (No collision)
```rust
const FLOWERS: [u32; 4] = [8, 9, 10, 11];             // Row 0: Flower decorations
const TREES: [u32; 4] = [24, 25, 26, 27];             // Row 1: Tree decorations
const CRYSTALS: [u32; 4] = [40, 41, 42, 43];          // Row 2: Crystal decorations
```

### ⚠️ Special Tiles
```rust
const SPIKES: [u32; 2] = [80, 81];                     // Row 5: Damage tiles
const WATER: [u32; 4] = [96, 97, 98, 99];             // Row 6: Water tiles
const LAVA: [u32; 4] = [112, 113, 114, 115];          // Row 7: Lava tiles
```

## How to Customize

### 1. Adding New Tile Types

**Step 1: Define the constant array**
```rust
// Add this near the top of level_loader.rs (around line 30)
const ICE_TILES: [u32; 4] = [128, 129, 130, 131];     // Row 8: Ice tiles
```

**Step 2: Add to collision system**
```rust
// In the create_collision_map() function, add:
for &tile in &ICE_TILES {
    solid_tiles.insert(tile);  // or platform_tiles for platforms
}
```

### 2. Modifying Existing Groups

**Example: Changing grass tile indices**
```rust
// Change from [0, 1, 2, 3] to [0, 1, 15, 31]
const GRASS_TILES: [u32; 4] = [0, 1, 15, 31];
```

**Example: Adding more platform variants**
```rust
// Expand from 4 to 6 tiles
const WOOD_PLATFORMS: [u32; 6] = [32, 33, 34, 35, 36, 37];
```

### 3. Creating Tile Subcategories

```rust
// Organize by function instead of material
const GROUND_TILES: [u32; 8] = [0, 1, 2, 3, 16, 17, 18, 19];
const WALL_TILES: [u32; 4] = [20, 21, 22, 23];
const CEILING_TILES: [u32; 4] = [24, 25, 26, 27];
```

## Index Calculation Guide

### Tileset Grid Reference
```
Tileset Layout (16x16 grid):
Row 0:  [  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15]
Row 1:  [ 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
Row 2:  [ 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47]
Row 3:  [ 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63]
...
Row 15: [240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255]
```

### Quick Reference Formula
```
tile_index = (row * 16) + column
```

**Examples:**
- Position (row 0, col 5) = **5**
- Position (row 2, col 3) = **35** 
- Position (row 5, col 10) = **90**

## Usage in Level Files

### Using Constants in Your Levels
When creating level files, you can now reference tiles by their logical meaning:

```
# Example level using organized tiles
30,20
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
255,255,255,255,255,255,255,255,255,32,33,34,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255
...
0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3,0,1
16,17,18,19,16,17,18,19,16,17,18,19,16,17,18,19,16,17,18,19,16,17,18,19,16,17,18,19,16,17
```

Where:
- `0,1,2,3` = Grass tile variants
- `32,33,34` = Wood platform sequence  
- `16,17,18,19` = Stone tile variants

## Utility Functions

The system provides helpful utility functions:

### Type Checking Functions
```rust
is_solid_tile(tile_index)       // Returns true if tile is solid
is_platform_tile(tile_index)    // Returns true if tile is platform
is_decorative_tile(tile_index)  // Returns true if tile is decorative
```

### Debug Functions
```rust
get_tile_type_name(tile_index)  // Returns human-readable tile type name
```

### Usage Example
```rust
let tile = 32; // Wood platform
if is_platform_tile(tile) {
    println!("This is a {} tile", get_tile_type_name(tile)); 
    // Output: "This is a Wood Platform tile"
}
```

## Best Practices

### 1. Logical Organization
```rust
// ✅ Good: Organized by material and function
const WOOD_SOLID: [u32; 4] = [16, 17, 18, 19];
const WOOD_PLATFORMS: [u32; 4] = [32, 33, 34, 35];

// ❌ Avoid: Random groupings
const RANDOM_TILES: [u32; 4] = [5, 23, 67, 89];
```

### 2. Consistent Naming
```rust
// ✅ Good: Clear, descriptive names
const METAL_SPIKES: [u32; 2] = [80, 81];
const LAVA_BUBBLES: [u32; 3] = [96, 97, 98];

// ❌ Avoid: Unclear abbreviations
const MTL_SP: [u32; 2] = [80, 81];
const LV_BUB: [u32; 3] = [96, 97, 98];
```

### 3. Size Documentation
```rust
// ✅ Good: Document array sizes and purpose
const GRASS_TILES: [u32; 4] = [0, 1, 2, 3];    // 4 variants of grass
const SMALL_ROCKS: [u32; 2] = [8, 9];          // 2 small decorative rocks
```

### 4. Related Groupings
```rust
// ✅ Good: Group related materials together
const STONE_SOLID: [u32; 4] = [16, 17, 18, 19];
const STONE_PLATFORMS: [u32; 4] = [48, 49, 50, 51];
const STONE_DECORATIONS: [u32; 4] = [64, 65, 66, 67];
```

## Testing Your Changes

### 1. Compile and Run
```bash
cargo run
```

### 2. Use Debug Tools
- **F4**: See tile type names when hovering
- **F6**: Highlight collision tiles (should show your solid tiles in red)
- **F7**: View tileset information

### 3. Verify Collision
- Walk into tiles that should be solid
- Try jumping through platform tiles
- Check that decorative tiles have no collision

## Example: Complete Custom Setup

Here's an example of setting up tiles for a forest-themed level:

```rust
// Forest Theme Tiles
const TREE_TRUNKS: [u32; 4] = [0, 1, 2, 3];           // Solid tree bases
const LEAVES: [u32; 6] = [16, 17, 18, 19, 20, 21];    // Decorative foliage
const BRANCH_PLATFORMS: [u32; 3] = [32, 33, 34];      // Jump-through branches
const MUSHROOMS: [u32; 4] = [48, 49, 50, 51];         // Decorative fungi
const FOREST_FLOOR: [u32; 4] = [64, 65, 66, 67];      // Solid ground with grass
const WATER_TILES: [u32; 4] = [80, 81, 82, 83];       // Decorative water

// In create_collision_map():
for &tile in &TREE_TRUNKS { solid_tiles.insert(tile); }
for &tile in &FOREST_FLOOR { solid_tiles.insert(tile); }
for &tile in &BRANCH_PLATFORMS { platform_tiles.insert(tile); }
// Note: LEAVES, MUSHROOMS, and WATER_TILES get no collision (decorative only)
```

This system makes your tile management much more organized and easier to maintain!