//! Level loading and tile mapping systems with organized tile constants

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


use crate::components::{
    LevelData, TileCollisionMap, TileIndex, TilesetInfo, TilesetRegistry,
};
use crate::constants::*;

// ========================================
// TILE TYPE CONSTANTS - CUSTOMIZE HERE
// ========================================

// TERRAIN TILES (Solid blocks)
const GRASS_TILES: [u32; 4] = [177, 178, 179, 180];           // Row 0: Grass variants
const STONE_TILES: [u32; 4] = [16, 17, 18, 19];       // Row 1: Stone variants
const BRICK_TILES: [u32; 4] = [20, 21, 22, 23];       // Row 1: Brick variants
const ROCK_TILES: [u32; 4] = [4, 5, 6, 7];            // Row 0: Rock variants

// PLATFORM TILES (Jump-through)
const WOOD_PLATFORMS: [u32; 4] = [32, 33, 34, 35];    // Row 2: Wood platforms
const STONE_PLATFORMS: [u32; 4] = [48, 49, 50, 51];   // Row 3: Stone platforms
const METAL_PLATFORMS: [u32; 4] = [64, 65, 66, 67];   // Row 4: Metal platforms

// DECORATIVE TILES (No collision)
const FLOWERS: [u32; 4] = [8, 9, 10, 11];             // Row 0: Flower decorations
const TREES: [u32; 4] = [24, 25, 26, 27];             // Row 1: Tree decorations
const CRYSTALS: [u32; 4] = [40, 41, 42, 43];          // Row 2: Crystal decorations

// SPECIAL TILES
const SPIKES: [u32; 2] = [80, 81];                     // Row 5: Damage tiles
const WATER: [u32; 4] = [96, 97, 98, 99];             // Row 6: Water tiles
const LAVA: [u32; 4] = [112, 113, 114, 115];          // Row 7: Lava tiles

// EMPTY TILE
const EMPTY_TILE: u32 = 255;  // Air/empty space (not rendered)

// ========================================
// LEVEL LOADING SYSTEM
// ========================================

/// Loads a level and sets up the tileset
pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create tileset info
    let tileset_texture = asset_server.load("scene/tileset.png");
    let tileset_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(TILESET_TILE_SIZE, TILESET_TILE_SIZE),
        TILES_PER_ROW,
        TILES_PER_COLUMN,
        None,
        None,
    ));

    let tileset_info = TilesetInfo {
        tile_size: TILESET_TILE_SIZE,
        tiles_per_row: TILES_PER_ROW,
        tiles_per_column: TILES_PER_COLUMN,
        texture_handle: tileset_texture,
        layout_handle: tileset_layout,
    };

    // Create tile collision mapping using constants
    let collision_map = create_collision_map();

    // Create a default level (you can replace this with file loading)
    let default_level = create_default_level();

    // Spawn the level
    spawn_level_tiles(&mut commands, &default_level, &tileset_info, &collision_map);

    // Insert resources for later use
    commands.insert_resource(TilesetRegistry {
        tilesets: vec![tileset_info],
        current_tileset: 0,
    });
    commands.insert_resource(collision_map);
}

/// Creates collision map using organized tile constants
fn create_collision_map() -> TileCollisionMap {
    let mut solid_tiles = std::collections::HashSet::new();
    let mut platform_tiles = std::collections::HashSet::new();

    // Add all solid tile types
    for &tile in &GRASS_TILES {
        solid_tiles.insert(tile);
    }
    for &tile in &STONE_TILES {
        solid_tiles.insert(tile);
    }
    for &tile in &BRICK_TILES {
        solid_tiles.insert(tile);
    }
    for &tile in &ROCK_TILES {
        solid_tiles.insert(tile);
    }

    // Add all platform tile types
    for &tile in &WOOD_PLATFORMS {
        platform_tiles.insert(tile);
    }
    for &tile in &STONE_PLATFORMS {
        platform_tiles.insert(tile);
    }
    for &tile in &METAL_PLATFORMS {
        platform_tiles.insert(tile);
    }

    // Special tiles can be added to either category
    // Example: Spikes could be solid but dangerous
    for &tile in &SPIKES {
        solid_tiles.insert(tile);
    }

    TileCollisionMap {
        solid_tiles,
        platform_tiles,
    }
}

