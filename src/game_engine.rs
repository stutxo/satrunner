use crate::{
    components::{Enemies, Particle},
    resources::{DotPos, EnemiesPool, EnemiesPos, ParticlePool},
};
use bevy::prelude::*;

pub fn move_dot(
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
    dots: ResMut<DotPos>,
) {
    for dot in dots.0.iter() {
        if let Some(pool) = particle_pool.0.pop_front() {
            match particles.get_mut(pool) {
                Ok((_particle, mut visibility, mut transform)) => {
                    transform.translation = *dot;
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
            particle_pool.0.push_back(pool);
        }
    }
}

pub fn move_enemies(
    mut enemies_pool: ResMut<EnemiesPool>,
    mut enemies: Query<(&mut Enemies, &mut Visibility, &mut Transform)>,
    enemies_pos: ResMut<EnemiesPos>,
) {
    let mut pool_iter = enemies_pool.0.iter_mut();

    for enemy in enemies_pos.0.iter() {
        if let Some(pool) = pool_iter.next() {
            match enemies.get_mut(*pool) {
                Ok((_enemies, mut visibility, mut transform)) => {
                    transform.translation = Vec3::new(*enemy, -50., 0.1);
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
        }
    }

    for pool in pool_iter {
        if let Ok((_particle, mut visibility, _transform)) = enemies.get_mut(*pool) {
            *visibility = Visibility::Hidden;
        }
    }
}
