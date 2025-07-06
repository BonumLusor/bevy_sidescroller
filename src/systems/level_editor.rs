//! Simple in-game level editor

use crate::components::*;
use crate::constants::*;
use crate::systems::level_loader::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct LevelEditor {
    pub enabled: bool,
    pub current_tile: u32,
    pub brush_size: u32,
    pub show_ui: bool,
}

impl Default for LevelEditor {
    fn default() -> Self {
        Self {
            enabled: false,
            current_tile: 180, // Sua grama
            brush_size: 1,
            show_ui: true,
        }
    }
}

#[derive(Component)]
pub struct EditorTile;

pub fn setup_level_editor(mut commands: Commands) {
    commands.insert_resource(LevelEditor::default());
}

pub fn toggle_level_editor(input: Res<ButtonInput<KeyCode>>, mut editor: ResMut<LevelEditor>) {
    if input.just_pressed(KeyCode::F1) {
        editor.enabled = !editor.enabled;
        info!(
            "Level Editor: {}",
            if editor.enabled { "ON" } else { "OFF" }
        );

        if editor.enabled {
            info!("Editor Controls:");
            info!("- F1: Toggle Editor");
            info!("- Mouse Left: Place tile");
            info!("- Mouse Right: Remove tile");
            info!("- 1-9: Select tile type");
            info!("- [ / ]: Change brush size");
            info!("- S: Save level");
            info!("- L: Load level");
            info!("- H: Toggle UI");
        }
    }

    if editor.enabled && input.just_pressed(KeyCode::KeyH) {
        editor.show_ui = !editor.show_ui;
    }
}

pub fn level_editor_input(input: Res<ButtonInput<KeyCode>>, mut editor: ResMut<LevelEditor>) {
    info!("level_editor_input rodando!"); // DEBUG: Verificar se sistema está ativo

    if input.just_pressed(KeyCode::F1) {
        info!("F1 pressionado!");
    }

    if input.just_pressed(KeyCode::F1) {
        editor.enabled = !editor.enabled;
        info!("Editor enabled? {}", editor.enabled);
    }

    if !editor.enabled {
        return;
    }

    // Trocar tiles
    if input.just_pressed(KeyCode::Digit1) {
        editor.current_tile = 180; // Grama
        info!("Selected: Grass (180)");
    }
    if input.just_pressed(KeyCode::Digit2) {
        editor.current_tile = 176; // Pedra
        info!("Selected: Stone (176)");
    }
    if input.just_pressed(KeyCode::Digit3) {
        editor.current_tile = 184; // Tijolo
        info!("Selected: Brick (184)");
    }
    if input.just_pressed(KeyCode::Digit4) {
        editor.current_tile = 181; // Plataforma
        info!("Selected: Platform (181)");
    }
    if input.just_pressed(KeyCode::Digit5) {
        editor.current_tile = 182; // Madeira
        info!("Selected: Wood (182)");
    }
    if input.just_pressed(KeyCode::Digit6) {
        editor.current_tile = 183; // Flor
        info!("Selected: Flower (183)");
    }
    if input.just_pressed(KeyCode::Digit7) {
        editor.current_tile = 185; // Árvore
        info!("Selected: Tree (185)");
    }
    if input.just_pressed(KeyCode::Digit8) {
        editor.current_tile = 187; // Cristal
        info!("Selected: Crystal (187)");
    }
    if input.just_pressed(KeyCode::Digit9) {
        editor.current_tile = 255; // Vazio
        info!("Selected: Empty (255)");
    }

    // Tamanho do pincel
    if input.just_pressed(KeyCode::BracketLeft) {
        editor.brush_size = (editor.brush_size - 1).max(1);
        info!("Brush size: {}", editor.brush_size);
    }
    if input.just_pressed(KeyCode::BracketRight) {
        editor.brush_size = (editor.brush_size + 1).min(5);
        info!("Brush size: {}", editor.brush_size);
    }
}