/// Creates a default level for testing with organized tile usage
fn create_default_level() -> LevelData {
    let width = 50;
    let height = 20;
    let mut tiles = vec![vec![EMPTY_TILE; width as usize]; height as usize];

    // Create ground layer using grass tiles
    for x in 0..width {
        tiles[18][x as usize] = GRASS_TILES[0]; // Main grass tile
        tiles[19][x as usize] = STONE_TILES[0]; // Underground stone
    }

    // Add some variety to the ground
    for x in (0..width).step_by(5) {
        if x + 1 < width {
            tiles[18][x as usize] = GRASS_TILES[1]; // Grass variant
        }
    }

    // Create wooden platforms
    for x in 10..15 {
        tiles[15][x as usize] = WOOD_PLATFORMS[0];
    }
    for x in 25..30 {
        tiles[12][x as usize] = WOOD_PLATFORMS[1];
    }
    for x in 35..40 {
        tiles[9][x as usize] = STONE_PLATFORMS[0];
    }

    // Add decorative elements
    tiles[17][5] = FLOWERS[0];   // Flower decoration
    tiles[17][45] = TREES[0];    // Tree decoration
    tiles[16][20] = CRYSTALS[0]; // Crystal decoration

    // Create walls using stone tiles
    for y in 10..18 {
        tiles[y][0] = STONE_TILES[2];  // Left wall
        tiles[y][49] = STONE_TILES[2]; // Right wall
    }

    // Add some brick accents
    tiles[17][15] = BRICK_TILES[0];
    tiles[17][35] = BRICK_TILES[1];

    LevelData {
        width,
        height,
        tiles,
    }
}

/// Spawns all tiles from level data
fn spawn_level_tiles(
    commands: &mut Commands,
    level_data: &LevelData,
    tileset_info: &TilesetInfo,
    collision_map: &TileCollisionMap,
) {
    for y in 0..level_data.height {
        for x in 0..level_data.width {
            let tile_index = level_data.tiles[y as usize][x as usize];

            // Skip empty tiles
            if tile_index == EMPTY_TILE {
                continue;
            }

            let world_x = x as f32 * TILE_SIZE_16;
            let world_y = -(y as f32 * TILE_SIZE_16); // Flip Y coordinate for screen space

            spawn_tile_at_position(
                commands,
                tile_index,
                Vec3::new(world_x, world_y, 0.0),
                tileset_info,
                collision_map,
            );
        }
    }
}

/// Spawns a single tile at the specified position
fn spawn_tile_at_position(
    commands: &mut Commands,
    tile_index: u32,
    position: Vec3,
    tileset_info: &TilesetInfo,
    collision_map: &TileCollisionMap,
) {
    // Calculate tileset coordinates
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
    ));

    // Add collision based on tile type
    if collision_map.solid_tiles.contains(&tile_index) {
        tile_entity.insert(Collider::cuboid(TILE_SIZE_16 / 2.0, TILE_SIZE_16 / 2.0));
    } else if collision_map.platform_tiles.contains(&tile_index) {
        // Platform collision (thinner for jump-through behavior)
        tile_entity.insert(Collider::cuboid(TILE_SIZE_16 / 2.0, TILE_SIZE_16 / 4.0));
    }
}

/// System to update tile collisions dynamically
pub fn update_tile_collisions(
    mut commands: Commands,
    tile_query: Query<(Entity, &TileIndex), Without<Collider>>,
    collision_map: Res<TileCollisionMap>,
) {
    for (entity, tile_index) in tile_query.iter() {
        if collision_map.solid_tiles.contains(&tile_index.index) {
            commands.entity(entity).insert(Collider::cuboid(TILE_SIZE_16 / 2.0, TILE_SIZE_16 / 2.0));
        } else if collision_map.platform_tiles.contains(&tile_index.index) {
            commands.entity(entity).insert(Collider::cuboid(TILE_SIZE_16 / 2.0, TILE_SIZE_16 / 4.0));
        }
    }
}

// ========================================
// UTILITY FUNCTIONS FOR TILE TYPES
// ========================================

/// Check if a tile index is a solid tile
pub fn is_solid_tile(tile_index: u32) -> bool {
    GRASS_TILES.contains(&tile_index) ||
    STONE_TILES.contains(&tile_index) ||
    BRICK_TILES.contains(&tile_index) ||
    ROCK_TILES.contains(&tile_index) ||
    SPIKES.contains(&tile_index)
}

