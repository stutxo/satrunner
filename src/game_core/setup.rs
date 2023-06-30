use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;

use crate::{
    game_util::components::{Enemies, Particle, Player, Target},
    game_util::resources::{EnemiesPool, ParticlePool},
};

pub const WORLD_BOUNDS: f32 = 300.0;
pub const PLAYER_SPEED: f32 = 1.0;

pub fn setup(
    mut commands: Commands,
    mut particle_pool: ResMut<ParticlePool>,
    mut enemies_pool: ResMut<EnemiesPool>,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 1.0)),
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -50., 0.1)),
            ..Default::default()
        })
        .insert(Player { moving: false })
        .insert(Target::default())
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 25., 0.)),
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(100.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    for _ in 0..1000 {
        let particle = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    color: Color::rgb(
                        rand::thread_rng().gen_range(0.0..1.0),
                        rand::thread_rng().gen_range(0.0..1.0),
                        rand::thread_rng().gen_range(0.0..1.0),
                    ),
                    ..default()
                },

                ..Default::default()
            })
            .insert(Particle())
            .insert(Visibility::Hidden)
            .id();
        particle_pool.0.push_back(particle);
    }

    for _ in 0..100 {
        let enemies = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 1.0)),
                    color: Color::RED,
                    ..default()
                },
                ..Default::default()
            })
            .insert(Enemies())
            .insert(Visibility::Hidden)
            .id();
        enemies_pool.0.push_back(enemies);
    }
}
