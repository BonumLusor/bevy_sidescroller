//! Tiled map loader integration system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::components::LevelData;

#[derive(Debug, Deserialize, Serialize)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<TiledLayer>,
    pub tilesets: Vec<TiledTileset>,
    #[serde(default)]
    pub properties: Vec<TiledProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiledLayer {
    pub name: String,
    #[serde(rename = "type")]
    pub layer_type: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
    #[serde(default)]
    pub visible: bool,
    #[serde(default)]
    pub opacity: f32,
    #[serde(default)]
    pub properties: Vec<TiledProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiledTileset {
    pub firstgid: u32,
    pub name: String,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub tilecount: u32,
    pub columns: u32,
    pub image: String,
    #[serde(default)]
    pub properties: Vec<TiledProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiledProperty {
    pub name: String,
    #[serde(rename = "type")]
    pub property_type: String,
    pub value: serde_json::Value,
}

#[derive(Debug)]
pub struct TiledLoadError {
    pub message: String,
}

impl std::fmt::Display for TiledLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tiled Load Error: {}", self.message)
    }
}

impl std::error::Error for TiledLoadError {}

impl From<std::io::Error> for TiledLoadError {
    fn from(err: std::io::Error) -> Self {
        TiledLoadError {
            message: format!("IO Error: {}", err),
        }
    }
}

impl From<serde_json::Error> for TiledLoadError {
    fn from(err: serde_json::Error) -> Self {
        TiledLoadError {
            message: format!("JSON Parse Error: {}", err),
        }
    }
}

/// Loads a Tiled map from a JSON file
pub fn load_tiled_map(file_path: &str) -> Result<TiledMap, TiledLoadError> {
    let file_content = std::fs::read_to_string(file_path)?;
    let tiled_map: TiledMap = serde_json::from_str(&file_content)?;
    Ok(tiled_map)
}

/// Converts a Tiled map to LevelData format
pub fn tiled_map_to_level_data(tiled_map: &TiledMap) -> Result<LevelData, TiledLoadError> {
    // Find the main tile layer (first tilelayer)
    let main_layer = tiled_map.layers.iter()
        .find(|layer| layer.layer_type == "tilelayer")
        .ok_or_else(|| TiledLoadError {
            message: "No tile layer found in Tiled map".to_string(),
        })?;

    // Initialize tiles with empty space
    let mut tiles = vec![vec![255; tiled_map.width as usize]; tiled_map.height as usize];

    // Convert Tiled data to our format
    for y in 0..tiled_map.height {
        for x in 0..tiled_map.width {
            let index = (y * tiled_map.width + x) as usize;
            if index < main_layer.data.len() {
                // Tiled uses 1-based indexing for tiles (0 = empty)
                // We use 0-based indexing with 255 = empty
                tiles[y as usize][x as usize] = if main_layer.data[index] > 0 {
                    main_layer.data[index] - 1
                } else {
                    255
                };
            }
        }
    }

    Ok(LevelData {
        width: tiled_map.width,
        height: tiled_map.height,
        tiles,
    })
}

/// Converts a Tiled map to LevelData with custom tile mapping
pub fn tiled_map_to_level_data_with_mapping(
    tiled_map: &TiledMap,
    tile_mapping: &HashMap<u32, u32>,
) -> Result<LevelData, TiledLoadError> {
    let main_layer = tiled_map.layers.iter()
        .find(|layer| layer.layer_type == "tilelayer")
        .ok_or_else(|| TiledLoadError {
            message: "No tile layer found in Tiled map".to_string(),
        })?;

    let mut tiles = vec![vec![255; tiled_map.width as usize]; tiled_map.height as usize];

    for y in 0..tiled_map.height {
        for x in 0..tiled_map.width {
            let index = (y * tiled_map.width + x) as usize;
            if index < main_layer.data.len() {
                let tiled_tile_id = main_layer.data[index];

                if tiled_tile_id > 0 {
                    // Convert from 1-based to 0-based
                    let normalized_id = tiled_tile_id - 1;

                    // Apply mapping if exists, otherwise use normalized id
                    let final_tile_id = tile_mapping.get(&normalized_id)
                        .copied()
                        .unwrap_or(normalized_id);

                    tiles[y as usize][x as usize] = final_tile_id;
                } else {
                    tiles[y as usize][x as usize] = 255; // Empty
                }
            }
        }
    }

    Ok(LevelData {
        width: tiled_map.width,
        height: tiled_map.height,
        tiles,
    })
}

