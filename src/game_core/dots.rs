use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    game_util::components::Particle,
    game_util::resources::{ClientTick, Dots, ParticlePool},
};

use super::player::Player;

pub const WORLD_BOUNDS: f32 = 300.0;

pub const FALL_SPEED: f32 = 0.5;

pub fn handle_dots(
    mut dots: ResMut<Dots>,
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&Particle, &mut Visibility, &mut Transform), Without<Player>>,
    client_tick: Res<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = dots.rng_seed {
            let seed = rng_seed ^ client_tick.tick.unwrap();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            for _ in 1..2 {
                let x_position: f32 = rng.gen_range(-WORLD_BOUNDS..WORLD_BOUNDS);

                let y_position: f32 = 25.;

                let dot_start = Vec3::new(x_position, y_position, 0.0);
                dots.pos.push(dot_start);
            }

            for dot in dots.pos.iter_mut() {
                dot.y += FALL_SPEED * -1.0;
            }

            dots.pos.retain(|dot| {
                dot.y >= -WORLD_BOUNDS
                    && dot.y <= WORLD_BOUNDS
                    && dot.x >= -WORLD_BOUNDS
                    && dot.x <= WORLD_BOUNDS
            });

            for dot in dots.pos.iter() {
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
    }
}
