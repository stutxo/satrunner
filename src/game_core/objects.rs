use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::game_util::{
    components::{Bolt, Rain},
    resources::{BoltPool, ClientTick, Objects, RainPool},
};

use super::player::Player;

pub const X_BOUNDS: f32 = 1000.0;
pub const Y_BOUNDS: f32 = 500.0;
pub const FALL_SPEED: f32 = 4.0;

#[derive(Debug)]
pub struct ObjectPos {
    pub tick: u64,
    pub pos: Vec3,
}

pub fn handle_rain(
    mut objects: ResMut<Objects>,
    mut rain_pool: ResMut<RainPool>,
    mut rain: Query<(&Rain, &mut Visibility, &mut Transform), Without<Player>>,
    client_tick: Res<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = objects.rng_seed {
            let seed = rng_seed ^ client_tick.tick.unwrap();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let x_position: f32 = rng.gen_range(-X_BOUNDS..X_BOUNDS);

            let y_position: f32 = Y_BOUNDS;

            if client_tick.tick.unwrap_or(0) % 10 != 0 {
                let pos_start = Vec3::new(x_position, y_position, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.rain_pos.push(new_pos);
            }

            for object in objects.rain_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -0.5;
            }

            objects.rain_pos.retain(|object| {
                object.pos.y >= -Y_BOUNDS
                    && object.pos.y <= Y_BOUNDS
                    && object.pos.x >= -X_BOUNDS
                    && object.pos.x <= X_BOUNDS
            });

            let mut pool_iter = rain_pool.0.iter_mut();

            for object in objects.rain_pos.iter() {
                if let Some(pool) = pool_iter.next() {
                    match rain.get_mut(*pool) {
                        Ok((_particles, mut visibility, mut transform)) => {
                            transform.translation = object.pos;
                            *visibility = Visibility::Visible;
                        }
                        Err(err) => {
                            info!("Error: {:?}", err);
                        }
                    }
                }
            }

            for pool in pool_iter {
                if let Ok((_particle, mut visibility, _transform)) = rain.get_mut(*pool) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn handle_bolt(
    mut objects: ResMut<Objects>,
    mut bolt_pool: ResMut<BoltPool>,
    mut bolt: Query<(&Bolt, &mut Visibility, &mut Transform), Without<Player>>,
    client_tick: Res<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = objects.rng_seed {
            let seed = rng_seed ^ (client_tick.tick.unwrap());
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let x_position: f32 = rng.gen_range(-X_BOUNDS..X_BOUNDS);

            let y_position: f32 = Y_BOUNDS;
            if client_tick.tick.unwrap_or(0) % 10 == 0 {
                let pos_start = Vec3::new(x_position, y_position, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.bolt_pos.push(new_pos);
            }

            for object in objects.bolt_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -0.5;
            }

            objects.bolt_pos.retain(|object| {
                object.pos.y >= -Y_BOUNDS
                    && object.pos.y <= Y_BOUNDS
                    && object.pos.x >= -X_BOUNDS
                    && object.pos.x <= X_BOUNDS
            });

            let mut pool_iter = bolt_pool.0.iter_mut();

            for object in objects.bolt_pos.iter() {
                if let Some(pool) = pool_iter.next() {
                    match bolt.get_mut(*pool) {
                        Ok((_particles, mut visibility, mut transform)) => {
                            transform.translation = object.pos;
                            *visibility = Visibility::Visible;
                        }
                        Err(err) => {
                            info!("Error: {:?}", err);
                        }
                    }
                }
            }

            for pool in pool_iter {
                if let Ok((_particle, mut visibility, _transform)) = bolt.get_mut(*pool) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
