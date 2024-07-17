use bevy::prelude::*;

use crate::{gamestate::GameState, spaceship::Spaceship};

#[derive(Component, Debug, Default)]
pub struct Status {
    pub health: u32,
    pub score: u32,
}

impl Status {
    pub fn new(health: u32, score: u32) -> Self {
        Self { health, score }
    }
}

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship_status)
            .add_systems(Update, display_spaceship_status);
    }
}

fn spawn_spaceship_status(mut commands: Commands, query: Query<&Status, With<Spaceship>>) {
    if let Ok(status) = query.get_single() {
        commands.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: format!("Health: {:?} ", status.health),
                        style: TextStyle {
                            font: Handle::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: format!("Score: {:?}", status.score),
                        style: TextStyle {
                            font: Handle::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        });
    }
}

fn display_spaceship_status(
    mut query: Query<(&mut Text, &mut Visibility), Without<GameState>>,
    query_spaceship: Query<&Status, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(spaceship_status) = query_spaceship.get_single() {
        if let Ok((mut text, mut visibility)) = query.get_single_mut() {
            text.sections[0].value = format!("Health: {:?} ", spaceship_status.health);
            text.sections[1].value = format!("Score: {:?}", spaceship_status.score);
            if keyboard_input.just_pressed(KeyCode::V) {
                if *visibility == Visibility::Hidden {
                    *visibility = Visibility::Inherited;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
