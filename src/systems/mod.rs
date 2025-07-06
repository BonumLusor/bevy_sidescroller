//! Systems module for the sidescroller game
//!
//! This module organizes all game systems into logical groups:
//! - Setup: Systems for initializing the game world
//! - Movement: Systems for handling player movement and physics
//! - Animation: Systems for sprite animations and visual effects
//! - Tiles: Systems for tile-based world generation and parallax backgrounds
//! - Level Loader: Systems for loading and managing tile-based levels
//! - Debug: Systems for debugging and development tools

pub mod animation;
pub mod debug;
pub mod level_editor;
pub mod level_loader;
pub mod level_parser;
pub mod level_templates;
pub mod movement;
pub mod setup;
pub mod tiles;
pub mod tiled_loader;

// Re-export commonly used systems for easier importing
pub use animation::{execute_animations, update_animation_state};
pub use debug::{debug_tile_collisions, debug_tile_grid, debug_tile_info, debug_tileset_info, toggle_debug_render};
pub use level_editor::{setup_level_editor, toggle_level_editor, level_editor_input, level_editor_mouse, level_editor_save_load, level_editor_ui};
pub use level_loader::{load_level, update_tile_collisions};
// pub use level_parser::{parse_level_from_symbols, load_level_from_symbol_file, save_level_to_symbol_file};
// pub use level_templates::{LevelTemplate, place_template, create_common_templates, create_template_level};
pub use movement::{move_player, update_facing_direction};
pub use setup::{setup_graphics, setup_physics};
pub use tiles::{
    setup_parallax_backgrounds, update_background_size_on_resize,
    update_camera_follow, update_parallax,
};
// pub use tiled_loader::{load_tiled_map, tiled_map_to_level_data, create_tile_mapping};
