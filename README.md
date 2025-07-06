# Bevy Sidescroller

A 2D sidescroller game built with Bevy Engine and Rapier2D physics, featuring character movement, sprite animations, and physics-based gameplay.

## Features

- **Character Movement**: WASD/Arrow key controls with jumping
- **Sprite Animations**: Idle and running animations with smooth transitions
- **Sprite Flipping**: Character faces the direction of movement
- **Physics Integration**: Rapier2D physics with collision detection
- **Tile-Based World**: Procedural tile system with multiple tile types
- **Full-Screen Parallax**: Multi-layer backgrounds that automatically scale to any window size
- **Camera Following**: Smooth camera that follows the player
- **Modular Architecture**: Clean, organized code structure
- **Debug Tools**: Physics debug rendering and FPS counter

## Controls

- **Movement**: `A`/`←` (Left), `D`/`→` (Right)
- **Jump**: `W`/`Space`/`↑`
- **Debug Physics**: `F3` (Toggle collision boxes)
- **FPS Debug**: `F2` (Toggle FPS display)
- **Player Info**: `F1` (Show player debug info)

## Project Structure

The project is organized into a modular architecture for better maintainability:

```
src/
├── main.rs              # Entry point and app setup
├── components.rs        # Game components and data structures
├── constants.rs         # Game constants and configuration
└── systems/
    ├── mod.rs          # Systems module exports
    ├── setup.rs        # Initialization systems
    ├── movement.rs     # Player movement systems
    ├── animation.rs    # Sprite animation systems
    ├── tiles.rs        # Tile system and parallax backgrounds
    └── debug.rs        # Debug and development tools
```

### Module Overview

#### `components.rs`
Contains all game components:
- `PlayerVelocity`: Player movement velocity
- `AnimationState`: Current animation state (Idle/Run)
- `FacingDirection`: Character facing direction for sprite flipping
- `AnimationConfig`: Individual animation configuration
- `AnimationCollection`: Collection of all character animations
- `AnimationHandles`: Asset handles for textures and layouts

#### `constants.rs`
Game configuration and constants:
- Movement parameters (speed, gravity, jump force)
- Physics settings (pixels per meter, ground dimensions)
- Animation settings (FPS, frame counts)
- Sprite dimensions and spawn positions

#### `systems/`
All game systems organized by functionality:

- **setup.rs**: World initialization
  - `setup_graphics()`: Camera setup
  - `setup_physics()`: Physics world, ground, and player creation

- **movement.rs**: Character movement
  - `move_player()`: Input handling and physics movement
  - `update_facing_direction()`: Direction tracking for sprite flipping

- **animation.rs**: Visual animations
  - `update_animation_state()`: Animation state transitions
  - `execute_animations()`: Sprite animation execution and flipping

- **tiles.rs**: World generation and parallax
  - `setup_parallax_backgrounds()`: Multi-layer background setup
  - `setup_tilemap()`: Tile-based world generation
  - `update_parallax()`: Parallax scrolling updates
  - `update_camera_follow()`: Smooth camera following

- **debug.rs**: Development tools
  - `toggle_debug_render()`: Physics debug visualization
  - `debug_fps()`: FPS monitoring
  - `debug_player_info()`: Player state debugging

## Assets

The game expects the following assets in the `assets/` directory:

```
assets/
├── character/
│   ├── IDLE.png     # Idle animation spritesheet (10 frames, 96x96 each)
│   └── RUN.png      # Running animation spritesheet (6 frames, 96x96 each)
└── scene/
    ├── background_0.png  # Farthest parallax layer
    ├── background_1.png  # Middle parallax layer
    ├── background_2.png  # Closest parallax layer
    └── tileset.png       # Tile atlas (32x32 tiles, 16x16 grid)
```

## Running the Game

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Build and Run

```bash
# Clone the repository
git clone <repository-url>
cd bevy_sidescroller

# Run the game
cargo run

# For release build
cargo run --release
```

### Development

```bash
# Check code without running
cargo check

# Run with debug information
RUST_LOG=debug cargo run

# Build only
cargo build
```

## Technical Details

### Dependencies

- **Bevy**: Game engine and ECS framework
- **Rapier2D**: Physics engine for 2D collision and movement
- **Standard Library**: Timer and duration utilities

### Architecture Highlights

1. **ECS (Entity Component System)**: Leverages Bevy's ECS for game logic
2. **Modular Design**: Clean separation of concerns across modules
3. **Physics Integration**: Rapier2D for realistic movement and collision
4. **Tile-Based World**: Efficient tile system with collision detection
5. **Full-Screen Parallax**: Responsive multi-layer backgrounds that adapt to any screen size
6. **Camera System**: Smooth following camera with configurable behavior
7. **Asset Management**: Efficient texture and animation handling
8. **Debug Tools**: Built-in debugging capabilities for development

### Performance Considerations

- Sprite flipping is applied once per frame outside animation loops
- Animation state changes only trigger when velocity changes
- Parallax updates only when camera position changes
- Automatic background resizing on window resize events
- Tile collision detection integrated with physics system
- Efficient asset loading with handle reuse for tiles and backgrounds
- Physics simulation optimized for 2D platformer gameplay

## Contributing

When adding new features:

1. Add new components to `components.rs`
2. Add configuration constants to `constants.rs`
3. Create systems in appropriate `systems/` modules
4. Export new systems in `systems/mod.rs`
5. Register systems in `main.rs`

## Advanced Features

### Tiles System
The game includes a comprehensive tile-based world system:
- **Multiple tile types**: Ground, platforms, and decorations
- **Automatic collision**: Solid tiles generate physics colliders
- **Extensible design**: Easy to add new tile types and behaviors
- **Efficient rendering**: Texture atlas-based tile rendering

### Parallax Backgrounds
Multi-layer parallax scrolling creates depth:
- **3 background layers** with different scroll speeds
- **Full-screen coverage** that automatically scales to any window size
- **Seamless wrapping** for infinite scrolling effect
- **Window resize support** for responsive design
- **Configurable speeds** for each layer
- **Performance optimized** updates

For detailed information about the tiles system, see [TILES_SYSTEM.md](TILES_SYSTEM.md).

## License

This project is open source and available under the MIT License.