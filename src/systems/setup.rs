//! Setup systems for initializing the game world

use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use crate::components::{
    AnimationCollection, AnimationConfig, AnimationHandles, AnimationState, FacingDirection,
    PlayerVelocity,
};
use crate::constants::*;

/// Sets up the graphics system (camera)
pub fn setup_graphics(mut commands: Commands) {
    // Create camera using Camera2d with MainCamera marker
    commands.spawn((Camera2d, crate::components::MainCamera));
}

/// Sets up the physics world, ground, and player
pub fn setup_physics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create the ground
    commands
        .spawn(Name::new("Ground"))
        .insert(Collider::cuboid(GROUND_WIDTH, GROUND_THICKNESS))
        .insert(Transform::from_xyz(0.0, GROUND_HEIGHT, 0.0))
        .insert(GlobalTransform::default());

    // Load animation assets
    let idle_texture_handle: Handle<Image> = asset_server.load("character/IDLE.png");
    let run_texture_handle: Handle<Image> = asset_server.load("character/RUN.png");
    let idle_layout_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_SIZE),
        IDLE_FRAMES,
        1,
        None,
        None,
    ));
    let run_layout_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_SIZE),
        RUN_FRAMES,
        1,
        None,
        None,
    ));

    let animation_collection = AnimationCollection {
        idle: AnimationConfig::new(0, (IDLE_FRAMES - 1) as usize, IDLE_ANIMATION_FPS),
        run: AnimationConfig::new(0, (RUN_FRAMES - 1) as usize, RUN_ANIMATION_FPS),
    };
    let animation_handles = AnimationHandles {
        idle_texture: idle_texture_handle.clone(),
        idle_layout: idle_layout_handle.clone(),
        run_texture: run_texture_handle,
        run_layout: run_layout_handle,
    };

    // Create the player
    commands.spawn((
        Name::new("Player"),
        // Physics components
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
        },
        Collider::capsule(Vec2::new(0.0, -10.0), Vec2::new(0.0, 10.0), 5.0),
        KinematicCharacterControllerOutput::default(),
        // Visual components with custom anchor for proper positioning
        //
        // Anchor options available:
        // - Anchor::Center (default): Sprite centered on transform position
        // - Anchor::BottomCenter: Bottom edge centered (good for characters on ground)
        // - Anchor::TopCenter: Top edge centered
        // - Anchor::CenterLeft: Left edge centered
        // - Anchor::CenterRight: Right edge centered
        // - Anchor::BottomLeft: Bottom-left corner
        // - Anchor::BottomRight: Bottom-right corner
        // - Anchor::TopLeft: Top-left corner
        // - Anchor::TopRight: Top-right corner
        // - Anchor::Custom(Vec2): Custom offset from center (-0.5 to 0.5)
        //   Example: Anchor::Custom(Vec2::new(0.0, -0.3)) for slightly below center
        Sprite {
            image: idle_texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: idle_layout_handle,
                index: animation_collection.idle.first_sprite_index,
            }),
            anchor: Anchor::Custom(Vec2::new(0.0, -0.2)), // Feet aligned with ground
            ..default()
        },
        Transform::from_xyz(PLAYER_SPAWN_X, PLAYER_SPAWN_Y, 0.0),
        // Game logic components
        PlayerVelocity::default(),
        AnimationState::default(),
        FacingDirection::default(),
        animation_collection,
        animation_handles,
    ));
}
