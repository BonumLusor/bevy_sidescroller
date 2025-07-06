//! Level building templates and patterns system

use crate::components::LevelData;


#[derive(Debug, Clone)]
pub struct LevelTemplate {
    pub name: String,
    pub pattern: Vec<Vec<u32>>,
    pub width: u32,
    pub height: u32,
}

impl LevelTemplate {
    /// Creates a ground platform template
    pub fn ground_platform(width: u32, grass_tile: u32, stone_tile: u32) -> Self {
        let mut pattern = vec![vec![grass_tile; width as usize]];
        pattern.push(vec![stone_tile; width as usize]);

        Self {
            name: "Ground Platform".to_string(),
            pattern,
            width,
            height: 2,
        }
    }

    /// Creates a floating platform template
    pub fn floating_platform(width: u32, platform_tile: u32) -> Self {
        let pattern = vec![vec![platform_tile; width as usize]];

        Self {
            name: "Floating Platform".to_string(),
            pattern,
            width,
            height: 1,
        }
    }

    /// Creates a vertical pillar template
    pub fn pillar(height: u32, stone_tile: u32) -> Self {
        let pattern = vec![vec![stone_tile]; height as usize];

        Self {
            name: "Stone Pillar".to_string(),
            pattern,
            width: 1,
            height,
        }
    }

    /// Creates a staircase template
    pub fn staircase(width: u32, height: u32, stone_tile: u32) -> Self {
        let mut pattern = vec![vec![255; width as usize]; height as usize];

        for y in 0..height {
            for x in 0..width {
                if x >= y {
                    pattern[y as usize][x as usize] = stone_tile;
                }
            }
        }

        Self {
            name: "Staircase".to_string(),
            pattern,
            width,
            height,
        }
    }

    /// Creates a room template
    pub fn room(width: u32, height: u32, wall_tile: u32, floor_tile: u32) -> Self {
        let mut pattern = vec![vec![255; width as usize]; height as usize];

        // Top and bottom walls
        for x in 0..width {
            pattern[0][x as usize] = wall_tile;
            pattern[(height - 1) as usize][x as usize] = floor_tile;
        }

        // Left and right walls
        for y in 0..height {
            pattern[y as usize][0] = wall_tile;
            pattern[y as usize][(width - 1) as usize] = wall_tile;
        }

        Self {
            name: "Room".to_string(),
            pattern,
            width,
            height,
        }
    }

    /// Creates a bridge template
    pub fn bridge(width: u32, platform_tile: u32, support_tile: u32) -> Self {
        let mut pattern = vec![vec![255; width as usize]; 3];

        // Bridge deck
        for x in 0..width {
            pattern[0][x as usize] = platform_tile;
        }

        // Support pillars at ends
        pattern[1][0] = support_tile;
        pattern[2][0] = support_tile;
        pattern[1][(width - 1) as usize] = support_tile;
        pattern[2][(width - 1) as usize] = support_tile;

        Self {
            name: "Bridge".to_string(),
            pattern,
            width,
            height: 3,
        }
    }

    /// Creates a tower template
    pub fn tower(width: u32, height: u32, wall_tile: u32, floor_tile: u32) -> Self {
        let mut pattern = vec![vec![255; width as usize]; height as usize];

        // Walls
        for y in 0..height {
            pattern[y as usize][0] = wall_tile;
            pattern[y as usize][(width - 1) as usize] = wall_tile;
        }

        // Floor every 4 levels
        for y in (3..height).step_by(4) {
            for x in 1..(width - 1) {
                pattern[y as usize][x as usize] = floor_tile;
            }
        }

        // Base
        for x in 0..width {
            pattern[(height - 1) as usize][x as usize] = floor_tile;
        }

        Self {
            name: "Tower".to_string(),
            pattern,
            width,
            height,
        }
    }

    /// Creates a pit template
    pub fn pit(width: u32, depth: u32, wall_tile: u32) -> Self {
        let mut pattern = vec![vec![255; width as usize]; depth as usize];

        // Pit walls
        for y in 0..depth {
            pattern[y as usize][0] = wall_tile;
            pattern[y as usize][(width - 1) as usize] = wall_tile;
        }

        // Pit bottom
        for x in 0..width {
            pattern[(depth - 1) as usize][x as usize] = wall_tile;
        }

        Self {
            name: "Pit".to_string(),
            pattern,
            width,
            height: depth,
        }
    }

    /// Creates a decorative garden template
    pub fn garden(width: u32, height: u32, grass_tile: u32, flower_tile: u32, tree_tile: u32) -> Self {
        let mut pattern = vec![vec![grass_tile; width as usize]; height as usize];

        // Add some flowers randomly
        for y in 0..(height - 1) {
            for x in 0..width {
                if (x + y) % 3 == 0 {
                    pattern[y as usize][x as usize] = flower_tile;
                }
            }
        }

        // Add trees at corners
        if width > 2 && height > 2 {
            pattern[0][1] = tree_tile;
            pattern[0][(width - 2) as usize] = tree_tile;
        }

        Self {
            name: "Garden".to_string(),
            pattern,
            width,
            height,
        }
    }