/// Loads multiple layers from a Tiled map
pub fn load_tiled_layers(tiled_map: &TiledMap) -> Result<Vec<(String, LevelData)>, TiledLoadError> {
    let mut layers = Vec::new();

    for layer in &tiled_map.layers {
        if layer.layer_type == "tilelayer" {
            let mut tiles = vec![vec![255; layer.width as usize]; layer.height as usize];

            for y in 0..layer.height {
                for x in 0..layer.width {
                    let index = (y * layer.width + x) as usize;
                    if index < layer.data.len() {
                        tiles[y as usize][x as usize] = if layer.data[index] > 0 {
                            layer.data[index] - 1
                        } else {
                            255
                        };
                    }
                }
            }

            let level_data = LevelData {
                width: layer.width,
                height: layer.height,
                tiles,
            };

            layers.push((layer.name.clone(), level_data));
        }
    }

    Ok(layers)
}

/// Creates a tile mapping from Tiled tileset to game tiles
pub fn create_tile_mapping() -> HashMap<u32, u32> {
    let mut mapping = HashMap::new();

    // Example mappings - customize these based on your Tiled tileset
    // Tiled tile ID -> Game tile ID
    mapping.insert(0, 180);  // First Tiled tile -> Grass
    mapping.insert(1, 176);  // Second Tiled tile -> Stone
    mapping.insert(2, 184);  // Third Tiled tile -> Brick
    mapping.insert(3, 181);  // Fourth Tiled tile -> Platform
    mapping.insert(4, 182);  // Fifth Tiled tile -> Wood
    mapping.insert(5, 183);  // Sixth Tiled tile -> Flower
    mapping.insert(6, 185);  // Seventh Tiled tile -> Tree
    mapping.insert(7, 187);  // Eighth Tiled tile -> Crystal

    mapping
}

/// Validates a Tiled map file
pub fn validate_tiled_map(tiled_map: &TiledMap) -> Result<(), TiledLoadError> {
    // Check if map has at least one tile layer
    if !tiled_map.layers.iter().any(|layer| layer.layer_type == "tilelayer") {
        return Err(TiledLoadError {
            message: "Map must contain at least one tile layer".to_string(),
        });
    }

    // Check if all layers have consistent dimensions
    for layer in &tiled_map.layers {
        if layer.layer_type == "tilelayer" {
            if layer.width != tiled_map.width || layer.height != tiled_map.height {
                return Err(TiledLoadError {
                    message: format!("Layer '{}' has inconsistent dimensions", layer.name),
                });
            }

            // Check if data length matches dimensions
            let expected_data_length = (layer.width * layer.height) as usize;
            if layer.data.len() != expected_data_length {
                return Err(TiledLoadError {
                    message: format!("Layer '{}' has incorrect data length", layer.name),
                });
            }
        }
    }

    Ok(())
}

/// Gets information about a Tiled map
pub fn get_tiled_map_info(tiled_map: &TiledMap) -> String {
    let mut info = format!("Tiled Map Information:\n");
    info.push_str(&format!("  Size: {}x{}\n", tiled_map.width, tiled_map.height));
    info.push_str(&format!("  Tile Size: {}x{}\n", tiled_map.tilewidth, tiled_map.tileheight));
    info.push_str(&format!("  Layers: {}\n", tiled_map.layers.len()));
    info.push_str(&format!("  Tilesets: {}\n", tiled_map.tilesets.len()));

    info.push_str("\nLayers:\n");
    for (i, layer) in tiled_map.layers.iter().enumerate() {
        info.push_str(&format!("  {}: {} ({}x{}) - {}\n",
            i + 1, layer.name, layer.width, layer.height, layer.layer_type));
    }

    info.push_str("\nTilesets:\n");
    for (i, tileset) in tiled_map.tilesets.iter().enumerate() {
        info.push_str(&format!("  {}: {} (GID: {}, Tiles: {})\n",
            i + 1, tileset.name, tileset.firstgid, tileset.tilecount));
    }

    info
}

