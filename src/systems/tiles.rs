//! Tile system for world generation and parallax backgrounds

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{BackgroundIndex, MainCamera, ParallaxLayer, PlayerVelocity, Tile, TileType};
use crate::constants::*;

/// Spawns the parallax background layers with proper infinite scrolling
pub fn setup_parallax_backgrounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    let (screen_width, screen_height) = if let Ok(window) = windows.single() {
        (window.width(), window.height())
    } else {
        (crate::constants::DEFAULT_WINDOW_WIDTH, crate::constants::DEFAULT_WINDOW_HEIGHT)
    };

    let layers = [
        ("scene/background_0.png", PARALLAX_BACKGROUND_0_SPEED, -100.0),
        ("scene/background_1.png", PARALLAX_BACKGROUND_1_SPEED, -50.0),
        ("scene/background_2.png", PARALLAX_BACKGROUND_2_SPEED, -10.0),
    ];

    for (texture_path, speed, depth) in layers.iter() {
        // Create 3 instances of each background for seamless scrolling
        for i in -2..=2 {
            let x_position = i as f32 * screen_width;

            commands.spawn((
                Sprite {
                    image: asset_server.load(*texture_path),
                    custom_size: Some(Vec2::new(screen_width, screen_height)),
                    ..default()
                },
                Transform::from_xyz(x_position, screen_height / 2.0, *depth),
                ParallaxLayer {
                    speed_multiplier: *speed,
                    repeat_width: screen_width,
                    layer_depth: *depth,
                },
                BackgroundIndex { index: i },
            ));
        }
    }
}

/// Creates a basic tile map with platforms and ground
pub fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the tileset texture
    let tileset_texture = asset_server.load("scene/tileset.png");

    // Create texture atlas layout for the tileset
    // Assuming the tileset has tiles arranged in a grid
    let tileset_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32), // Each tile is 32x32 pixels
        16,                 // 16 tiles per row
        16,                 // 16 rows
        None,
        None,
    ));

    // Create ground tiles
    let ground_y = GROUND_HEIGHT + GROUND_THICKNESS;
    for x in -25..=25 {
        spawn_tile(
            &mut commands,
            tileset_texture.clone(),
            tileset_layout.clone(),
            Vec3::new(x as f32 * TILE_SIZE, ground_y, 0.0),
            TileType::Ground,
            0,    // Ground tile index
            true, // Solid
        );
    }

    // Create some floating platforms
    let platform_positions = vec![
        (10.0, 200.0),
        (20.0, 300.0),
        (-10.0, 250.0),
        (-20.0, 350.0),
        (0.0, 400.0),
        (15.0, 450.0),
        (-15.0, 500.0),
    ];

    for (x, y) in platform_positions {
        // Create platform (3 tiles wide)
        for i in -1..=1 {
            spawn_tile(
                &mut commands,
                tileset_texture.clone(),
                tileset_layout.clone(),
                Vec3::new(x + (i as f32 * TILE_SIZE), y, 0.0),
                TileType::Platform,
                1,    // Platform tile index
                true, // Solid
            );
        }
    }

    // Create some decorative tiles (trees, rocks, etc.)
    let decoration_positions = vec![
        (-30.0, ground_y + TILE_SIZE),
        (30.0, ground_y + TILE_SIZE),
        (-35.0, ground_y + TILE_SIZE),
        (35.0, ground_y + TILE_SIZE),
    ];

    for (x, y) in decoration_positions {
        spawn_tile(
            &mut commands,
            tileset_texture.clone(),
            tileset_layout.clone(),
            Vec3::new(x, y, 0.0),
            TileType::Decoration,
            2,     // Decoration tile index
            false, // Not solid
        );
    }
}

/// Spawns a single tile at the given position
fn spawn_tile(
    commands: &mut Commands,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    position: Vec3,
    tile_type: TileType,
    atlas_index: usize,
    solid: bool,
) {
    let mut tile_entity = commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout,
                index: atlas_index,
            }),
            ..default()
        },
        Transform::from_translation(position),
    ));

    tile_entity.insert(Tile { tile_type, solid });

    // Add collider for solid tiles
    if solid {
        match tile_type {
            TileType::Ground | TileType::Platform => {
                tile_entity.insert(Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0));
            }
            TileType::Decoration => {
                // Decorations are not solid, no collider needed
            }
        }
    }
}