pub fn level_editor_mouse(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    editor: Res<LevelEditor>,
    mut commands: Commands,
    level_data: Option<ResMut<LevelData>>,
    tileset_registry: Res<TilesetRegistry>,
    collision_map: Res<TileCollisionMap>,
    existing_tiles: Query<(Entity, &TileIndex, &Transform), With<EditorTile>>,
) {
    info!("level_editor_mouse rodando!");
    if mouse_input.pressed(MouseButton::Left) {
        info!("Mouse esquerdo pressionado!");
    }
    if mouse_input.pressed(MouseButton::Right) {
        info!("Mouse direito pressionado!");
    }
    if !editor.enabled {
        return;
    }

    let Some(mut level_data) = level_data else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            if mouse_input.pressed(MouseButton::Left) {
                info!("Tentando colocar tile em {:?}", world_pos);
                place_tile_at_world_pos(
                    &mut commands,
                    &mut *level_data,
                    world_pos,
                    editor.current_tile,
                    editor.brush_size,
                    &tileset_registry,
                    &collision_map,
                    &existing_tiles,
                );
            }

            if mouse_input.pressed(MouseButton::Right) {
                info!("Tentando remover tile em {:?}", world_pos);
                remove_tile_at_world_pos(
                    &mut commands,
                    &mut *level_data,
                    world_pos,
                    editor.brush_size,
                    &existing_tiles,
                );
            }
        }
    }
}

fn place_tile_at_world_pos(
    commands: &mut Commands,
    level_data: &mut LevelData,
    world_pos: Vec2,
    tile_index: u32,
    brush_size: u32,
    tileset_registry: &TilesetRegistry,
    collision_map: &TileCollisionMap,
    existing_tiles: &Query<(Entity, &TileIndex, &Transform), With<EditorTile>>,
) {
    let center_x = (world_pos.x / TILE_SIZE_16).floor() as i32;
    let center_y = (-world_pos.y / TILE_SIZE_16).floor() as i32;

    let brush_radius = (brush_size / 2) as i32;

    for dy in -brush_radius..=brush_radius {
        for dx in -brush_radius..=brush_radius {
            let tile_x = center_x + dx;
            let tile_y = center_y + dy;

            if tile_x >= 0
                && tile_x < level_data.width as i32
                && tile_y >= 0
                && tile_y < level_data.height as i32
            {
                // Remove existing tile at this position
                let world_tile_pos = Vec3::new(
                    tile_x as f32 * TILE_SIZE_16,
                    -(tile_y as f32 * TILE_SIZE_16),
                    0.0,
                );

                for (entity, _, transform) in existing_tiles.iter() {
                    if transform.translation.distance(world_tile_pos) < TILE_SIZE_16 / 2.0 {
                        commands.entity(entity).despawn();
                    }
                }

                // Update level data
                level_data.tiles[tile_y as usize][tile_x as usize] = tile_index;

                // Create new tile if not empty
                if tile_index != 255 {
                    let tileset_info = &tileset_registry.tilesets[tileset_registry.current_tileset];
                    spawn_editor_tile(
                        commands,
                        tile_index,
                        world_tile_pos,
                        tileset_info,
                        collision_map,
                    );
                }
            }
        }
    }
}

fn remove_tile_at_world_pos(
    commands: &mut Commands,
    level_data: &mut LevelData,
    world_pos: Vec2,
    brush_size: u32,
    existing_tiles: &Query<(Entity, &TileIndex, &Transform), With<EditorTile>>,
) {
    let center_x = (world_pos.x / TILE_SIZE_16).floor() as i32;
    let center_y = (-world_pos.y / TILE_SIZE_16).floor() as i32;

    let brush_radius = (brush_size / 2) as i32;

    for dy in -brush_radius..=brush_radius {
        for dx in -brush_radius..=brush_radius {
            let tile_x = center_x + dx;
            let tile_y = center_y + dy;

            if tile_x >= 0
                && tile_x < level_data.width as i32
                && tile_y >= 0
                && tile_y < level_data.height as i32
            {
                // Remove existing tile at this position
                let world_tile_pos = Vec3::new(
                    tile_x as f32 * TILE_SIZE_16,
                    -(tile_y as f32 * TILE_SIZE_16),
                    0.0,
                );

                for (entity, _, transform) in existing_tiles.iter() {
                    if transform.translation.distance(world_tile_pos) < TILE_SIZE_16 / 2.0 {
                        commands.entity(entity).despawn();
                    }
                }

                // Update level data
                level_data.tiles[tile_y as usize][tile_x as usize] = 255;
            }
        }
    }
}

