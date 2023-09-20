use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::game_util::{
    components::{Badge, Bolt, Rain},
    resources::{BadgePool, BoltPool, ClientTick, Objects, RainPool},
};

use super::player::{Enemy, Player};

pub const X_BOUNDS: f32 = 1000.0;
pub const Y_BOUNDS: f32 = 500.0;
pub const FALL_SPEED: f32 = 3.0;

#[derive(Debug)]
pub struct ObjectPos {
    pub tick: u64,
    pub pos: Vec3,
}

pub fn handle_rain(
    mut objects: ResMut<Objects>,
    mut rain_pool: ResMut<RainPool>,
    mut rain: Query<(&Rain, &mut Visibility, &mut Transform), Without<Player>>,
    client_tick: ResMut<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = objects.rng_seed {
            let seed = rng_seed ^ client_tick.tick.unwrap();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let x_position: f32 = rng.gen_range(-X_BOUNDS..X_BOUNDS);

            if client_tick.tick.unwrap_or(0) % 5 != 0 {
                let pos_start = Vec3::new(x_position, Y_BOUNDS, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.rain_pos.push(new_pos);
            }

            for object in objects.rain_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -1.;
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

            if client_tick.tick.unwrap_or(0) % 5 == 0 {
                let pos_start = Vec3::new(x_position, Y_BOUNDS, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.bolt_pos.push(new_pos);
            }

            for object in objects.bolt_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -1.;
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

#[allow(clippy::type_complexity)]
pub fn handle_rain_behind(
    objects: &mut ResMut<Objects>,
    rain_pool: &mut ResMut<RainPool>,
    rain: &mut Query<
        (&Rain, &mut Visibility, &mut Transform),
        (Without<Player>, Without<Enemy>, Without<Bolt>),
    >,
    client_tick: &ResMut<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = objects.rng_seed {
            let seed = rng_seed ^ client_tick.tick.unwrap();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let x_position: f32 = rng.gen_range(-X_BOUNDS..X_BOUNDS);

            if client_tick.tick.unwrap_or(0) % 5 != 0 {
                let pos_start = Vec3::new(x_position, Y_BOUNDS, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.rain_pos.push(new_pos);
            }

            for object in objects.rain_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -1.0;
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

#[allow(clippy::type_complexity)]
pub fn handle_bolt_behind(
    objects: &mut ResMut<Objects>,
    bolt_pool: &mut ResMut<BoltPool>,
    bolt: &mut Query<
        (&Bolt, &mut Visibility, &mut Transform),
        (Without<Player>, Without<Enemy>, Without<Rain>),
    >,
    client_tick: &ResMut<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = objects.rng_seed {
            let seed = rng_seed ^ (client_tick.tick.unwrap());
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let x_position: f32 = rng.gen_range(-X_BOUNDS..X_BOUNDS);

            if client_tick.tick.unwrap_or(0) % 5 == 0 {
                let pos_start = Vec3::new(x_position, Y_BOUNDS, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.bolt_pos.push(new_pos);
            }

            for object in objects.bolt_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -1.0;
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

pub fn handle_badge(
    mut objects: ResMut<Objects>,
    mut badge_pool: ResMut<BadgePool>,
    mut badge: Query<(&Badge, &mut Visibility, &mut Transform), Without<Player>>,
    client_tick: ResMut<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = objects.rng_seed {
            let seed = rng_seed ^ client_tick.tick.unwrap();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let x_position: f32 = rng.gen_range(-X_BOUNDS..X_BOUNDS);

            if client_tick.tick.unwrap_or(0) % 50 == 0 {
                let pos_start = Vec3::new(x_position, Y_BOUNDS, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.badge_pos.push(new_pos);
            }

            for object in objects.badge_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -1.;
            }

            objects.badge_pos.retain(|object| {
                object.pos.y >= -Y_BOUNDS
                    && object.pos.y <= Y_BOUNDS
                    && object.pos.x >= -X_BOUNDS
                    && object.pos.x <= X_BOUNDS
            });

            let mut pool_iter = badge_pool.0.iter_mut();

            for object in objects.badge_pos.iter() {
                if let Some(pool) = pool_iter.next() {
                    match badge.get_mut(*pool) {
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
                if let Ok((_particle, mut visibility, _transform)) = badge.get_mut(*pool) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_badge_behind(
    objects: &mut ResMut<Objects>,
    badge_pool: &mut ResMut<BadgePool>,
    badge: &mut Query<
        (&Badge, &mut Visibility, &mut Transform),
        (
            Without<Player>,
            Without<Enemy>,
            Without<Rain>,
            Without<Bolt>,
        ),
    >,
    client_tick: &ResMut<ClientTick>,
) {
    if client_tick.pause == 0 {
        if let Some(rng_seed) = objects.rng_seed {
            let seed = rng_seed ^ (client_tick.tick.unwrap());
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let x_position: f32 = rng.gen_range(-X_BOUNDS..X_BOUNDS);

            if client_tick.tick.unwrap_or(0) % 5 == 0 {
                let pos_start = Vec3::new(x_position, Y_BOUNDS, 0.0);
                let new_pos = ObjectPos {
                    tick: client_tick.tick.unwrap(),
                    pos: pos_start,
                };
                objects.badge_pos.push(new_pos);
            }

            for object in objects.badge_pos.iter_mut() {
                object.pos.y += FALL_SPEED * -1.0;
            }

            objects.badge_pos.retain(|object| {
                object.pos.y >= -Y_BOUNDS
                    && object.pos.y <= Y_BOUNDS
                    && object.pos.x >= -X_BOUNDS
                    && object.pos.x <= X_BOUNDS
            });

            let mut pool_iter = badge_pool.0.iter_mut();

            for object in objects.badge_pos.iter() {
                if let Some(pool) = pool_iter.next() {
                    match badge.get_mut(*pool) {
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
                if let Ok((_particle, mut visibility, _transform)) = badge.get_mut(*pool) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
