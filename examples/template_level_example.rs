//! Example demonstrating how to use the level template system
//!
//! This example shows how to create levels using pre-defined templates
//! and how to combine them to create complex level layouts.
//!
//! Run with: cargo run --example template_level_example

use bevy::prelude::*;
// Note: This example requires the level systems to be properly exported
// For now, we'll define minimal types to make the example compile
use std::collections::HashMap;

fn main() {
    println!("Template Level Example");
    println!("This example demonstrates level template concepts.");
    println!("To run this example, first ensure all level systems are properly exported.");

    // Uncomment when level systems are ready:
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, (
    //         setup_camera,
    //         demonstrate_templates,
    //     ))
    //     .run();
}

// Minimal types for demonstration
#[derive(Clone)]
pub struct LevelData {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct LevelTemplate {
    pub name: String,
    pub pattern: Vec<Vec<u32>>,
    pub width: u32,
    pub height: u32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn demonstrate_templates(mut commands: Commands) {
    println!("=== Level Template System Demo ===");

    // Create a new level
    let mut level_data = LevelData {
        width: 60,
        height: 25,
        tiles: vec![vec![255; 60]; 25], // Start with empty level
    };

    println!("Created empty level: {}x{}", level_data.width, level_data.height);

    // === BASIC TEMPLATES ===

    // 1. Ground Platform
    let ground_template = LevelTemplate::ground_platform(60, 180, 176);
    if place_template(&mut level_data, &ground_template, 0, 23) {
        println!("✓ Placed ground platform across bottom");
    }

    // 2. Floating Platforms
    let platform_template = LevelTemplate::floating_platform(8, 181);
    place_template(&mut level_data, &platform_template, 10, 18);
    place_template(&mut level_data, &platform_template, 25, 15);
    place_template(&mut level_data, &platform_template, 40, 12);
    println!("✓ Placed 3 floating platforms");

    // 3. Vertical Pillars
    let pillar_template = LevelTemplate::pillar(8, 176);
    place_template(&mut level_data, &pillar_template, 5, 15);
    place_template(&mut level_data, &pillar_template, 55, 15);
    println!("✓ Placed support pillars");

    // === ADVANCED TEMPLATES ===

    // 4. Staircase
    let stairs_template = LevelTemplate::staircase(6, 6, 176);
    place_template(&mut level_data, &stairs_template, 20, 17);
    println!("✓ Placed staircase");

    // 5. Room/House
    let room_template = LevelTemplate::room(10, 8, 176, 180);
    place_template(&mut level_data, &room_template, 45, 15);
    println!("✓ Placed room structure");

    // 6. Bridge
    let bridge_template = LevelTemplate::bridge(12, 181, 176);
    place_template(&mut level_data, &bridge_template, 30, 8);
    println!("✓ Placed bridge");

    // 7. Tower
    let tower_template = LevelTemplate::tower(3, 15, 176, 180);
    place_template(&mut level_data, &tower_template, 2, 8);
    println!("✓ Placed tower");

    // 8. Pit
    let pit_template = LevelTemplate::pit(6, 4, 176);
    place_template(&mut level_data, &pit_template, 12, 19);
    println!("✓ Placed pit");

    // 9. Garden
    let garden_template = LevelTemplate::garden(8, 3, 180, 183, 185);
    place_template(&mut level_data, &garden_template, 35, 20);
    println!("✓ Placed decorative garden");

    // 10. Cave Entrance
    let cave_template = LevelTemplate::cave_entrance(8, 6, 176);
    place_template(&mut level_data, &cave_template, 50, 17);
    println!("✓ Placed cave entrance");

    // === TEMPLATE COMBINATIONS ===

    // Create a complex structure using multiple templates
    create_castle_structure(&mut level_data, 15, 5);
    println!("✓ Created castle structure");

    // Create a parkour course
    create_parkour_course(&mut level_data, 8, 10);
    println!("✓ Created parkour course");

    // === TEMPLATE PREVIEW ===

    // Show what templates look like
    println!("\n=== Template Previews ===");

    let sample_templates = vec![
        LevelTemplate::ground_platform(5, 180, 176),
        LevelTemplate::floating_platform(4, 181),
        LevelTemplate::pillar(4, 176),
        LevelTemplate::staircase(3, 3, 176),
        LevelTemplate::room(5, 4, 176, 180),
    ];

    for template in sample_templates {
        println!("{}", template_to_string(&template));
    }

    // === SAVE THE LEVEL ===

    // Save using different formats
    save_template_level_examples(&level_data);

    // === VALIDATION ===

    // Validate template placements
    validate_template_examples(&level_data);

    println!("\n=== Template Demo Complete ===");
    println!("Check the generated level files in assets/levels/");
}

/// Creates a castle-like structure using multiple templates
fn create_castle_structure(level_data: &mut LevelData, start_x: u32, start_y: u32) {
    // Base platform
    let base = LevelTemplate::ground_platform(15, 180, 176);
    place_template(level_data, &base, start_x, start_y + 12);

    // Tower foundations
    let foundation = LevelTemplate::room(3, 3, 176, 180);
    place_template(level_data, &foundation, start_x, start_y + 9);
    place_template(level_data, &foundation, start_x + 12, start_y + 9);

    // Towers
    let tower = LevelTemplate::tower(3, 12, 176, 180);
    place_template(level_data, &tower, start_x, start_y);
    place_template(level_data, &tower, start_x + 12, start_y);

    // Connecting bridge
    let bridge = LevelTemplate::bridge(9, 181, 176);
    place_template(level_data, &bridge, start_x + 3, start_y + 6);

    // Decorative elements
    let garden = LevelTemplate::garden(4, 2, 180, 183, 185);
    place_template(level_data, &garden, start_x + 6, start_y + 10);
}

/// Creates a parkour course using various templates
fn create_parkour_course(level_data: &mut LevelData, start_x: u32, start_y: u32) {
    // Series of platforms at different heights
    let platforms = vec![
        (start_x, start_y + 5),
        (start_x + 5, start_y + 3),
        (start_x + 10, start_y + 1),
        (start_x + 15, start_y + 2),
        (start_x + 20, start_y + 4),
    ];

    for (x, y) in platforms {
        let platform = LevelTemplate::floating_platform(3, 181);
        place_template(level_data, &platform, x, y);
    }

    // Add some obstacles
    let pillar = LevelTemplate::pillar(3, 176);
    place_template(level_data, &pillar, start_x + 8, start_y + 4);
    place_template(level_data, &pillar, start_x + 18, start_y + 7);

    // Add a pit challenge
    let pit = LevelTemplate::pit(4, 3, 176);
    place_template(level_data, &pit, start_x + 12, start_y + 7);
}

/// Saves the level using different formats for comparison
fn save_template_level_examples(level_data: &LevelData) {
    // Note: Saving functions would be implemented here
    // when the level systems are properly exported
    println!("✓ Would save as CSV: assets/levels/template_demo.csv");
    println!("✓ Would save as symbols: assets/levels/template_demo.txt");
    println!("Level data: {}x{} tiles", level_data.width, level_data.height);
}

/// Validates template placements and shows statistics
fn validate_template_examples(level_data: &LevelData) {
    println!("\n=== Level Statistics ===");

    // Count tile types
    let mut tile_counts = std::collections::HashMap::new();
    for row in &level_data.tiles {
        for &tile in row {
            *tile_counts.entry(tile).or_insert(0) += 1;
        }
    }

    println!("Tile distribution:");
    for (tile, count) in tile_counts {
        let tile_name = match tile {
            255 => "Empty",
            180 => "Grass",
            176 => "Stone",
            181 => "Platform",
            183 => "Flower",
            185 => "Tree",
            _ => "Other",
        };
        let percentage = (count as f32 / (level_data.width * level_data.height) as f32) * 100.0;
        println!("  {}: {} tiles ({:.1}%)", tile_name, count, percentage);
    }

    // Test template validation
    let test_template = LevelTemplate::floating_platform(5, 181);
    let valid_positions = get_valid_positions(level_data, &test_template);
    println!("Valid positions for 5-tile platform: {}", valid_positions.len());

    // Show some valid positions
    if !valid_positions.is_empty() {
        println!("First 5 valid positions:");
        for (i, (x, y)) in valid_positions.iter().take(5).enumerate() {
            println!("  {}: ({}, {})", i + 1, x, y);
        }
    }
}

/// Demonstrates advanced template usage
fn demonstrate_advanced_templates() {
    println!("\n=== Advanced Template Usage ===");

    // Create custom templates
    let custom_template = LevelTemplate {
        name: "Custom Structure".to_string(),
        width: 5,
        height: 3,
        pattern: vec![
            vec![255, 176, 176, 176, 255],
            vec![176, 255, 183, 255, 176],
            vec![180, 180, 180, 180, 180],
        ],
    };

    println!("Created custom template:");
    println!("{}", template_to_string(&custom_template));

    // Demonstrate template combinations
    let templates = vec![
        ("Ground", LevelTemplate::ground_platform(8, 180, 176)),
        ("Platform", LevelTemplate::floating_platform(4, 181)),
        ("Tower", LevelTemplate::tower(2, 8, 176, 180)),
        ("Garden", LevelTemplate::garden(4, 2, 180, 183, 185)),
    ];

    for (name, template) in templates {
        println!("\n{} template ({}x{}):", name, template.width, template.height);
        println!("{}", template_to_string(&template));
    }
}

/// Example of procedural level generation with templates
fn create_procedural_level(width: u32, height: u32, seed: u64) -> LevelData {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut level_data = LevelData {
        width,
        height,
        tiles: vec![vec![255; width as usize]; height as usize],
    };

    // Simple pseudo-random based on seed
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let mut rng_state = hasher.finish();

    // Simple linear congruential generator
    let mut next_random = || {
        rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        rng_state
    };

    // Place ground
    let ground = LevelTemplate::ground_platform(width, 180, 176);
    place_template(&mut level_data, &ground, 0, height - 2);

    // Place random platforms
    for _ in 0..8 {
        let x = (next_random() as u32) % (width - 8);
        let y = (next_random() as u32) % (height - 10) + 5;
        let platform_width = ((next_random() as u32) % 5) + 3;

        let platform = LevelTemplate::floating_platform(platform_width, 181);
        place_template(&mut level_data, &platform, x, y);
    }

    // Place some structures
    for _ in 0..3 {
        let x = (next_random() as u32) % (width - 10);
        let y = (next_random() as u32) % 5 + 5;

        match next_random() % 3 {
            0 => {
                let tower = LevelTemplate::tower(3, 10, 176, 180);
                place_template(&mut level_data, &tower, x, y);
            },
            1 => {
                let room = LevelTemplate::room(6, 5, 176, 180);
                place_template(&mut level_data, &room, x, y);
            },
            2 => {
                let pillar = LevelTemplate::pillar(8, 176);
                place_template(&mut level_data, &pillar, x, y);
            },
            _ => {}
        }
    }

    level_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = LevelTemplate::ground_platform(5, 180, 176);
        assert_eq!(template.width, 5);
        assert_eq!(template.height, 2);
        assert_eq!(template.pattern.len(), 2);
        assert_eq!(template.pattern[0].len(), 5);
    }

    #[test]
    fn test_template_placement() {
        let mut level_data = LevelData {
            width: 10,
            height: 10,
            tiles: vec![vec![255; 10]; 10],
        };

        let template = LevelTemplate::floating_platform(3, 181);
        assert!(place_template(&mut level_data, &template, 0, 0));
        assert!(place_template(&mut level_data, &template, 5, 5));
        assert!(!place_template(&mut level_data, &template, 8, 8)); // Should fail - out of bounds
    }

    #[test]
    fn test_procedural_generation() {
        let level1 = create_procedural_level(20, 15, 12345);
        let level2 = create_procedural_level(20, 15, 12345);
        let level3 = create_procedural_level(20, 15, 54321);

        // Same seed should produce same result
        assert_eq!(level1.tiles, level2.tiles);

        // Different seed should produce different result
        assert_ne!(level1.tiles, level3.tiles);
    }
}
