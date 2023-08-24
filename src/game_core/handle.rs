use bevy::prelude::*;

use speedy::Readable;

use crate::{
    game_core::sprites::{spawn_enemies, spawn_player},
    game_util::{
        components::{Bolt, Rain},
        resources::{BoltPool, ClientTick, NetworkStuff, Objects, RainPool},
    },
    network::messages::NetworkMessage,
    GameStage,
};

use super::{
    objects::{handle_bolt_behind, handle_rain_behind},
    player::{Enemy, Player},
};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_server(
    mut incoming: ResMut<NetworkStuff>,
    mut query_player: Query<(&mut Player, &mut Transform)>,
    mut query_enemy: Query<(Entity, &mut Enemy, &mut Transform, &mut Visibility), Without<Player>>,
    mut commands: Commands,
    mut client_tick: ResMut<ClientTick>,
    mut objects: ResMut<Objects>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameStage>>,
    mut rain_pool: ResMut<RainPool>,
    mut rain: Query<(&Rain, &mut Visibility, &mut Transform), (Without<Player>, Without<Enemy>)>,
    mut bolt_pool: ResMut<BoltPool>,
    mut bolt: Query<
        (&Bolt, &mut Visibility, &mut Transform),
        (Without<Player>, Without<Enemy>, Without<Rain>),
    >,
) {
    if let Some(ref mut receive_rx) = incoming.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            info!("Got message {:?}", message);
            match NetworkMessage::read_from_buffer(&message) {
                Ok(NetworkMessage::GameUpdate(game_update)) => {
                    for (mut player, mut t) in query_player.iter_mut() {
                        if game_update.id == player.id {
                            player.server_reconciliation(
                                &mut t,
                                &client_tick,
                                game_update.pos,
                                game_update.tick,
                            );
                        }
                    }

                    for (_, mut enemy, mut t, _) in query_enemy.iter_mut() {
                        if game_update.id == enemy.id && !enemy.dead {
                            enemy.target.x = game_update.input[0];
                            enemy.target.y = game_update.input[1];
                            enemy.enemy_reconciliation(
                                &mut t,
                                &client_tick,
                                game_update.pos,
                                game_update.tick,
                            );
                        }
                    }
                }
                Ok(NetworkMessage::PlayerInput(input)) => {
                    for (_, mut enemy, _, _) in query_enemy.iter_mut() {
                        if input.id == enemy.id {
                            enemy.pending_inputs.push_back(input.clone());
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
                    for (_entity, mut enemy, _t, _) in query_enemy.iter_mut() {
                        if score.id == enemy.id {
                            enemy.score = score.score;
                        }
                    }
                }
                Ok(NetworkMessage::NewGame(new_game)) => {
                    info!("new game: {:?}", new_game);
                    client_tick.tick = Some(new_game.server_tick + 2);
                    objects.rng_seed = Some(new_game.rng_seed);
                    objects.high_scores = new_game.high_scores;

                    // for (id, player_pos) in &new_game.player_positions {
                    //     if id == &new_game.id {
                    spawn_player(&mut commands, &new_game.id, &asset_server, &mut next_state);
                    //     } else if player_pos.alive {
                    //         spawn_enemies(
                    //             &mut commands,
                    //             id,
                    //             player_pos.pos,
                    //             Some(player_pos.target),
                    //             player_pos.score,
                    //             player_pos.name.clone(),
                    //             &asset_server,
                    //         );
                    //     }
                    // }

                    // info!("players: {:?}", new_game.player_positions);
                }
                Ok(NetworkMessage::PlayerConnected(player)) => {
                    //info!("player connected: {:?}", player_id);
                    let mut enemy_spawned = Vec::new();
                    for (_entity, mut enemy, _t, mut visibility) in query_enemy.iter_mut() {
                        if player.id == enemy.id {
                            *visibility = Visibility::Visible;
                            enemy.dead = false;
                            enemy_spawned.push(enemy.id);
                        }
                    }
                    if !enemy_spawned.contains(&player.id) {
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
                }
                Ok(NetworkMessage::PlayerDisconnected(player_id)) => {
                    //info!("player disconnected: {:?}", player_id);
                    for (entity, enemy, _t, _) in query_enemy.iter_mut() {
                        if player_id == enemy.id {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
                Ok(NetworkMessage::DamagePlayer(damage)) => {
                    if !damage.win {
                        if let Some(index) = objects
                            .rain_pos
                            .iter()
                            .position(|object| object.tick == damage.tick.unwrap())
                        {
                            objects.rain_pos.remove(index);
                        }
                    } else {
                        objects.high_scores = damage.high_scores.unwrap();
                    }
                    for (mut player, mut t) in query_player.iter_mut() {
                        if damage.id == player.id {
                            t.translation.x = damage.pos[0];
                            t.translation.y = damage.pos[1];
                            player.death_time = Some(damage.secs_alive);
                            player.target = t.translation.truncate();
                            next_state.set(GameStage::GameOver);
                        }
                    }
                    for (_entity, mut enemy, mut t, mut visibility) in query_enemy.iter_mut() {
                        if damage.id == enemy.id {
                            enemy.dead = true;
                            t.translation.x = damage.pos[0];
                            t.translation.y = damage.pos[1];
                            enemy.target = t.translation.truncate();
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
                Ok(NetworkMessage::SyncClient(sync_client)) => {
                    for (mut player, mut t) in query_player.iter_mut() {
                        //if we are ahead of the server, then pause the game for how many ticks we are ahead.
                        if sync_client.tick_adjustment > 0
                            && client_tick.tick.unwrap() > sync_client.server_tick
                        {
                            client_tick.pause = sync_client.tick_adjustment - 2;

                            // if we are behind the server, then apply the new adjustment iteration. we know its a new iter if the number is higher than the one we have saved.
                        } else if sync_client.tick_adjustment < 0
                            && client_tick.tick.unwrap() < sync_client.server_tick
                        {
                            let mut ticks_behind = sync_client.tick_adjustment - 2;
                            // player.adjust_iter = sync_client.adjustment_iteration;

                            while ticks_behind < 0 {
                                handle_rain_behind(
                                    &mut objects,
                                    &mut rain_pool,
                                    &mut rain,
                                    &client_tick,
                                );
                                handle_bolt_behind(
                                    &mut objects,
                                    &mut bolt_pool,
                                    &mut bolt,
                                    &client_tick,
                                );
                                player.apply_input(&mut t, &client_tick);
                                ticks_behind += 1;

                                if let Some(tick) = &mut client_tick.tick {
                                    *tick += 1;
                                }
                            }
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
