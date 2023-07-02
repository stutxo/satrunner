use crate::{
    game_core::setup::{PLAYER_SPEED, WORLD_BOUNDS},
    game_util::components::{Particle, Player, Target},
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

//need to test to see if this is a better way to hide/show particles. requires 4000 sprites to spawn instead of 1000.

// pub fn move_dot(
//     particle_pool: Res<ParticlePool>,
//     mut particles: Query<(&Particle, &mut Visibility, &mut Transform)>,
//     dots: Res<DotPos>,
// ) {
//     let mut pool_iter = particle_pool.0.iter();

//     for dot in dots.0.iter() {
//         if let Some(pool) = pool_iter.next() {
//             match particles.get_mut(*pool) {
//                 Ok((_particle, mut visibility, mut transform)) => {
//                     transform.translation = *dot;
//                     *visibility = Visibility::Visible;
//                 }
//                 Err(err) => {
//                     info!("Error: {:?}", err);
//                 }
//             }
//         }
//     }

//     // Make the remaining particles invisible
//     for pool in pool_iter {
//         if let Ok((_particle, mut visibility, _transform)) = particles.get_mut(*pool) {
//             *visibility = Visibility::Hidden;
//         }
//     }
// }

pub fn move_local(mut query: Query<(&mut Transform, &Target, &mut Player)>) {
    for (mut t, tg, mut p) in query.iter_mut() {
        if p.moving {
            let current_position = Vec2::new(t.translation.x, t.translation.y);
            let direction = Vec2::new(tg.x, tg.y) - current_position;
            let distance_to_target = direction.length();

            if distance_to_target > 0.0 {
                let normalized_direction = direction / distance_to_target;
                let movement = normalized_direction * PLAYER_SPEED;

                let new_position = current_position + movement;

                if new_position.x.abs() <= WORLD_BOUNDS && new_position.y.abs() <= WORLD_BOUNDS {
                    if movement.length() < distance_to_target {
                        t.translation += Vec3::new(movement.x, 0.0, 0.0);
                    } else {
                        t.translation = Vec3::new(tg.x, -50.0, 0.0);

                        p.moving = false;
                    }
                } else {
                    p.moving = false;
                }
            } else {
                p.moving = false;
            }
        }
        if Instant::now()
            .duration_since(tg.last_input_time)
            .as_millis()
            > 100
            && p.server_index != tg.index
        {
            info!("ROLL BACK: {:?} -> {:?}", tg.index, p.server_index,);
            t.translation.x = p.server_pos;
        }
    }
}
