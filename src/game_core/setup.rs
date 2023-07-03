use bevy::prelude::*;
use rand::Rng;

use crate::{
    game_util::components::Particle,
    game_util::resources::{DotPos, ParticlePool},
};

pub const WORLD_BOUNDS: f32 = 300.0;
pub const PLAYER_SPEED: f32 = 1.0;
const FALL_SPEED: f32 = 0.5;

pub fn setup(mut commands: Commands, mut particle_pool: ResMut<ParticlePool>) {
    for _ in 0..1000 {
        let color = if rand::random() {
            Color::rgb(0.75, 0.75, 0.75) // silver
        } else {
            Color::rgb(1.0, 0.84, 0.0) // gold
        };

        let particle = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    color,
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

pub fn internal_server(mut dots: ResMut<DotPos>) {
    let mut rng = rand::thread_rng();
    let num_balls: i32 = rng.gen_range(1..10);

    for _ in 0..num_balls {
        let x_position: f32 = rng.gen_range(-WORLD_BOUNDS..WORLD_BOUNDS);
        let y_position: f32 = 25.;

        let dot_start = Vec3::new(x_position, y_position, 0.1);

        dots.0.push(dot_start);
    }
}

pub fn out_server(mut dots: ResMut<DotPos>) {
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
}
