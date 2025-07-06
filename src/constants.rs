//! Game constants and configuration values

/// Player movement constants
pub const PLAYER_SPEED: f32 = 300.0;
pub const GRAVITY: f32 = -981.0;
pub const JUMP_FORCE: f32 = 300.0;

/// Physics constants
pub const PIXELS_PER_METER: f32 = 100.0;
pub const GROUND_HEIGHT: f32 = -100.0;
pub const GROUND_WIDTH: f32 = 500.0;
pub const GROUND_THICKNESS: f32 = 50.0;

/// Animation constants
pub const IDLE_ANIMATION_FPS: u8 = 5;
pub const RUN_ANIMATION_FPS: u8 = 10;

/// Sprite constants
pub const SPRITE_SIZE: u32 = 96;
pub const IDLE_FRAMES: u32 = 10;
pub const RUN_FRAMES: u32 = 6;

/// Character spawn position
pub const PLAYER_SPAWN_X: f32 = 0.0;
pub const PLAYER_SPAWN_Y: f32 = 100.0;

/// Tile system constants
pub const TILE_SIZE: f32 = 32.0;
pub const TILEMAP_WIDTH: u32 = 50;
pub const TILEMAP_HEIGHT: u32 = 20;

/// New 16x16 tile system constants
pub const TILE_SIZE_16: f32 = 16.0;
pub const TILESET_TILE_SIZE: u32 = 16;
pub const TILESET_WIDTH: u32 = 256; // Assuming 256px wide tileset
pub const TILESET_HEIGHT: u32 = 256; // Assuming 256px tall tileset
pub const TILES_PER_ROW: u32 = TILESET_WIDTH / TILESET_TILE_SIZE; // 16 tiles per row
pub const TILES_PER_COLUMN: u32 = TILESET_HEIGHT / TILESET_TILE_SIZE; // 16 tiles per column

/// Level system constants
pub const MAX_LEVEL_WIDTH: u32 = 200;
pub const MAX_LEVEL_HEIGHT: u32 = 50;
pub const DEFAULT_LEVEL_WIDTH: u32 = 100;
pub const DEFAULT_LEVEL_HEIGHT: u32 = 30;

/// Parallax constants
pub const PARALLAX_BACKGROUND_0_SPEED: f32 = 0.03;
pub const PARALLAX_BACKGROUND_1_SPEED: f32 = 0.1;
pub const PARALLAX_BACKGROUND_2_SPEED: f32 = 0.2;

/// Default window dimensions (fallback values)
pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;

/// Background layer scaling
pub const BACKGROUND_SCALE_FACTOR: f32 = 1.0;

/// Camera settings
pub const CAMERA_FOLLOW_SPEED: f32 = 5.0;
pub const CAMERA_OFFSET_Y: f32 = 100.0;
