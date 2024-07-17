use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::gamestate::Pause;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;
use crate::status::Status;
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_HEALTH_RANGE: Range<u32> = 1..3;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ASTEROID_ROTATION_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroids, rotate_asteriod)
                .in_set(InGameSet::EntityUpdates)
                .run_if(|pause: Res<Pause>| !pause.is_paused),
        );
    }
}

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    let health = rng.gen_range(SPAWN_HEALTH_RANGE);

    let score = 2 * health - 1;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            collider: Collider::new(RADIUS),
            status: Status::new(health, score),
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}

fn rotate_asteriod(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ASTEROID_ROTATION_SPEED * time.delta_seconds());
    }
}
