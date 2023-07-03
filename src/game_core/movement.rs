use crate::{
    game_core::setup::{PLAYER_SPEED, WORLD_BOUNDS},
    game_util::components::{Particle, Player},
    game_util::resources::{DotPos, ParticlePool},
};
use bevy::{prelude::*, utils::Instant};

pub fn move_dot(
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&Particle, &mut Visibility, &mut Transform)>,
    dots: Res<DotPos>,
) {
    for dot in dots.0.iter() {
        if let Some(pool) = particle_pool.0.pop_front() {
            match particles.get_mut(pool) {
                Ok((_particle, mut visibility, mut transform)) => {
                    transform.translation = *dot;
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    error!("Error: {:?}", err);
                }
            }
            particle_pool.0.push_back(pool);
        }
    }
}

pub fn move_local(mut query: Query<(&mut Transform, &Player)>) {
    for (mut t, player) in query.iter_mut() {
        let direction = player.target - Vec2::new(t.translation.x, t.translation.y);
        let distance_to_target = direction.length();

        if distance_to_target > 0.0 {
            let movement = if distance_to_target <= PLAYER_SPEED {
                direction
            } else {
                direction.normalize() * PLAYER_SPEED
            };

            let new_position = Vec2::new(t.translation.x, t.translation.y) + movement;

            if new_position.x.abs() <= WORLD_BOUNDS && new_position.y.abs() <= WORLD_BOUNDS {
                t.translation += Vec2::new(movement.x, 0.0).extend(0.0);
            }
        }

        if Instant::now()
            .duration_since(player.last_input_time)
            .as_millis()
            > 100
            && player.server_index != player.index
        {
            info!("ROLL BACK: {:?} -> {:?}", player.index, player.server_index,);
            t.translation.x = player.server_pos;
        }
    }
}