fn spawn_editor_tile(
    commands: &mut Commands,
    tile_index: u32,
    position: Vec3,
    tileset_info: &TilesetInfo,
    collision_map: &TileCollisionMap,
) {
    info!("spawn_editor_tile: tile_index={}, position={:?}", tile_index, position);
    let tileset_x = tile_index % tileset_info.tiles_per_row;
    let tileset_y = tile_index / tileset_info.tiles_per_row;

    let mut tile_entity = commands.spawn((
        Sprite {
            image: tileset_info.texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: tileset_info.layout_handle.clone(),
                index: tile_index as usize,
            }),
            ..default()
        },
        Transform::from_translation(position),
        TileIndex {
            index: tile_index,
            tileset_x,
            tileset_y,
        },
        EditorTile,
    ));

    // Add collision based on tile type
    if collision_map.solid_tiles.contains(&tile_index) {
        tile_entity.insert(Collider::cuboid(TILE_SIZE_16 / 2.0, TILE_SIZE_16 / 2.0));
    } else if collision_map.platform_tiles.contains(&tile_index) {
        tile_entity.insert(Collider::cuboid(TILE_SIZE_16 / 2.0, TILE_SIZE_16 / 4.0));
    }
}

pub fn level_editor_save_load(
    input: Res<ButtonInput<KeyCode>>,
    editor: Res<LevelEditor>,
    level_data: Option<Res<LevelData>>,
) {
    if !editor.enabled {
        return;
    }

    let Some(level_data) = level_data else {
        return;
    };

    if input.just_pressed(KeyCode::KeyS) {
        match save_level_to_file(&*level_data, "assets/levels/editor_level.csv") {
            Ok(_) => info!("Level saved successfully!"),
            Err(e) => error!("Failed to save level: {}", e),
        }
    }

    if input.just_pressed(KeyCode::KeyL) {
        match load_level_from_file("assets/levels/editor_level.csv") {
            Ok(_loaded_level) => {
                info!("Level loaded successfully!");
                // Note: You'll need to implement level switching logic here
            }
            Err(e) => error!("Failed to load level: {}", e),
        }
    }
}

// UI System for editor
pub fn level_editor_ui(mut contexts: EguiContexts, editor: Res<LevelEditor>) {
    if !editor.enabled || !editor.show_ui {
        return;
    }

    egui::Window::new("Level Editor").default_width(250.0).show(
        contexts.ctx_mut().expect("Failed to get egui context"),
        |ui| {
            ui.label("Level Editor Active");
            ui.separator();

            ui.label(format!("Current Tile: {}", editor.current_tile));
            ui.label(format!("Brush Size: {}", editor.brush_size));

            ui.separator();
            ui.label("Controls:");
            ui.label("F1 - Toggle Editor");
            ui.label("1-9 - Select Tile");
            ui.label("[ / ] - Brush Size");
            ui.label("S - Save Level");
            ui.label("L - Load Level");
            ui.label("H - Toggle UI");

            ui.separator();
            ui.label("Tiles:");
            ui.label("1 - Grass (180)");
            ui.label("2 - Stone (176)");
            ui.label("3 - Brick (184)");
            ui.label("4 - Platform (181)");
            ui.label("5 - Wood (182)");
            ui.label("6 - Flower (183)");
            ui.label("7 - Tree (185)");
            ui.label("8 - Crystal (187)");
            ui.label("9 - Empty (255)");
        },
    );
}