/// Updates background sizes when window is resized
pub fn update_background_size_on_resize(
    mut background_query: Query<(&mut Sprite, &mut ParallaxLayer), (With<ParallaxLayer>, With<BackgroundIndex>)>,
    windows: Query<&Window>,
    mut resize_reader: EventReader<bevy::window::WindowResized>,
) {
    for _resize_event in resize_reader.read() {
        let (screen_width, screen_height) = if let Ok(window) = windows.single() {
            (window.width(), window.height())
        } else {
            (crate::constants::DEFAULT_WINDOW_WIDTH, crate::constants::DEFAULT_WINDOW_HEIGHT)
        };

        // info!("Window resized to {}x{}", screen_width, screen_height);

        for (mut sprite, mut parallax_layer) in background_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(screen_width, screen_height));
            parallax_layer.repeat_width = screen_width;
        }
    }
}

/// Updates parallax backgrounds based on camera movement
pub fn update_parallax(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut parallax_query: Query<(&mut Transform, &ParallaxLayer, &BackgroundIndex), Without<MainCamera>>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_x = camera_transform.translation.x;
        let camera_y = camera_transform.translation.y;

        // Get screen dimensions for proper positioning
        let _screen_height = 720.0; // Use a default or get from window

        for (mut transform, parallax_layer, bg_index) in parallax_query.iter_mut() {
            let repeat_width = parallax_layer.repeat_width;
            let speed = parallax_layer.speed_multiplier;

            // Use the stored background index for stable positioning
            let original_offset = bg_index.index;

            // Calculate the parallax movement
            let parallax_x = camera_x * speed;

            // Calculate final position: base position - parallax offset
            let base_x = original_offset as f32 * repeat_width;
            let final_x = base_x - parallax_x;

            // Update position
            transform.translation.x = final_x;
            transform.translation.y = camera_y;

            // Handle infinite wrapping - move backgrounds that are too far
            let distance_from_camera = final_x - camera_x;
            if distance_from_camera > repeat_width * 2.0 {
                transform.translation.x -= repeat_width * 5.0;
            } else if distance_from_camera < -repeat_width * 2.0 {
                transform.translation.x += repeat_width * 5.0;
            }
        }
    }
}

/// Updates camera position to follow the player
pub fn update_camera_follow(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerVelocity>)>,
    player_query: Query<&Transform, (With<PlayerVelocity>, Without<MainCamera>)>,
    time: Res<Time>,
) {
    if let (Ok(mut camera_transform), Ok(player_transform)) =
        (camera_query.single_mut(), player_query.single())
    {
        // Calculate target position
        let target_x = player_transform.translation.x;
        let target_y = player_transform.translation.y + CAMERA_OFFSET_Y;

        // Smooth camera following
        let current_pos = camera_transform.translation;
        let target_pos = Vec3::new(target_x, target_y, current_pos.z);

        // Lerp towards target position
        let lerp_factor = CAMERA_FOLLOW_SPEED * time.delta_secs();
        camera_transform.translation = current_pos.lerp(target_pos, lerp_factor);

        // Remove debug spam - only log significant camera moves
        // if camera_transform.translation.distance(current_pos) > 100.0 {
        //     info!("Camera moved to: ({:.1}, {:.1})", target_x, target_y);
        // }
    }
}

/// Cleans up tiles that are far from the camera (for performance)
pub fn cleanup_distant_tiles(
    mut commands: Commands,
    camera_query: Query<&Transform, With<MainCamera>>,
    tile_query: Query<(Entity, &Transform), (With<Tile>, Without<MainCamera>)>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_x = camera_transform.translation.x;
        let cleanup_distance = 1600.0; // Cleanup tiles beyond 2 screen widths

        for (entity, tile_transform) in tile_query.iter() {
            let distance = (tile_transform.translation.x - camera_x).abs();
            if distance > cleanup_distance {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Generates new tiles ahead of the player (procedural generation)
pub fn generate_tiles_ahead(
    _commands: Commands,
    camera_query: Query<&Transform, With<MainCamera>>,
    _asset_server: Res<AssetServer>,
    _texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    existing_tiles: Query<&Transform, With<Tile>>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        let camera_x = camera_transform.translation.x;
        let generation_distance = 800.0; // Generate tiles 1 screen width ahead

        // Check if we need to generate tiles ahead
        let rightmost_tile = existing_tiles
            .iter()
            .map(|t| t.translation.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        if camera_x + generation_distance > rightmost_tile {
            // Generate new tiles ahead
            // This is a simple example - you can make this more sophisticated
            let _start_x = ((rightmost_tile / TILE_SIZE).floor() as i32 + 1) * TILE_SIZE as i32;

            // Note: This is a simplified version. In a real game, you'd want to
            // implement proper procedural generation or load from a level file
        }
    }
}
