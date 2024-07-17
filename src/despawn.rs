use bevy::prelude::*;

use crate::{gamestate::Pause, schedule::InGameSet, spaceship::Spaceship};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_far_away_entities
                .in_set(InGameSet::DespawnEntities)
                .run_if(|pause: Res<Pause>| !pause.is_paused),
        );
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<Spaceship>>,
    spaceship_query: Query<&GlobalTransform, With<Spaceship>>,
) {
    if let Ok(spaceship_transform) = spaceship_query.get_single() {
        for (entity, transform) in query.iter() {
            let distance = transform
                .translation()
                .distance(spaceship_transform.translation());

            if distance > DESPAWN_DISTANCE {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
