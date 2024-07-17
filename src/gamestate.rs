use bevy::prelude::*;

use crate::{schedule::InGameSet, spaceship::Spaceship, status::Status};

#[derive(Resource, Debug)]
pub struct Pause {
    pub is_paused: bool,
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Pause { is_paused: false })
            .add_systems(Update, pause_or_resume);
    }
}

fn pause_or_resume(mut pause: ResMut<Pause>, input: Res<Input<KeyCode>>, game_over: Res<GameOver>) {
    // Pause the game if the spaceship is dead
    if game_over.is_game_over {
        pause.is_paused = true;
    } else if input.just_pressed(KeyCode::P) {
        pause.is_paused ^= true;
    }
}

#[derive(Resource, Debug)]
pub struct GameOver {
    pub is_game_over: bool,
    pub was_game_over: bool,
    pub display_text: Option<Entity>,
}

#[derive(Component, Debug)]
pub struct GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameOver {
            is_game_over: false,
            was_game_over: false,
            display_text: None,
        })
        .add_systems(Update, game_over.in_set(InGameSet::GameOver))
        .add_systems(
            Update,
            display_when_spaceship_dead
                .after(InGameSet::GameOver)
                .run_if(|game_over: Res<GameOver>| game_over.is_game_over),
        );
    }
}

fn game_over(query: Query<&Status, With<Spaceship>>, mut game_over: ResMut<GameOver>) {
    if query.get_single().is_ok() && query.get_single().unwrap().health == 0 {
        game_over.is_game_over = true;
    }
}

fn display_when_spaceship_dead(
    mut commands: Commands,
    query: Query<&Status, With<Spaceship>>,
    mut game_over: ResMut<GameOver>,
) {
    if game_over.was_game_over {
        return;
    }

    game_over.was_game_over = true;

    let text_entity = commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "You died!".to_string(),
                        style: TextStyle {
                            font: Handle::default(),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: format!("Score: {:?}", query.get_single().unwrap().score),
                        style: TextStyle {
                            font: Handle::default(),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(42.0),
                left: Val::Percent(32.0),
                ..default()
            },
            ..default()
        },
        GameState,
    ));

    game_over.display_text = Some(text_entity.id());
}
