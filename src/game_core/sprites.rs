use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;
use uuid::Uuid;
// use uuid::Uuid;

use crate::{
    game_util::{components::Particle, resources::ParticlePool},
    network::messages::NewGame,
};

use super::player::{Enemy, Player};

pub fn spawn_player(commands: &mut Commands, new_game: &NewGame) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 1.0)),
                color: Color::LIME_GREEN,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -50., 0.1)),
            ..Default::default()
        })
        .insert(Player {
            id: new_game.id,
            target: Vec2::ZERO,
            score: 0,
            pending_inputs: Vec::new(),
            adjust_iter: 0,
        })
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
}

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

pub fn spawn_enemies(
    commands: &mut Commands,
    player_id: &Uuid,
    player_pos: Option<f32>,
    target: Option<[f32; 2]>,
    score: usize,
) {
    let target = target.unwrap_or([0.0, 0.0]);
    let player_pos = player_pos.unwrap_or(0.0);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 1.0)),
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(player_pos, -50., 0.0)),
            ..Default::default()
        })
        .insert(Enemy {
            id: *player_id,
            target: Vec2 {
                x: target[0],
                y: target[1],
            },
            score,
        });
}
