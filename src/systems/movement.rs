//! Movement systems for player character

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{FacingDirection, PlayerVelocity};
use crate::constants::*;

/// Handles player movement input and physics
pub fn move_player(
    time: Res<Time>,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &mut PlayerVelocity,
        &KinematicCharacterControllerOutput,
    )>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut controller, mut velocity, output) in controllers.iter_mut() {
        if output.grounded {
            velocity.0.y = 0.0;
        }

        velocity.0.y += GRAVITY * time.delta_secs();

        let mut horizontal_movement = 0.0;
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            horizontal_movement -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            horizontal_movement += 1.0;
        }
        velocity.0.x = horizontal_movement * PLAYER_SPEED;

        if (keyboard.just_pressed(KeyCode::KeyW)
            || keyboard.just_pressed(KeyCode::Space)
            || keyboard.just_pressed(KeyCode::ArrowUp))
            && output.grounded
        {
            velocity.0.y = JUMP_FORCE;
        }

        controller.translation = Some(velocity.0 * time.delta_secs());
    }
}

/// Updates the facing direction based on player velocity for sprite flipping
/// This system runs after movement updates to ensure the character sprite
/// faces the correct direction when moving left or right
pub fn update_facing_direction(mut query: Query<(&PlayerVelocity, &mut FacingDirection)>) {
    for (velocity, mut facing_direction) in query.iter_mut() {
        // Update facing direction based on horizontal movement
        if velocity.0.x > 0.0 {
            *facing_direction = FacingDirection::Right;
        } else if velocity.0.x < 0.0 {
            *facing_direction = FacingDirection::Left;
        }
        // Note: We don't change direction when velocity is 0 to maintain
        // the last facing direction when the character stops
    }
}
