// uncomment this to hide the console window
#![windows_subsystem = "windows"]

mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod gamestate;
mod movement;
mod schedule;
mod spaceship;
mod status;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
#[allow(unused_imports)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
// use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use gamestate::{GameOverPlugin, PausePlugin};
use schedule::SchedulePlugin;
use status::StatusPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba_u8(27, 21, 45, 235)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.80,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Airwar".to_string(),
                position: WindowPosition::new(IVec2::new(100, 100)),
                ..default()
            }),
            ..default()
        }))
        //.add_plugins(WorldInspectorPlugin::new())
        // User configured plugins
        // load the assets like the spaceship and asteroids, etc.
        .add_plugins(AssetLoaderPlugin)
        // handle the movement of the moving objects
        .add_plugins(MovementPlugin)
        // spawn the spaceship(player) and a camera to follow it
        .add_plugins(SpaceshipPlugin)
        // spawn the asteroids
        .add_plugins(AsteroidsPlugin)
        // handle the collision detection
        .add_plugins(CollisionDetectionPlugin)
        // despawn the entities when collision happens
        .add_plugins(DespawnPlugin)
        // add a child camera to the spaceship
        // deprecated
        //.add_plugins(CameraPlugin)
        // show the info, press the key 'Enter' to print the spaceship info
        .add_plugins(DebugPlugin)
        // in game set
        .add_plugins(SchedulePlugin)
        // show the status, including score, health, etc.
        // pressed the key 'V' to show the status
        .add_plugins(StatusPlugin)
        // press the 'P' key to pause the game
        .add_plugins((PausePlugin, GameOverPlugin))
        .run();
}
