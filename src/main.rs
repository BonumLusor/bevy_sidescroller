use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const GRAVITY: f32 = -981.0;
const JUMP_FORCE: f32 = 450.0;

#[derive(Component, Default)]
struct PlayerVelocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, move_player)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Name::new("Ground"))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0));

    /* Create the player */
    commands
        .spawn(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
        })
        .insert(Collider::ball(30.0))
        .insert(Name::new("Player"))
        .insert(Transform::from_xyz(0.0, 100.0, 0.0))
        .insert(PlayerVelocity::default())
        .insert(KinematicCharacterControllerOutput::default());
}

fn move_player(
    time: Res<Time>,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &mut PlayerVelocity,
        &KinematicCharacterControllerOutput,
    )>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut controller, mut velocity, output) in controllers.iter_mut() {
        // Reinicia a velocidade vertical se estiver no chão para evitar o acúmulo de gravidade.
        if output.grounded {
            velocity.0.y = 0.0;
        }

        // Aplica a gravidade
        velocity.0.y += GRAVITY * time.delta_secs();

        // Movimento horizontal
        let mut horizontal_movement = 0.0;
        if keyboard.pressed(KeyCode::KeyA) {
            horizontal_movement -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            horizontal_movement += 1.0;
        }
        velocity.0.x = horizontal_movement * PLAYER_SPEED;

        // Pulo
        if keyboard.just_pressed(KeyCode::KeyW) && output.grounded {
            velocity.0.y = JUMP_FORCE;
        }

        // Aplica a velocidade ao controlador
        controller.translation = Some(velocity.0 * time.delta_secs());
    }
}



