use bevy::prelude::*;

use crate::{schedule::InGameSet, spaceship::Spaceship, status::Status};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_spaceship.after(InGameSet::EntityUpdates));
    }
}

fn print_spaceship(
    query: Query<(&Transform, &Status), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        for (transform, status) in query.iter() {
            info!(
                "Spaceship is at transform {:?} with status {:?}",
                transform.translation, status
            );
        }
    }
}
