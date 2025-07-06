//! Bevy Sidescroller Game
//!
//! A 2D sidescroller game built with Bevy and Rapier2D physics.
//! Features character movement, animations, and sprite flipping.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_egui::EguiPlugin;

mod components;
mod constants;
mod systems;

use constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, PIXELS_PER_METER};
use systems::{
    setup_level_editor,
    level_editor_input,
    level_editor_mouse,
    level_editor_save_load,
    level_editor_ui,
    debug_tile_collisions, debug_tile_grid, debug_tile_info, debug_tileset_info,
    execute_animations, load_level, move_player, setup_graphics, setup_parallax_backgrounds,
    setup_physics, toggle_debug_render, update_animation_state, update_background_size_on_resize,
    update_camera_follow, update_facing_direction, update_parallax, update_tile_collisions,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Sidescroller".into(),
                resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                setup_graphics,
                setup_parallax_backgrounds,
                setup_physics,
                load_level,
                setup_level_editor,
            ),
        )
        .add_systems(
            Update,
            (
                toggle_debug_render,
                move_player,
                update_facing_direction,
                update_animation_state,
                execute_animations,
                update_camera_follow,
                update_parallax,
                update_background_size_on_resize,
                // Sistemas do editor de level
                level_editor_input,
                level_editor_mouse,
                level_editor_save_load,
                level_editor_ui,
                // Sistemas de debug e tile
                update_tile_collisions,
                debug_tile_info,
                debug_tile_grid,
                debug_tile_collisions,
                debug_tileset_info,
            ),
        )
        .run();
}
