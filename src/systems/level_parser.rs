//! Enhanced level parsing with symbol system

use std::collections::HashMap;
use crate::components::LevelData;

#[derive(Debug, Clone)]
pub struct LevelSymbolMap {
    pub symbols: HashMap<char, u32>,
    pub reverse_map: HashMap<u32, char>,
}

impl LevelSymbolMap {
    pub fn new() -> Self {
        let mut symbols = HashMap::new();

        // Basic tiles
        symbols.insert('.', 255);  // Empty/Air
        symbols.insert('G', 180);  // Grass (your custom position)
        symbols.insert('S', 176);  // Stone
        symbols.insert('B', 184);  // Brick
        symbols.insert('R', 177);  // Rock

        // Platforms
        symbols.insert('P', 181);  // Platform
        symbols.insert('W', 182);  // Wood platform
        symbols.insert('M', 183);  // Metal platform

        // Decorative
        symbols.insert('F', 183);  // Flower
        symbols.insert('T', 185);  // Tree
        symbols.insert('C', 187);  // Crystal

        // Special tiles
        symbols.insert('^', 188);  // Spikes
        symbols.insert('~', 189);  // Water
        symbols.insert('L', 190);  // Lava

        // Walls and structures
        symbols.insert('#', 176);  // Solid wall (stone)
        symbols.insert('=', 181);  // Horizontal platform
        symbols.insert('|', 176);  // Vertical wall
        symbols.insert('+', 184);  // Brick wall
        symbols.insert('*', 187);  // Special block

        // Create reverse mapping
        let reverse_map: HashMap<u32, char> = symbols.iter()
            .map(|(&ch, &tile)| (tile, ch))
            .collect();

        Self { symbols, reverse_map }
    }

    /// Creates a custom symbol map with user-defined mappings
    pub fn custom(mappings: Vec<(char, u32)>) -> Self {
        let mut symbols = HashMap::new();

        // Add default empty tile
        symbols.insert('.', 255);

        // Add custom mappings
        for (symbol, tile_id) in mappings {
            symbols.insert(symbol, tile_id);
        }

        let reverse_map: HashMap<u32, char> = symbols.iter()
            .map(|(&ch, &tile)| (tile, ch))
            .collect();

        Self { symbols, reverse_map }
    }

    /// Gets the tile index for a symbol
    pub fn get_tile(&self, symbol: char) -> Option<u32> {
        self.symbols.get(&symbol).copied()
    }

    /// Gets the symbol for a tile index
    pub fn get_symbol(&self, tile_index: u32) -> Option<char> {
        self.reverse_map.get(&tile_index).copied()
    }

    /// Adds a new symbol mapping
    pub fn add_mapping(&mut self, symbol: char, tile_index: u32) {
        self.symbols.insert(symbol, tile_index);
        self.reverse_map.insert(tile_index, symbol);
    }
}

impl Default for LevelSymbolMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses level data from symbol-based text
pub fn parse_level_from_symbols(text: &str) -> Result<LevelData, String> {
    let symbol_map = LevelSymbolMap::new();
    parse_level_from_symbols_with_map(text, &symbol_map)
}

/// Parses level data from symbol-based text with custom symbol map
pub fn parse_level_from_symbols_with_map(text: &str, symbol_map: &LevelSymbolMap) -> Result<LevelData, String> {
    let lines: Vec<&str> = text.lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with("//"))
        .collect();

    if lines.is_empty() {
        return Err("Empty level file".to_string());
    }

    let height = lines.len() as u32;
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u32;

    let mut tiles = vec![vec![255; width as usize]; height as usize];

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if x < width as usize {
                let tile_index = symbol_map.get_tile(ch).unwrap_or(255);
                tiles[y][x] = tile_index;
            }
        }
    }

    Ok(LevelData {
        width,
        height,
        tiles,
    })
}

/// Converts level data to symbol-based text
pub fn level_to_symbols(level_data: &LevelData) -> String {
    let symbol_map = LevelSymbolMap::new();
    level_to_symbols_with_map(level_data, &symbol_map)
}

/// Converts level data to symbol-based text with custom symbol map
pub fn level_to_symbols_with_map(level_data: &LevelData, symbol_map: &LevelSymbolMap) -> String {
    let mut result = String::new();

    for row in &level_data.tiles {
        for &tile in row {
            let symbol = symbol_map.get_symbol(tile).unwrap_or('?');
            result.push(symbol);
        }
        result.push('\n');
    }

    result
}

/// Loads level from a symbol-based text file
pub fn load_level_from_symbol_file(file_path: &str) -> Result<LevelData, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file_path)?;
    parse_level_from_symbols(&content).map_err(|e| e.into())
}

