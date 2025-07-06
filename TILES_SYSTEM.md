# Tiles System Documentation

## Overview

The tiles system provides a comprehensive solution for creating tile-based worlds with parallax scrolling backgrounds. It includes support for:

- **Tile-based world generation** with different tile types
- **Multi-layer parallax backgrounds** with configurable speeds
- **Camera following system** for smooth player tracking
- **Collision detection** for solid tiles
- **Extensible architecture** for adding new tile types and features

## Architecture

### Components

#### `Tile`
Represents individual tiles in the game world.
```rust
pub struct Tile {
    pub tile_type: TileType,  // Type of tile (Ground, Platform, Decoration)
    pub solid: bool,          // Whether the tile has collision
}
```

#### `TileType`
Enum defining different types of tiles:
- `Ground`: Solid ground tiles
- `Platform`: Floating platform tiles
- `Decoration`: Non-solid decorative tiles

#### `ParallaxLayer`
Component for parallax scrolling background layers.
```rust
pub struct ParallaxLayer {
    pub speed_multiplier: f32,  // How fast this layer moves relative to camera
    pub repeat_width: f32,      // Width before the layer repeats (auto-set to screen width)
    pub layer_depth: f32,       // Z-depth of the layer
}
```

#### `MainCamera`
Marker component for the main camera used for parallax calculations.

### Constants

Located in `src/constants.rs`:

```rust
// Tile system
pub const TILE_SIZE: f32 = 32.0;

// Parallax speeds (lower = slower, farther back)
pub const PARALLAX_BACKGROUND_0_SPEED: f32 = 0.1;  // Farthest layer
pub const PARALLAX_BACKGROUND_1_SPEED: f32 = 0.3;  // Middle layer
pub const PARALLAX_BACKGROUND_2_SPEED: f32 = 0.6;  // Closest layer

// Default window dimensions (used as fallback)
pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;

// Camera settings
pub const CAMERA_FOLLOW_SPEED: f32 = 5.0;
pub const CAMERA_OFFSET_Y: f32 = 100.0;
```

## Systems

### Setup Systems

#### `setup_parallax_backgrounds`
- Spawns 3 layers of parallax backgrounds that automatically scale to screen size
- Creates duplicates for seamless scrolling
- Configures each layer with different speeds and depths
- Automatically detects window dimensions for full-screen coverage

#### `setup_tilemap`
- Loads the tileset texture atlas
- Creates ground tiles across the level
- Spawns floating platforms at various heights
- Adds decorative tiles
- Automatically adds colliders to solid tiles

### Runtime Systems

#### `update_parallax`
- Updates parallax background positions based on camera movement
- Handles seamless wrapping for infinite scrolling
- Runs when the camera transform changes

#### `update_background_size_on_resize`
- Automatically resizes background layers when window is resized
- Maintains full-screen coverage at all window sizes
- Updates repeat width for proper parallax wrapping

#### `update_camera_follow`
- Smoothly follows the player with configurable speed
- Adds vertical offset for better gameplay view
- Uses lerp for smooth camera movement

#### `cleanup_distant_tiles` (Optional)
- Removes tiles that are far from the camera for performance
- Configurable cleanup distance

#### `generate_tiles_ahead` (Optional)
- Placeholder for procedural tile generation
- Can be extended for infinite level generation

## Asset Structure

The system expects the following assets in `assets/scene/`:

```
assets/scene/
├── background_0.png    # Farthest parallax layer
├── background_1.png    # Middle parallax layer
├── background_2.png    # Closest parallax layer
└── tileset.png         # Tile atlas (32x32 tiles, 16x16 grid)
```

### Tileset Layout

The tileset is expected to be a 16x16 grid of 32x32 pixel tiles:
- Index 0: Ground tiles
- Index 1: Platform tiles
- Index 2: Decoration tiles
- Additional indices can be added for more tile types

## Usage

### Adding New Tile Types

1. **Add to TileType enum**:
```rust
pub enum TileType {
    Ground,
    Platform,
    Decoration,
    Water,        // New tile type
    Spikes,       // Another new type
}
```

2. **Update spawn_tile function**:
```rust
match tile_type {
    TileType::Water => {
        // Add water-specific logic
    }
    TileType::Spikes => {
        // Add spikes-specific logic
    }
    // ... existing cases
}
```

3. **Add to setup_tilemap**:
```rust
// Spawn water tiles
spawn_tile(
    &mut commands,
    tileset_texture.clone(),
    tileset_layout.clone(),
    Vec3::new(x, y, 0.0),
    TileType::Water,
    3,     // Water tile atlas index
    false, // Not solid
);
```

### Configuring Parallax Layers

To adjust parallax speeds, modify the constants:

```rust
// Slower background (more distant feel)
pub const PARALLAX_BACKGROUND_0_SPEED: f32 = 0.05;

// Faster background (closer feel)
pub const PARALLAX_BACKGROUND_2_SPEED: f32 = 0.8;
```

### Camera Configuration

Adjust camera behavior:

```rust
// Faster camera following
pub const CAMERA_FOLLOW_SPEED: f32 = 10.0;

// Different camera offset
pub const CAMERA_OFFSET_Y: f32 = 150.0;
```

### Window Configuration

Configure default window size and behavior:

```rust
// Default window dimensions
pub const DEFAULT_WINDOW_WIDTH: f32 = 1920.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 1080.0;
```

The window is configured as resizable by default, and backgrounds automatically adapt to any window size.

## Integration

The tiles system integrates with the main game through:

1. **Startup Systems**:
   - `setup_parallax_backgrounds`
   - `setup_tilemap`

2. **Update Systems**:
   - `update_camera_follow`
   - `update_parallax`
   - `update_background_size_on_resize`

3. **Physics Integration**:
   - Automatic collider creation for solid tiles
   - Works with Rapier2D physics system

## Performance Considerations

- **Parallax Update**: Only runs when camera changes position
- **Window Resize**: Only runs when window is actually resized
- **Full-Screen Scaling**: Backgrounds automatically scale without performance penalty
- **Tile Cleanup**: Optional system to remove distant tiles
- **Asset Loading**: Reuses texture handles for efficiency
- **Collision**: Only solid tiles have colliders

## Extension Points

The system is designed to be easily extensible:

1. **Procedural Generation**: Extend `generate_tiles_ahead`
2. **Animated Tiles**: Add animation components to tiles
3. **Interactive Tiles**: Add trigger components for special tiles
4. **Level Loading**: Replace manual tile placement with level files
5. **Tile Variants**: Add random tile variants for visual diversity

## Troubleshooting

### Common Issues

1. **Parallax not working**: Ensure camera has `MainCamera` component
2. **Tiles not visible**: Check tileset atlas configuration
3. **No collision**: Verify tiles are marked as solid
4. **Performance issues**: Enable tile cleanup system

### Debug Commands

- `F3`: Toggle physics debug rendering to see tile colliders
- `F1`: Show player debug info including position
- `F2`: Toggle FPS counter

## Future Enhancements

Potential improvements for the tiles system:

1. **Chunk-based Loading**: Load tiles in chunks for better performance
2. **Tile Animations**: Support for animated tiles
3. **Lighting System**: Add tile-based lighting
4. **Destructible Tiles**: Allow tiles to be destroyed
5. **Tile Entities**: More complex tile behaviors
6. **Level Editor**: Visual editor for creating levels
7. **Multiple Tilesets**: Support for different themed areas