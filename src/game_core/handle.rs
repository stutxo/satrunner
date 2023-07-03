use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    utils::{hashbrown::HashMap, Instant},
};
use uuid::Uuid;

use crate::{
    game_util::{
        components::{LocalPlayer, Player},
        resources::{Dots, PlayerInit, Server},
    },
    network::messages::NetworkMessage,
};

pub fn handle_server(
    mut server: ResMut<Server>,
    mut local_player: ResMut<PlayerInit>,
    mut query: Query<(Entity, &mut Player, &mut Transform)>,
    mut commands: Commands,
    mut dots: ResMut<Dots>,
) {
    if let Some(ref mut receive_rx) = server.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            match serde_json::from_str::<NetworkMessage>(&message) {
                Ok(NetworkMessage::GameUpdate(mut game_update)) => {
                    dots.game_tick = game_update.game_tick;
                    dots.rng_seed = Some(game_update.rng_seed);

                    for (_, mut player, _) in query.iter_mut() {
                        if let Some(player_info) = game_update.players.get_mut(&player.id) {
                            player.server_pos = player_info.pos.x;
                            player.server_index = player_info.index;
                        }
                    }

                    let player_ids = query
                        .iter_mut()
                        .map(|(entity, player, _)| (player.id, entity))
                        .collect::<HashMap<Uuid, Entity>>();

                    for (player_id, entity) in &player_ids {
                        if !game_update.players.contains_key(player_id) {
                            commands.entity(*entity).despawn();
                        }
                    }

                    for player_key in game_update.players.keys() {
                        if !player_ids.contains_key(player_key)
                            && Some(*player_key) != local_player.id
                        {
                            if let Some(player_info) = game_update.players.get(player_key) {
                                commands
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(0.5, 1.0)),
                                            color: Color::GRAY,
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            player_info.pos.x,
                                            -50.,
                                            0.0,
                                        )),
                                        ..Default::default()
                                    })
                                    .insert(Player {
                                        id: *player_key,
                                        server_pos: player_info.pos.x,
                                        server_index: player_info.index,
                                        index: player_info.index,
                                        last_input_time: Instant::now(),
                                        target: player_info.pos,
                                    });
                            }
                        }
                    }
                }
                Ok(NetworkMessage::NewInput(new_input)) => {
                    for (_, mut player, _) in query.iter_mut() {
                        if new_input.id == player.id {
                            player.target = new_input.target;

                            player.index += 1;
                            player.last_input_time = Instant::now();
                        }
                    }
                }
                Ok(NetworkMessage::NewGame(new_game)) => {
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
                            server_pos: 0.0,
                            server_index: 0,
                            index: 0,
                            last_input_time: Instant::now(),
                            target: Vec2::ZERO,
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

                    local_player.id = Some(new_game.id);
                }
                Err(_) => {}
            }
        }
    }
}