/// Saves level to a symbol-based text file
pub fn save_level_to_symbol_file(level_data: &LevelData, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = level_to_symbols(level_data);
    std::fs::write(file_path, content)?;
    Ok(())
}

/// Creates a sample level in symbol format
pub fn create_sample_symbol_level() -> String {
    r#"
// Sample Level - Lines starting with // are comments
// Symbols:
// . = Empty space
// G = Grass
// S = Stone
// P = Platform
// F = Flower
// T = Tree
// # = Wall
// = = Horizontal platform

.....T.........T.....
.....................
...................F.
..........===........
.....................
...#.................
...#.....P...........
...#.................
GGGGGGGGGGGGGGGGGGGGG
SSSSSSSSSSSSSSSSSSSSS
"#.trim().to_string()
}

/// Validates symbol-based level text
pub fn validate_symbol_level(text: &str) -> Result<(), String> {
    let symbol_map = LevelSymbolMap::new();
    let lines: Vec<&str> = text.lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with("//"))
        .collect();

    if lines.is_empty() {
        return Err("Level contains no valid lines".to_string());
    }

    let expected_width = lines[0].len();
    for (i, line) in lines.iter().enumerate() {
        if line.len() != expected_width {
            return Err(format!("Line {} has different width than first line", i + 1));
        }

        for (j, ch) in line.chars().enumerate() {
            if symbol_map.get_tile(ch).is_none() {
                return Err(format!("Unknown symbol '{}' at line {}, column {}", ch, i + 1, j + 1));
            }
        }
    }

    Ok(())
}

/// Gets information about available symbols
pub fn get_symbol_info() -> String {
    let symbol_map = LevelSymbolMap::new();
    let mut info = String::from("Available symbols:\n");

    let mut symbols: Vec<_> = symbol_map.symbols.iter().collect();
    symbols.sort_by_key(|(ch, _)| *ch);

    for (symbol, tile_id) in symbols {
        let description = match *symbol {
            '.' => "Empty space",
            'G' => "Grass",
            'S' => "Stone",
            'B' => "Brick",
            'R' => "Rock",
            'P' => "Platform",
            'W' => "Wood platform",
            'M' => "Metal platform",
            'F' => "Flower",
            'T' => "Tree",
            'C' => "Crystal",
            '^' => "Spikes",
            '~' => "Water",
            'L' => "Lava",
            '#' => "Solid wall",
            '=' => "Horizontal platform",
            '|' => "Vertical wall",
            '+' => "Brick wall",
            '*' => "Special block",
            _ => "Unknown",
        };

        info.push_str(&format!("  '{}' - {} (tile {})\n", symbol, description, tile_id));
    }

    info
}

/// Creates a blank level template
pub fn create_blank_symbol_level(width: u32, height: u32) -> String {
    let mut result = String::new();

    for _ in 0..height {
        for _ in 0..width {
            result.push('.');
        }
        result.push('\n');
    }

    result
}

/// Replaces symbols in level text
pub fn replace_symbols_in_level(text: &str, replacements: &[(char, char)]) -> String {
    let mut result = text.to_string();

    for (from, to) in replacements {
        result = result.replace(*from, &to.to_string());
    }

    result
}

/// Counts symbols in level text
pub fn count_symbols_in_level(text: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for line in text.lines() {
        for ch in line.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
    }

    counts
}

/// Creates a level editor helper that shows symbol mappings
pub fn create_symbol_reference() -> String {
    format!("Symbol Reference for Level Editor:\n{}\n\nExample level:\n{}",
            get_symbol_info(),
            create_sample_symbol_level())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_mapping() {
        let symbol_map = LevelSymbolMap::new();
        assert_eq!(symbol_map.get_tile('G'), Some(180));
        assert_eq!(symbol_map.get_tile('.'), Some(255));
        assert_eq!(symbol_map.get_symbol(180), Some('G'));
        assert_eq!(symbol_map.get_symbol(255), Some('.'));
    }

    #[test]
    fn test_parse_simple_level() {
        let level_text = "GGG\nSSS\n...";
        let result = parse_level_from_symbols(level_text);
        assert!(result.is_ok());

        let level_data = result.unwrap();
        assert_eq!(level_data.width, 3);
        assert_eq!(level_data.height, 3);
        assert_eq!(level_data.tiles[0][0], 180); // G
        assert_eq!(level_data.tiles[1][0], 176); // S
        assert_eq!(level_data.tiles[2][0], 255); // .
    }

    #[test]
    fn test_level_validation() {
        let valid_level = "GGG\nSSS\n...";
        assert!(validate_symbol_level(valid_level).is_ok());

        let invalid_level = "GGG\nSS\n..."; // Different widths
        assert!(validate_symbol_level(invalid_level).is_err());
    }
}
