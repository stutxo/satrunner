use std::time::Duration;

use bevy::{prelude::*, utils::Instant};
use speedy::Readable;

use crate::{
    game_core::sprites::spawn_enemies,
    game_util::resources::{ClientTick, Dots, NetworkStuff, PingTimer},
    network::messages::NetworkMessage,
};

use super::{
    player::{Enemy, Player},
    sprites::spawn_player,
};

pub fn handle_server(
    mut incoming: ResMut<NetworkStuff>,
    mut query_player: Query<(&mut Player, &mut Transform)>,
    mut query_enemy: Query<(Entity, &mut Enemy, &mut Transform), Without<Player>>,
    mut commands: Commands,
    mut client_tick: ResMut<ClientTick>,
    mut dots: ResMut<Dots>,
    mut ping: ResMut<PingTimer>,
) {
    if let Some(ref mut receive_rx) = incoming.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            ping.ping_timer = Instant::now();
            match NetworkMessage::read_from_buffer(&message) {
                Ok(NetworkMessage::GameUpdate(game_update)) => {
                    //info!("got game update: {:?}", game_update);
                    for (mut player, mut t) in query_player.iter_mut() {
                        if game_update.id == player.id {
                            //if we are ahead of the server, then pause the game for how many ticks we are ahead.
                            if game_update.tick_adjustment > 0 {
                                client_tick.pause = game_update.tick_adjustment - 8;
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
                    info!("new game: {:?}", new_game);
                    client_tick.tick = Some(new_game.server_tick + 8);
                    dots.rng_seed = Some(new_game.rng_seed);
                    for (id, player_pos) in &new_game.player_positions {
                        if id == &new_game.id {
                            spawn_player(&mut commands, &new_game);
                        } else {
                            spawn_enemies(
                                &mut commands,
                                id,
                                player_pos.pos,
                                Some(player_pos.target),
                                player_pos.score,
                                player_pos.name.clone(),
                            );
                        }
                    }

                    // info!("players: {:?}", new_game.player_positions);
                }
                Ok(NetworkMessage::PlayerConnected(player)) => {
                    ping.ping_timer = Instant::now();
                    //info!("player connected: {:?}", player_id);
                    spawn_enemies(&mut commands, &player.id, Some(0.0), None, 0, player.name);
                }
                Ok(NetworkMessage::PlayerDisconnected(player_id)) => {
                    ping.ping_timer = Instant::now();
                    //info!("player disconnected: {:?}", player_id);
                    for (entity, enemy, _t) in query_enemy.iter_mut() {
                        if player_id == enemy.id {
                            commands.entity(entity).despawn();
                        }
                    }
                }
                Ok(NetworkMessage::Ping) => {}
                Err(_) => {}
            }
        }
    }
}

pub fn disconnect_check_system(ping_timer: ResMut<PingTimer>) {
    if ping_timer.ping_timer.elapsed() > Duration::from_secs(10) {
        if let Some(disconnected_tx) = &ping_timer.disconnected_tx {
            disconnected_tx.clone().try_send(()).unwrap();
            info!("No ping received for 6 seconds, sending disconnect signal.");
        }
    }
}
