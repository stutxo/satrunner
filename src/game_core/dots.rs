use bevy::prelude::*;
use rand::Rng;

use crate::{
    game_util::components::Particle,
    game_util::resources::{DotPos, ParticlePool},
};

pub const WORLD_BOUNDS: f32 = 300.0;
pub const PLAYER_SPEED: f32 = 1.0;
const FALL_SPEED: f32 = 0.5;

pub fn pool_dots(mut commands: Commands, mut particle_pool: ResMut<ParticlePool>) {
    for _ in 0..1000 {
        let particle = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    color: Color::rgb(
                        rand::thread_rng().gen_range(0.0..1.0),
                        rand::thread_rng().gen_range(0.0..2.0),
                        rand::thread_rng().gen_range(0.0..3.0),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Particle)
            .insert(Visibility::Hidden)
            .id();
        particle_pool.0.push_back(particle);
    }
}

pub fn handle_dots(
    mut dots: ResMut<DotPos>,
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&Particle, &mut Visibility, &mut Transform)>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let x_position: f32 = rng.gen_range(-WORLD_BOUNDS..WORLD_BOUNDS);
        let y_position: f32 = 25.;
        let dot_start = Vec3::new(x_position, y_position, 0.1);
        dots.0.push(dot_start);
    }

    for dot in dots.0.iter_mut() {
        dot.x += FALL_SPEED * 0.0;
        dot.y += FALL_SPEED * -1.0;
    }

    dots.0.retain(|dot| {
        dot.y >= -WORLD_BOUNDS
            && dot.y <= WORLD_BOUNDS
            && dot.x >= -WORLD_BOUNDS
            && dot.x <= WORLD_BOUNDS
    });

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
