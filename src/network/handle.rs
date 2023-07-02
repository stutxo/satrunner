use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    utils::{hashbrown::HashMap, Instant},
};
use uuid::Uuid;

use crate::game_util::{
    components::{LocalPlayer, Player, Target},
    resources::{DotPos, PlayerInit, Server},
};

use super::messages::NetworkMessage;

pub fn handle_server(
    mut server: ResMut<Server>,
    mut local_player: ResMut<PlayerInit>,
    mut query: Query<(Entity, &mut Player, &mut Target, &mut Transform)>,
    mut commands: Commands,
) {
    if let Some(ref mut receive_rx) = server.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            match serde_json::from_str::<NetworkMessage>(&message) {
                Ok(NetworkMessage::GameUpdate(mut game_update)) => {
                    for (_, mut player, _, _) in query.iter_mut() {
                        if let Some(player_info) = game_update.players.get_mut(&player.id) {
                            player.server_pos = player_info.pos.x;
                            player.server_index = player_info.index;
                        }
                    }

                    let player_ids = query
                        .iter_mut()
                        .map(|(entity, player, _, _)| (player.id, entity))
                        .collect::<HashMap<Uuid, Entity>>();

                    // despawn entities not in the game_update
                    for (player_id, entity) in &player_ids {
                        if !game_update.players.contains_key(player_id) {
                            commands.entity(*entity).despawn();
                            info!("Player {:?} does not exist in the game_update", player_id)
                        }
                    }

                    // create entities present in the game_update but not in player_ids
                    for player_key in game_update.players.keys() {
                        if !player_ids.contains_key(player_key)
                            && Some(*player_key) != local_player.id
                        {
                            info!(
                                "Key {:?} does not exist in the player_ids vector",
                                player_key
                            );
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
                                        moving: true,
                                        id: *player_key,
                                        server_pos: player_info.pos.x,
                                        server_index: player_info.index,
                                    })
                                    .insert(Target {
                                        x: player_info.pos.x,
                                        y: player_info.pos.y,
                                        index: player_info.index,
                                        last_input_time: Instant::now(),
                                    });
                            }
                        }
                    }
                }
                Ok(NetworkMessage::NewInput(new_input)) => {
                    for (_, mut player, mut target, _) in query.iter_mut() {
                        if new_input.id == player.id {
                            target.x = new_input.target.x;
                            target.y = new_input.target.y;
                            target.index += 1;
                            target.last_input_time = Instant::now();
                            player.moving = true;
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
                            moving: false,
                            id: new_game.id,
                            server_pos: 0.0,
                            server_index: 0,
                        })
                        .insert(LocalPlayer)
                        .insert(Target::new())
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
