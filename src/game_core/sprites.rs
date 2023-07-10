use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;
use uuid::Uuid;

use crate::{
    game_util::{
        components::{LocalPlayer, Particle},
        resources::ParticlePool,
    },
    network::messages::{NewGame, PlayerInfo, PlayerInput},
};

use super::player::Player;

pub fn spawn_players(commands: &mut Commands, server_tick: u64, player_id: &Uuid, player_pos: f32) {
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
        .insert(Player {
            id: *player_id,
            server_tick,
            target: Vec2 {
                x: player_pos,
                y: -50.,
            },

            score: 0,
            pending_inputs: vec![
                (PlayerInput::new(
                    Vec2 {
                        x: player_pos,
                        y: -50.,
                    },
                    *player_id,
                    server_tick,
                )),
            ],
        });
}

pub fn spawn_local(commands: &mut Commands, new_game: &NewGame) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 1.0)),
                color: Color::LIME_GREEN,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -50., 0.0)),
            ..Default::default()
        })
        .insert(Player {
            id: new_game.id,
            server_tick: new_game.server_tick,
            target: Vec2::ZERO,
            score: 0,
            pending_inputs: Vec::new(),
        })
        .insert(LocalPlayer)
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
