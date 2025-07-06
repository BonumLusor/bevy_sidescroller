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
    debug_tile_collisions, debug_tile_grid, debug_tile_info, debug_tileset_info,
    execute_animations, move_player, setup_graphics,
    setup_physics, toggle_debug_render, update_animation_state, update_facing_direction,
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
                setup_physics,
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
                debug_tile_info,
                debug_tile_grid,
                debug_tile_collisions,
                debug_tileset_info,
            ),
        )
        .run();
}
