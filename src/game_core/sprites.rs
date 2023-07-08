use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;
use uuid::Uuid;

use crate::{
    game_util::{
        components::{LocalPlayer, Particle, Player},
        resources::ParticlePool,
    },
    network::messages::{NewGame, PlayerInfo, PlayerInput},
};

pub fn spawn_players(
    commands: &mut Commands,
    server_tick: u64,
    player_id: &Uuid,
    player_info: &PlayerInfo,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 1.0)),
                color: Color::GRAY,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(player_info.pos.x, -50., 0.0)),
            ..Default::default()
        })
        .insert(Player {
            id: *player_id,
            server_tick,
            target: player_info.pos,
            score: 0,
            pending_inputs: vec![(PlayerInput::new(player_info.pos, *player_id, server_tick))],
            client_tick: server_tick,
        });
}

pub fn spawn_local(commands: &mut Commands, new_game: &NewGame) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 1.0)),
                color: Color::ORANGE_RED,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -50., 0.0)),
            ..Default::default()
        })
        .insert(Player {
            id: new_game.id,
            server_tick: 0,
            target: Vec2::ZERO,
            score: 0,
            pending_inputs: Vec::new(),
            client_tick: 0,
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
