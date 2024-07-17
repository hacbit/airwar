use bevy::{prelude::*, utils::HashMap};

use crate::{
    asteroids::Asteroid,
    gamestate::Pause,
    schedule::InGameSet,
    spaceship::Spaceship,
    status::Status,
};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: Vec::new(),
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection
                .before(InGameSet::CollisionDetection)
                .run_if(|pause: Res<Pause>| !pause.is_paused),
        )
        .add_systems(
            Update,
            handle_collisions
                .in_set(InGameSet::DespawnEntities)
                .run_if(|pause: Res<Pause>| !pause.is_paused),
        );
    }
}

fn collision_detection(
    mut query: Query<(
        Entity,
        &GlobalTransform,
        &mut Collider,
    )>,
) {
    let mut colliding_entities = HashMap::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_collisions(
    mut commands: Commands,
    query: Query<
        (Entity, &Collider, &mut Status),
        (
            With<Asteroid>,
            Without<Spaceship>,
        ),
    >,
    mut spaceship_query: Query<&mut Status, With<Spaceship>>,
) {
    /* let mut to_reduce_health = Vec::new();
    let mut to_despawn = Vec::new();
 */
    for (entity, collider, _) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            if let Ok(spaceship_status) = spaceship_query.get_single() {
                // If the spaceship is dead, don't handle collisions
                if spaceship_status.health == 0 {
                    return;
                }

                commands.entity(entity).despawn_recursive();
                spaceship_query.single_mut().score += 1;
            }
            if let Ok(mut status) = spaceship_query.get_mut(collided_entity) {
                status.health -= 1;
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    /* for entity in to_reduce_health {
        if let Ok((_, _, mut status)) = query.get_mut(entity) {
            status.health -= 1;
        } else if let Ok(mut spaceship_status) = spaceship_query.get_mut(entity) {
            spaceship_status.health -= 1;
            info!("Your are hit! Now health: {:?}", spaceship_status.health);
        }
    }

    for entity in to_despawn {
        if spaceship_query.get(entity).is_err() {
            spaceship_query.single_mut().score += query.get(entity).unwrap().2.score;
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, _, status) in query.iter() {
        if status.health == 0 {
            spaceship_query.single_mut().score += status.score;
            commands.entity(entity).despawn_recursive();
        }
    } */
}
