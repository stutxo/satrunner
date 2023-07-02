use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    utils::{hashbrown::HashMap, Instant},
};
use uuid::Uuid;

use crate::game_util::{
    components::{LocalPlayer, Player, Target},
    resources::{DotPos, Server},
};

use super::messages::NetworkMessage;

pub fn handle_server(
    mut server: ResMut<Server>,
    mut query: Query<(Entity, &mut Player, &mut Target, &mut Transform)>,
    mut commands: Commands,
) {
    if let Some(ref mut receive_rx) = server.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            match serde_json::from_str::<NetworkMessage>(&message) {
                Ok(NetworkMessage::GameUpdate(mut game_update)) => {
                    let player_ids = query
                        .iter_mut()
                        .map(|(entity, player, _, _)| (player.id, entity))
                        .collect::<HashMap<Uuid, Entity>>();

                    let player_ids_clone = player_ids.clone();

                    for (player_id, entity) in player_ids {
                        if game_update.players.contains_key(&player_id) {
                            info!("Player {:?} exists in the game_update", player_id)
                        } else {
                            commands.entity(entity).despawn();
                            info!("Player {:?} does not exist in the game_update", player_id)
                        }
                    }

                    for player_key in game_update.players.keys() {
                        if player_ids_clone.contains_key(player_key) {
                            info!("Key {:?} exists in the player_ids vector", player_key);
                        } else {
                            info!(
                                "Key {:?} does not exist in the player_ids vector",
                                player_key
                            );
                            if let Some(player_info) = game_update.players.get(player_key) {
                                commands
                                    .spawn(SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(0.5, 1.0)),
                                            color: Color::RED,
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            player_info.pos.x,
                                            -50.,
                                            0.1,
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

                    for (_, mut player, _, _) in query.iter_mut() {
                        if let Some(player_info) = game_update.players.get_mut(&player.id) {
                            player.server_pos = player_info.pos.x;
                            player.server_index = player_info.index;
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
                                color: Color::WHITE,
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(0., -50., 0.1)),
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
                }
                Err(_) => {}
            }
        }
    }
}

// pub fn new_player(
//     mut commands: Commands,
//     mut server: ResMut<Server>,
//     mut query: Query<(&mut Target, &mut Player)>,
// ) {
//     if let Some(ref mut receive_rx) = server.input {
//         while let Ok(message) = receive_rx.try_next() {
//             if let Some(player_info) = message {
//                 // Check if a player with the same id already exists
//                 let mut player_exists = false;
//                 for (mut target, mut player) in query.iter_mut() {
//                     for (id, player_info) in player_info.input.iter() {
//                         target.x = player_info.target.x;
//                         target.y = player_info.target.y;
//                         target.index = player_info.index;
//                         target.last_input_time = Instant::now();
//                         player.moving = true;
//                         player_exists = true;
//                     }
//                 }

//                 // If player does not exist, create a new one
//                 if !player_exists {
//                     commands
//                         .spawn(SpriteBundle {
//                             sprite: Sprite {
//                                 custom_size: Some(Vec2::new(0.5, 1.0)),
//                                 color: Color::RED,
//                                 ..Default::default()
//                             },
//                             transform: Transform::from_translation(Vec3::new(
//                                 player_info.pos.x,
//                                 -50.,
//                                 0.1,
//                             )),
//                             ..Default::default()
//                         })
//                         .insert(Player {
//                             moving: true,
//                             id: player_info.player_id,
//                             server_pos: player_info.pos.x,
//                             server_index: 1,
//                         })
//                         .insert(Target {
//                             x: player_info.target.x,
//                             y: player_info.target.y,
//                             index: player_info.index,
//                             last_input_time: Instant::now(),
//                         });
//                 }
//             }
//         }
//     }
// }
