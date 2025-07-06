//! Animation systems for character sprite animations

use bevy::prelude::*;

use crate::components::{
    AnimationCollection, AnimationHandles, AnimationState, FacingDirection, PlayerVelocity,
};

/// Updates animation state based on player movement
pub fn update_animation_state(
    mut query: Query<(&PlayerVelocity, &mut AnimationState), Changed<PlayerVelocity>>,
) {
    for (velocity, mut state) in query.iter_mut() {
        let new_state = if velocity.0.x.abs() > 0.0 {
            AnimationState::Run
        } else {
            AnimationState::Idle
        };

        if *state != new_state {
            *state = new_state;
        }
    }
}

/// Executes sprite animations, handles texture switching, and applies sprite flipping
pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(
        &mut Sprite,
        &mut AnimationCollection,
        &AnimationHandles,
        &AnimationState,
        &FacingDirection,
    )>,
) {
    for (mut sprite, mut collection, handles, state, facing_direction) in query.iter_mut() {
        let (target_image, target_layout) = match *state {
            AnimationState::Idle => (&handles.idle_texture, &handles.idle_layout),
            AnimationState::Run => (&handles.run_texture, &handles.run_layout),
        };

        // Check if we need to change the texture atlas
        let needs_texture_change = if let Some(atlas) = &sprite.texture_atlas {
            atlas.layout != *target_layout
        } else {
            false
        };

        // Handle texture change first (before borrowing atlas mutably)
        if needs_texture_change {
            sprite.image = target_image.clone();
        }

        // Then handle atlas changes
        if let Some(atlas) = &mut sprite.texture_atlas {
            if needs_texture_change {
                atlas.layout = target_layout.clone();
                atlas.index = match *state {
                    AnimationState::Idle => collection.idle.first_sprite_index,
                    AnimationState::Run => collection.run.first_sprite_index,
                };
            }

            // Handle timer and animation logic
            match *state {
                AnimationState::Idle => {
                    collection.idle.frame_timer.tick(time.delta());
                    if collection.idle.frame_timer.just_finished() {
                        atlas.index = if atlas.index >= collection.idle.last_sprite_index {
                            collection.idle.first_sprite_index
                        } else {
                            atlas.index + 1
                        };
                    }
                }
                AnimationState::Run => {
                    collection.run.frame_timer.tick(time.delta());
                    if collection.run.frame_timer.just_finished() {
                        atlas.index = if atlas.index >= collection.run.last_sprite_index {
                            collection.run.first_sprite_index
                        } else {
                            atlas.index + 1
                        };
                    }
                }
            }
        }

        // Apply sprite flipping based on facing direction
        // flip_x = true makes the sprite face left, false makes it face right
        // This is done outside the animation loop for better performance
        sprite.flip_x = *facing_direction == FacingDirection::Left;
    }
}
