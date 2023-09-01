use bevy::{prelude::*, utils::HashSet};

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
    objects::{handle_bolt_behind, handle_rain_behind, ObjectPos},
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
            match NetworkMessage::read_from_buffer(&message) {
                Ok(NetworkMessage::GameUpdate(game_update)) => {
                    for game_update in &game_update {
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
                }
                Ok(NetworkMessage::GameState(player_state)) => {
                    let current_player_ids: HashSet<_> =
                        player_state.iter().map(|p| p.id).collect();
                    let mut existing_entities = Vec::new();

                    for (player, _) in query_player.iter_mut() {
                        existing_entities.push(player.id);
                    }
                    for (entity, enemy, _, _) in query_enemy.iter_mut() {
                        existing_entities.push(enemy.id);
                        for entity_id in &existing_entities {
                            if !current_player_ids.contains(entity_id) && entity_id == &enemy.id {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                    }

                    for player in player_state.clone() {
                        for (mut local_player, _) in query_player.iter_mut() {
                            if local_player.id == player.id {
                                local_player.score = player.score;
                            }
                        }
                        for (_, mut enemy, _, _) in query_enemy.iter_mut() {
                            if enemy.id == player.id {
                                enemy.score = player.score;
                            }
                        }
                        if !existing_entities.contains(&player.id) {
                            spawn_enemies(
                                &mut commands,
                                &player.id,
                                Some(player.pos),
                                Some(player.target),
                                player.score,
                                player.name,
                                &asset_server,
                                player.time_alive,
                            );
                        }
                    }
                }
                Ok(NetworkMessage::NewGame(new_game)) => {
                    client_tick.tick = Some(new_game.server_tick + 10);
                    objects.rng_seed = Some(new_game.rng_seed);
                    objects.high_scores = new_game.high_scores;

                    objects.rain_pos = new_game
                        .objects
                        .rain_pos
                        .iter()
                        .map(|&(tick, [x, y])| ObjectPos {
                            tick,
                            pos: Vec3 { x, y, z: 0.0 },
                        })
                        .collect();

                    objects.bolt_pos = new_game
                        .objects
                        .bolt_pos
                        .iter()
                        .map(|&(tick, [x, y])| ObjectPos {
                            tick,
                            pos: Vec3 { x, y, z: 0.0 },
                        })
                        .collect();

                    spawn_player(&mut commands, &new_game.id, &asset_server, &mut next_state);
                }
                Ok(NetworkMessage::DamagePlayer(damage)) => {
                    if let Some(index) = objects
                        .rain_pos
                        .iter()
                        .position(|object| object.tick == damage.tick.unwrap())
                    {
                        objects.rain_pos.remove(index);
                    }

                    if let Some(high_scores) = damage.high_scores {
                        objects.high_scores = high_scores;
                    }

                    for (mut player, mut t) in query_player.iter_mut() {
                        if damage.id == player.id {
                            t.translation = Vec3::ZERO;
                            player.death_time = Some(damage.secs_alive);
                            player.score = damage.score;
                            player.target = t.translation.truncate();
                            next_state.set(GameStage::GameOver);
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
                Ok(NetworkMessage::SyncClient(sync_client)) => {
                    for (mut player, mut t) in query_player.iter_mut() {
                        if sync_client.tick_adjustment > 0
                            && client_tick.tick.unwrap() > sync_client.server_tick
                        {
                            client_tick.pause = sync_client.tick_adjustment;
                        } else if sync_client.tick_adjustment < 0
                            && client_tick.tick.unwrap() < sync_client.server_tick
                        {
                            let mut ticks_behind = sync_client.tick_adjustment - 3;

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