/// Check if a tile index is a platform tile
pub fn is_platform_tile(tile_index: u32) -> bool {
    WOOD_PLATFORMS.contains(&tile_index) ||
    STONE_PLATFORMS.contains(&tile_index) ||
    METAL_PLATFORMS.contains(&tile_index)
}

/// Check if a tile index is decorative
pub fn is_decorative_tile(tile_index: u32) -> bool {
    FLOWERS.contains(&tile_index) ||
    TREES.contains(&tile_index) ||
    CRYSTALS.contains(&tile_index)
}

/// Get tile type name for debugging
pub fn get_tile_type_name(tile_index: u32) -> &'static str {
    if GRASS_TILES.contains(&tile_index) { "Grass" }
    else if STONE_TILES.contains(&tile_index) { "Stone" }
    else if BRICK_TILES.contains(&tile_index) { "Brick" }
    else if ROCK_TILES.contains(&tile_index) { "Rock" }
    else if WOOD_PLATFORMS.contains(&tile_index) { "Wood Platform" }
    else if STONE_PLATFORMS.contains(&tile_index) { "Stone Platform" }
    else if METAL_PLATFORMS.contains(&tile_index) { "Metal Platform" }
    else if FLOWERS.contains(&tile_index) { "Flower" }
    else if TREES.contains(&tile_index) { "Tree" }
    else if CRYSTALS.contains(&tile_index) { "Crystal" }
    else if SPIKES.contains(&tile_index) { "Spikes" }
    else if WATER.contains(&tile_index) { "Water" }
    else if LAVA.contains(&tile_index) { "Lava" }
    else if tile_index == EMPTY_TILE { "Empty" }
    else { "Unknown" }
}

// ========================================
// LEVEL FILE OPERATIONS
// ========================================

/// Loads level data from a text file
pub fn load_level_from_file(file_path: &str) -> Result<LevelData, Box<dyn std::error::Error>> {
    let level_text = std::fs::read_to_string(file_path)?;
    parse_level_text(&level_text)
}

/// Parses level text in simple CSV format
fn parse_level_text(text: &str) -> Result<LevelData, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = text.lines().collect();

    if lines.is_empty() {
        return Err("Empty level file".into());
    }

    // Parse dimensions
    let dimensions: Vec<&str> = lines[0].split(',').collect();
    let width: u32 = dimensions[0].parse()?;
    let height: u32 = dimensions[1].parse()?;

    let mut tiles = Vec::new();

    // Parse tile data
    for y in 0..height {
        let line_index = (y + 1) as usize;
        if line_index >= lines.len() {
            return Err("Insufficient tile data".into());
        }

        let row_data: Result<Vec<u32>, _> = lines[line_index]
            .split(',')
            .map(|s| s.trim().parse::<u32>())
            .collect();

        tiles.push(row_data?);
    }

    Ok(LevelData {
        width,
        height,
        tiles,
    })
}

/// Saves level data to a file
pub fn save_level_to_file(level_data: &LevelData, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = format!("{},{}\n", level_data.width, level_data.height);

    for row in &level_data.tiles {
        let row_string: Vec<String> = row.iter().map(|&tile| tile.to_string()).collect();
        content.push_str(&row_string.join(","));
        content.push('\n');
    }

    std::fs::write(file_path, content)?;
    Ok(())
}

/// Utility function to get tile at world position
pub fn get_tile_at_position(level_data: &LevelData, world_pos: Vec2) -> Option<u32> {
    let tile_x = (world_pos.x / TILE_SIZE_16).floor() as i32;
    let tile_y = (-world_pos.y / TILE_SIZE_16).floor() as i32;

    if tile_x >= 0 && tile_x < level_data.width as i32 &&
       tile_y >= 0 && tile_y < level_data.height as i32 {
        Some(level_data.tiles[tile_y as usize][tile_x as usize])
    } else {
        None
    }
}

/// Utility function to set tile at world position
pub fn set_tile_at_position(level_data: &mut LevelData, world_pos: Vec2, tile_index: u32) -> bool {
    let tile_x = (world_pos.x / TILE_SIZE_16).floor() as i32;
    let tile_y = (-world_pos.y / TILE_SIZE_16).floor() as i32;

    if tile_x >= 0 && tile_x < level_data.width as i32 &&
       tile_y >= 0 && tile_y < level_data.height as i32 {
        level_data.tiles[tile_y as usize][tile_x as usize] = tile_index;
        true
    } else {
        false
    }
}