    /// Creates a cave entrance template
    pub fn cave_entrance(width: u32, height: u32, stone_tile: u32) -> Self {
        let mut pattern = vec![vec![255; width as usize]; height as usize];

        // Create arch shape
        let center_x = width / 2;
        let center_y = height / 2;

        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                let distance = (dx * dx + dy * dy) as f32;
                let radius = (center_x.min(center_y) as f32).powi(2);

                if distance > radius {
                    pattern[y as usize][x as usize] = stone_tile;
                }
            }
        }

        Self {
            name: "Cave Entrance".to_string(),
            pattern,
            width,
            height,
        }
    }
}

/// Places a template in the level data at the specified position
pub fn place_template(
    level_data: &mut LevelData,
    template: &LevelTemplate,
    start_x: u32,
    start_y: u32,
) -> bool {
    if start_x + template.width > level_data.width || start_y + template.height > level_data.height {
        return false;
    }

    for (y, row) in template.pattern.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            let world_x = start_x + x as u32;
            let world_y = start_y + y as u32;

            if tile != 255 { // Only place non-empty tiles
                level_data.tiles[world_y as usize][world_x as usize] = tile;
            }
        }
    }

    true
}

/// Places multiple templates in sequence
pub fn place_templates(
    level_data: &mut LevelData,
    templates: &[(LevelTemplate, u32, u32)], // (template, x, y)
) -> Vec<bool> {
    templates.iter()
        .map(|(template, x, y)| place_template(level_data, template, *x, *y))
        .collect()
}

/// Creates a collection of common templates using your custom tile indices
pub fn create_common_templates() -> Vec<LevelTemplate> {
    vec![
        // Using your grass tile (180) and stone tile (176)
        LevelTemplate::ground_platform(10, 180, 176),
        LevelTemplate::floating_platform(5, 181), // Platform tile
        LevelTemplate::pillar(6, 176), // Stone pillar
        LevelTemplate::staircase(5, 5, 176),
        LevelTemplate::room(8, 6, 176, 180),
        LevelTemplate::bridge(8, 181, 176),
        LevelTemplate::tower(4, 12, 176, 180),
        LevelTemplate::pit(4, 3, 176),
        LevelTemplate::garden(6, 4, 180, 183, 185), // Grass, flower, tree
        LevelTemplate::cave_entrance(6, 6, 176),
    ]
}

/// Creates a level using template-based generation
pub fn create_template_level(width: u32, height: u32) -> LevelData {
    let mut level_data = LevelData {
        width,
        height,
        tiles: vec![vec![255; width as usize]; height as usize],
    };

    let templates = create_common_templates();

    // Place ground
    if let Some(ground_template) = templates.get(0) {
        for x in (0..width).step_by(ground_template.width as usize) {
            place_template(&mut level_data, ground_template, x, height - 2);
        }
    }

    // Place some floating platforms
    if let Some(platform_template) = templates.get(1) {
        place_template(&mut level_data, platform_template, 10, height - 8);
        place_template(&mut level_data, platform_template, 20, height - 12);
        place_template(&mut level_data, platform_template, 30, height - 6);
    }

    // Place some pillars
    if let Some(pillar_template) = templates.get(2) {
        place_template(&mut level_data, pillar_template, 5, height - 8);
        place_template(&mut level_data, pillar_template, 35, height - 8);
    }

    // Place a room
    if let Some(room_template) = templates.get(4) {
        place_template(&mut level_data, room_template, width - 10, height - 8);
    }

    level_data
}

/// Utility function to preview a template as a string
pub fn template_to_string(template: &LevelTemplate) -> String {
    let mut result = format!("Template: {} ({}x{})\n", template.name, template.width, template.height);

    for row in &template.pattern {
        for &tile in row {
            let char = match tile {
                255 => '.',
                180 => 'G', // Grass
                176 => 'S', // Stone
                181 => 'P', // Platform
                183 => 'F', // Flower
                185 => 'T', // Tree
                _ => '#',
            };
            result.push(char);
        }
        result.push('\n');
    }

    result
}

/// Validates that a template fits within level bounds
pub fn validate_template_placement(
    level_data: &LevelData,
    template: &LevelTemplate,
    x: u32,
    y: u32,
) -> bool {
    x + template.width <= level_data.width && y + template.height <= level_data.height
}

/// Gets all possible positions where a template can be placed
pub fn get_valid_positions(
    level_data: &LevelData,
    template: &LevelTemplate,
) -> Vec<(u32, u32)> {
    let mut positions = Vec::new();

    for y in 0..level_data.height {
        for x in 0..level_data.width {
            if validate_template_placement(level_data, template, x, y) {
                positions.push((x, y));
            }
        }
    }

    positions
}
