use bevy::{prelude::*, render::camera::ScalingMode, utils::hashbrown::HashMap};
use uuid::Uuid;

use crate::{
    game_util::{
        components::{LocalPlayer, Player},
        resources::{Dots, NetworkStuff, PlayerInit},
    },
    network::messages::{NetworkMessage, PlayerInput},
};

pub fn handle_server(
    mut incoming: ResMut<NetworkStuff>,
    mut local_player: ResMut<PlayerInit>,
    mut query: Query<(Entity, &mut Player, &mut Transform)>,
    mut commands: Commands,
    mut dots: ResMut<Dots>,
) {
    if let Some(ref mut receive_rx) = incoming.read {
        let player_ids = query
            .iter_mut()
            .map(|(entity, player, _)| (player.id, entity))
            .collect::<HashMap<Uuid, Entity>>();

        while let Ok(Some(message)) = receive_rx.try_next() {
            match serde_json::from_str::<NetworkMessage>(&message) {
                Ok(NetworkMessage::GameUpdate(mut game_update)) => {
                    for (entity, mut player, mut t) in query.iter_mut() {
                        if let Some(player_info) = game_update.players.get_mut(&player.id) {
                            if player.client_tick < game_update.game_tick {
                                player.client_tick = game_update.game_tick;
                            }

                            player.server_tick = game_update.game_tick;

                            t.translation.x = player_info.pos.x;
                            player.reconcile_server(&mut t);
                        } else {
                            commands.entity(entity).despawn();
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
                                        server_tick: game_update.game_tick,
                                        target: player_info.pos,
                                        score: 0,
                                        pending_inputs: vec![
                                            (PlayerInput::new(
                                                player_info.pos,
                                                *player_key,
                                                game_update.game_tick,
                                            )),
                                        ],
                                        client_tick: game_update.game_tick,
                                    });
                            }
                        }
                    }

                    dots.server_tick = game_update.game_tick;
                    if dots.client_tick == 0 {
                        dots.client_tick = game_update.game_tick;
                    }
                    dots.rng_seed = Some(game_update.rng_seed);
                }
                Ok(NetworkMessage::NewInput(new_input)) => {
                    for (_, mut player, _) in query.iter_mut() {
                        if new_input.id == player.id {
                            player.target = new_input.target;
                            player.pending_inputs.push(PlayerInput::new(
                                new_input.target,
                                new_input.id,
                                new_input.tick,
                            ));
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

                    local_player.id = Some(new_game.id);
                }
                Err(_) => {}
            }
        }
    }
}
