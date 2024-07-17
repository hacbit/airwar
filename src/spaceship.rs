use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    gamestate::Pause,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::InGameSet,
    status::Status,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const SPACESHIP_CAMERA_TRANSLATION: Vec3 = Vec3::new(0., 10., -30.);
const SPACESHIP_STARTING_HEALTH: u32 = 3;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;
const MISSILE_HEALTH: u32 = 1;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship).add_systems(
            Update,
            (
                spaceship_movement_control,
                spaceship_weapon_control,
                spaceship_shield_control,
            )
                .chain()
                .in_set(InGameSet::UserInput)
                .run_if(|pause: Res<Pause>| !pause.is_paused),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands
        .spawn((
            MovingObjectBundle {
                velocity: Velocity::new(Vec3::ZERO),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(SPACESHIP_RADIUS),
                status: Status::new(SPACESHIP_STARTING_HEALTH, 0),
                model: SceneBundle {
                    scene: scene_assets.spaceship.clone(),
                    transform: Transform::from_translation(STARTING_TRANSLATION),
                    ..default()
                },
            },
            Spaceship,
        ))
        .with_children(|parent| {
            // camera
            parent.spawn(Camera3dBundle {
                transform: Transform::from_translation(SPACESHIP_CAMERA_TRANSLATION)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });
}

fn spaceship_movement_control(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // move forward or backward
    if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    }

    // rotate left or right
    if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    // roll left or right
    if keyboard_input.pressed(KeyCode::Q) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::E) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // update transform
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_control(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                status: Status::new(MISSILE_HEALTH, 0),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation - transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}

fn spaceship_shield_control(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}