/// Extracts object layers from Tiled map (for entities, spawn points, etc.)
pub fn extract_object_layers(tiled_map: &TiledMap) -> Vec<TiledObjectLayer> {
    tiled_map.layers.iter()
        .filter(|layer| layer.layer_type == "objectgroup")
        .map(|layer| TiledObjectLayer {
            name: layer.name.clone(),
            // Note: You'd need to add object data parsing here
            // This is a simplified version
        })
        .collect()
}

#[derive(Debug)]
pub struct TiledObjectLayer {
    pub name: String,
    // Add object data fields as needed
}

/// Utility function to convert Tiled coordinates to world coordinates
pub fn tiled_to_world_coords(tiled_x: u32, tiled_y: u32, tile_size: f32) -> (f32, f32) {
    let world_x = tiled_x as f32 * tile_size;
    let world_y = -(tiled_y as f32 * tile_size); // Flip Y for Bevy's coordinate system
    (world_x, world_y)
}

/// Utility function to convert world coordinates to Tiled coordinates
pub fn world_to_tiled_coords(world_x: f32, world_y: f32, tile_size: f32) -> (u32, u32) {
    let tiled_x = (world_x / tile_size).floor() as u32;
    let tiled_y = (-world_y / tile_size).floor() as u32;
    (tiled_x, tiled_y)
}

/// Creates a sample Tiled map for testing
pub fn create_sample_tiled_map() -> TiledMap {
    let sample_data = vec![
        1, 1, 1, 1, 1,
        0, 0, 0, 0, 0,
        0, 0, 2, 0, 0,
        0, 0, 0, 0, 0,
        2, 2, 2, 2, 2,
    ];

    let layer = TiledLayer {
        name: "Ground".to_string(),
        layer_type: "tilelayer".to_string(),
        width: 5,
        height: 5,
        data: sample_data,
        visible: true,
        opacity: 1.0,
        properties: vec![],
    };

    let tileset = TiledTileset {
        firstgid: 1,
        name: "Sample Tileset".to_string(),
        tilewidth: 16,
        tileheight: 16,
        tilecount: 256,
        columns: 16,
        image: "tileset.png".to_string(),
        properties: vec![],
    };

    TiledMap {
        width: 5,
        height: 5,
        tilewidth: 16,
        tileheight: 16,
        layers: vec![layer],
        tilesets: vec![tileset],
        properties: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_tiled_map() {
        let tiled_map = create_sample_tiled_map();
        assert_eq!(tiled_map.width, 5);
        assert_eq!(tiled_map.height, 5);
        assert_eq!(tiled_map.layers.len(), 1);
        assert_eq!(tiled_map.tilesets.len(), 1);
    }

    #[test]
    fn test_tiled_to_level_data_conversion() {
        let tiled_map = create_sample_tiled_map();
        let level_data = tiled_map_to_level_data(&tiled_map).unwrap();

        assert_eq!(level_data.width, 5);
        assert_eq!(level_data.height, 5);

        // Check first row (should be tile 0 after conversion from 1-based)
        assert_eq!(level_data.tiles[0][0], 0);
        assert_eq!(level_data.tiles[0][1], 0);

        // Check empty space (should be 255)
        assert_eq!(level_data.tiles[1][0], 255);
    }

    #[test]
    fn test_coordinate_conversion() {
        let (world_x, world_y) = tiled_to_world_coords(1, 1, 16.0);
        assert_eq!(world_x, 16.0);
        assert_eq!(world_y, -16.0);

        let (tiled_x, tiled_y) = world_to_tiled_coords(16.0, -16.0, 16.0);
        assert_eq!(tiled_x, 1);
        assert_eq!(tiled_y, 1);
    }
}
