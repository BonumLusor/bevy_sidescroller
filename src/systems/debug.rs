//! Debug systems for development and testing

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{TileIndex, TilesetRegistry};

/// Toggles the Rapier physics debug rendering on/off with F3 key
pub fn toggle_debug_render(
    mut debug_context: ResMut<DebugRenderContext>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug_context.enabled = !debug_context.enabled;
        info!(
            "Rapier debug rendering toggled: {}",
            if debug_context.enabled { "ON" } else { "OFF" }
        );
    }
}

/// Debug system to display player information in console
/// This can be enabled/disabled for debugging purposes
#[allow(dead_code)]
pub fn debug_player_info(
    query: Query<(
        &Transform,
        &crate::components::PlayerVelocity,
        &crate::components::AnimationState,
    )>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        for (transform, velocity, animation_state) in query.iter() {
            info!(
                "Player Debug Info - Position: ({:.2}, {:.2}), Velocity: ({:.2}, {:.2}), Animation: {:?}",
                transform.translation.x,
                transform.translation.y,
                velocity.0.x,
                velocity.0.y,
                animation_state
            );
        }
    }
}

/// Debug system to display FPS information
/// Toggle with F2 key
pub fn debug_fps(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut show_fps: Local<bool>,
    mut timer: Local<f32>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
        *show_fps = !*show_fps;
        info!("FPS Debug: {}", if *show_fps { "ON" } else { "OFF" });
    }

    if *show_fps {
        *timer += time.delta_secs();
        if *timer >= 1.0 {
            let fps = 1.0 / time.delta_secs();
            info!("FPS: {:.1}", fps);
            *timer = 0.0;
        }
    }
}

/// Debug system to display tile information
pub fn debug_tile_info(
    _camera_query: Query<&GlobalTransform, With<crate::components::MainCamera>>,
    tile_query: Query<(&Transform, &TileIndex)>,
    windows: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut show_tile_debug: Local<bool>,
) {
    // Toggle tile debug with F4
    if keyboard.just_pressed(KeyCode::F4) {
        *show_tile_debug = !*show_tile_debug;
        info!("Tile Debug: {}", if *show_tile_debug { "ON" } else { "OFF" });
    }

    if !*show_tile_debug {
        return;
    }

    // Get mouse position (simplified - you'd need proper screen-to-world conversion)
    if let Ok(window) = windows.single() {
        if let Some(cursor_pos) = window.cursor_position() {
            // Convert screen coordinates to world coordinates (simplified)
            let world_x = cursor_pos.x - window.width() / 2.0;
            let world_y = -(cursor_pos.y - window.height() / 2.0);

            // Find tiles near cursor
            for (transform, tile_index) in tile_query.iter() {
                let distance = transform.translation.truncate().distance(Vec2::new(world_x, world_y));
                if distance < 32.0 {
                    info!(
                        "Tile at ({:.1}, {:.1}): Index {}, Tileset pos ({}, {})",
                        transform.translation.x,
                        transform.translation.y,
                        tile_index.index,
                        tile_index.tileset_x,
                        tile_index.tileset_y
                    );
                    break;
                }
            }
        }
    }
}

/// Debug system to show tile grid overlay
pub fn debug_tile_grid(
    mut gizmos: Gizmos,
    camera_query: Query<&Transform, With<crate::components::MainCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut show_grid: Local<bool>,
) {
    // Toggle grid with F5
    if keyboard.just_pressed(KeyCode::F5) {
        *show_grid = !*show_grid;
        info!("Tile Grid: {}", if *show_grid { "ON" } else { "OFF" });
    }

    if !*show_grid {
        return;
    }

    if let Ok(camera_transform) = camera_query.single() {
        let camera_pos = camera_transform.translation;
        let tile_size = crate::constants::TILE_SIZE_16;

        // Draw grid around camera
        let grid_range = 20;
        for x in -grid_range..=grid_range {
            for y in -grid_range..=grid_range {
                let world_x = (camera_pos.x / tile_size).floor() * tile_size + x as f32 * tile_size;
                let world_y = (camera_pos.y / tile_size).floor() * tile_size + y as f32 * tile_size;

                // Draw tile boundary
                gizmos.rect_2d(
                    Vec2::new(world_x, world_y),
                    Vec2::splat(tile_size),
                    Color::srgba(0.0, 1.0, 0.0, 0.3),
                );
            }
        }
    }
}

/// Debug system to highlight tiles with collision
pub fn debug_tile_collisions(
    mut gizmos: Gizmos,
    tile_query: Query<&Transform, (With<TileIndex>, With<Collider>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut show_collisions: Local<bool>,
) {
    // Toggle collision debug with F6
    if keyboard.just_pressed(KeyCode::F6) {
        *show_collisions = !*show_collisions;
        info!("Tile Collisions: {}", if *show_collisions { "ON" } else { "OFF" });
    }

    if !*show_collisions {
        return;
    }

    // Highlight tiles with collision
    for transform in tile_query.iter() {
        gizmos.rect_2d(
            transform.translation.truncate(),
            Vec2::splat(crate::constants::TILE_SIZE_16),
            Color::srgba(1.0, 0.0, 0.0, 0.5),
        );
    }
}

/// Debug system to show tileset information
pub fn debug_tileset_info(
    tileset_registry: Option<Res<TilesetRegistry>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F7) {
        if let Some(registry) = tileset_registry {
            info!("=== Tileset Information ===");
            for (i, tileset) in registry.tilesets.iter().enumerate() {
                info!(
                    "Tileset {}: {}x{} tiles, tile size: {}px",
                    i,
                    tileset.tiles_per_row,
                    tileset.tiles_per_column,
                    tileset.tile_size
                );
            }
            info!("Current tileset: {}", registry.current_tileset);
        } else {
            info!("No tileset registry found");
        }
    }
}
