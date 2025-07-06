//! Game components for the sidescroller game

use bevy::prelude::*;
use std::time::Duration;

/// Player velocity component wrapping a Vec2
#[derive(Component, Default)]
pub struct PlayerVelocity(pub Vec2);

/// Animation states for the player character
#[derive(Component, PartialEq, Eq, Clone, Copy, Default, Debug)]
pub enum AnimationState {
    #[default]
    Idle,
    Run,
}

/// Tracks which direction the character is facing for sprite flipping
#[derive(Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum FacingDirection {
    #[default]
    Right,
    Left,
}

/// Configuration for a single animation sequence
#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            frame_timer: Timer::new(
                Duration::from_secs_f32(1.0 / fps as f32),
                TimerMode::Repeating,
            ),
        }
    }
}

/// Collection of all animation configurations for a character
#[derive(Component)]
pub struct AnimationCollection {
    pub idle: AnimationConfig,
    pub run: AnimationConfig,
}

/// Handles for texture and layout assets used in animations
#[derive(Component)]
pub struct AnimationHandles {
    pub idle_texture: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub run_texture: Handle<Image>,
    pub run_layout: Handle<TextureAtlasLayout>,
}

/// Component for individual tiles in the game world
#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub solid: bool,
}

/// Different types of tiles available
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Ground,
    Platform,
    Decoration,
}

/// Component for parallax scrolling background layers
#[derive(Component)]
pub struct ParallaxLayer {
    pub speed_multiplier: f32,
    pub repeat_width: f32,
    pub layer_depth: f32,
}

/// Component to track which background instance this is (for infinite scrolling)
#[derive(Component)]
pub struct BackgroundIndex {
    pub index: i32,
}

/// Marker component for the main camera to track for parallax
#[derive(Component)]
pub struct MainCamera;

/// Component for managing tile maps
#[derive(Component)]
pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
}

/// Component for tracking camera position for parallax calculations
#[derive(Component)]
pub struct CameraTracker {
    pub last_position: Vec3,
}

/// Component for individual tiles with tileset index
#[derive(Component, Clone, Copy)]
pub struct TileIndex {
    pub index: u32,
    pub tileset_x: u32,
    pub tileset_y: u32,
}

/// Component for tileset information
#[derive(Component)]
pub struct TilesetInfo {
    pub tile_size: u32,
    pub tiles_per_row: u32,
    pub tiles_per_column: u32,
    pub texture_handle: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
}

/// Level data structure for loading from files
#[derive(Clone, Resource)]
pub struct LevelData {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<u32>>, // 2D array of tile indices
}

/// Component for the loaded level
#[derive(Component)]
pub struct Level {
    pub data: LevelData,
    pub tile_size: f32,
}

/// Resource for managing all tilesets
#[derive(Resource)]
pub struct TilesetRegistry {
    pub tilesets: Vec<TilesetInfo>,
    pub current_tileset: usize,
}

/// Resource for tile collision properties based on index
#[derive(Resource)]
pub struct TileCollisionMap {
    pub solid_tiles: std::collections::HashSet<u32>,
    pub platform_tiles: std::collections::HashSet<u32>,
}
