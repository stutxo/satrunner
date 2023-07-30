use std::time::Duration;

use bevy::{prelude::*, utils::Instant, window::PrimaryWindow};

use speedy::Readable;

use crate::{
    game_core::sprites::{spawn_enemies, spawn_player},
    game_util::resources::{ClientTick, NetworkStuff, Objects, PingTimer},
    network::messages::NetworkMessage,
    GameStage,
};

use super::player::{Enemy, Player};

pub fn handle_server(
    mut incoming: ResMut<NetworkStuff>,
    mut query_player: Query<(&mut Player, &mut Transform)>,
    mut query_enemy: Query<(Entity, &mut Enemy, &mut Transform), Without<Player>>,
    mut commands: Commands,
    mut client_tick: ResMut<ClientTick>,
    mut objects: ResMut<Objects>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameStage>>,
) {
    if let Some(ref mut receive_rx) = incoming.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            match NetworkMessage::read_from_buffer(&message) {
                Ok(NetworkMessage::GameUpdate(game_update)) => {
                    //info!("got game update: {:?}", game_update);
                    for (mut player, mut t) in query_player.iter_mut() {
                        if game_update.id == player.id {
                            //if we are ahead of the server, then pause the game for how many ticks we are ahead.
                            if game_update.tick_adjustment > 0 {
                                client_tick.pause = game_update.tick_adjustment - 6;
                                player.adjust_iter = game_update.adjustment_iteration;
                            // if we are behind the server, then apply the new adjustment iteration. we know its a new iter if the number is higher than the one we have saved.
                            } else if game_update.tick_adjustment < 0
                                && player.adjust_iter < game_update.adjustment_iteration
                            {
                                let mut ticks_behind = game_update.tick_adjustment - 4;
                                player.adjust_iter = game_update.adjustment_iteration;

                                while ticks_behind < 0 {
                                    player.apply_input(&mut t, &client_tick);
                                    ticks_behind += 1;
                                    // info!(
                                    //     "adjusting: {}, player iter {:?}",
                                    //     ticks_behind, player.adjust_iter
                                    // );
                                    if let Some(tick) = &mut client_tick.tick {
                                        *tick += 1;
                                    }
                                }
                            } else {
                                player.server_reconciliation(
                                    &mut t,
                                    &client_tick,
                                    game_update.pos,
                                    game_update.tick,
                                );
                            }
                        }
                    }
                    for (_entity, mut enemy, mut t) in query_enemy.iter_mut() {
                        if game_update.id == enemy.id {
                            enemy.target.x = game_update.input[0];
                            enemy.target.y = game_update.input[1];
                            t.translation.x = game_update.pos;
                        }
                    }
                }
                Ok(NetworkMessage::ScoreUpdate(score)) => {
                    if let Some(index) = objects
                        .bolt_pos
                        .iter()
                        .position(|object| object.tick == score.tick)
                    {
                        objects.bolt_pos.remove(index);
                    }

                    for (mut player, _t) in query_player.iter_mut() {
                        if score.id == player.id {
                            player.score = score.score;
                        }
                    }
                    for (_entity, mut enemy, _t) in query_enemy.iter_mut() {
                        if score.id == enemy.id {
                            enemy.score = score.score;
                        }
                    }
                }
                Ok(NetworkMessage::NewGame(new_game)) => {
                    client_tick.tick = Some(new_game.server_tick + 6);
                    objects.rng_seed = Some(new_game.rng_seed);
                    // info!("new game: {:?}", new_game);

                    for (id, player_pos) in &new_game.player_positions {
                        if id == &new_game.id {
                            spawn_player(
                                &mut commands,
                                &new_game.id,
                                &asset_server,
                                &mut next_state,
                            );
                        } else if player_pos.alive {
                            spawn_enemies(
                                &mut commands,
                                id,
                                player_pos.pos,
                                Some(player_pos.target),
                                player_pos.score,
                                player_pos.name.clone(),
                                &asset_server,
                            );
                        }
                    }

                    // info!("players: {:?}", new_game.player_positions);
                }
                Ok(NetworkMessage::PlayerConnected(player)) => {
                    //info!("player connected: {:?}", player_id);
                    spawn_enemies(
                        &mut commands,
                        &player.id,
                        None,
                        None,
                        0,
                        Some(player.name),
                        &asset_server,
                    );
                }
                Ok(NetworkMessage::PlayerDisconnected(player_id)) => {
                    //info!("player disconnected: {:?}", player_id);
                    for (entity, enemy, _t) in query_enemy.iter_mut() {
                        if player_id == enemy.id {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
                Ok(NetworkMessage::DamagePlayer(damage)) => {
                    if let Some(index) = objects
                        .rain_pos
                        .iter()
                        .position(|object| object.tick == damage.tick)
                    {
                        objects.rain_pos.remove(index);
                    }
                    for (player, _t) in query_player.iter_mut() {
                        if damage.id == player.id {
                            next_state.set(GameStage::GameOver);
                        }
                    }
                }
                Ok(NetworkMessage::Ping) => {}
                Err(_) => {}
            }
        }
    }
}

// pub fn disconnect_check_system(ping_timer: ResMut<PingTimer>) {
//     if ping_timer.ping_timer.elapsed() > Duration::from_secs(10) {
//         if let Some(disconnected_tx) = &ping_timer.disconnected_tx {
//             disconnected_tx.clone().try_send(()).unwrap();
//             info!("No ping received for 10 seconds, sending disconnect signal.");
//         }
//     }
// }
