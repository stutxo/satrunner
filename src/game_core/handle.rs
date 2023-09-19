use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
    utils::HashSet,
};

use speedy::Readable;

use crate::{
    game_core::sprites::{spawn_enemies, spawn_player},
    game_util::{
        components::{Bolt, Rain},
        resources::{BoltPool, ClientTick, NetworkStuff, Objects, RainPool},
    },
    network::messages::NetworkMessage,
    GameStage, KeyboardState,
};

use super::{
    objects::{handle_bolt_behind, handle_rain_behind, ObjectPos},
    player::{Enemy, Player},
    sprites::PLAYER_SIZE,
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
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
    windows: Query<&Window>,
    mut textures: ResMut<Assets<Image>>,
    mut sprite: Query<&mut Handle<Image>, With<Player>>,
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
                            if game_update.id == enemy.id {
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

                    for player_state in player_state.clone() {
                        info!("badge: {:?}", player_state.badge_url);
                        for (mut local_player, _) in query_player.iter_mut() {
                            if local_player.id == player_state.id {
                                local_player.score = player_state.score;
                            }
                        }
                        for (_, mut enemy, _, _) in query_enemy.iter_mut() {
                            if enemy.id == player_state.id {
                                enemy.score = player_state.score;
                            }
                        }
                        if !existing_entities.contains(&player_state.id) {
                            spawn_enemies(
                                &mut commands,
                                &player_state.id,
                                Some(player_state.pos),
                                Some(player_state.target),
                                player_state.score,
                                player_state.name,
                                &asset_server,
                                player_state.time_alive,
                                player_state.badge_url,
                            );
                        }
                    }
                }
                Ok(NetworkMessage::NewGame(new_game)) => {
                    client_tick.tick = Some(new_game.server_tick);
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

                    spawn_player(
                        &mut commands,
                        &new_game.id,
                        &asset_server,
                        &mut next_state,
                        &mut keyboard_state,
                        &windows,
                    );
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
                            let mut ticks_behind = sync_client.tick_adjustment;

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
                Ok(NetworkMessage::BadgeUrl(badge)) => {
                    for (player, _) in query_player.iter_mut() {
                        if badge.id == player.id {
                            let img_result = image::load_from_memory(badge.url.as_slice());

                            match img_result {
                                Ok(dynamic_image) => {
                                    let image_data = dynamic_image.to_rgba8();
                                    let (width, height) = image_data.dimensions();
                                    let data = image_data.into_raw();

                                    let extent = Extent3d {
                                        width,
                                        height,
                                        depth_or_array_layers: 1,
                                    };

                                    let dimensions = TextureDimension::D2;

                                    let img_format = TextureFormat::bevy_default();

                                    let bevy_image =
                                        Image::new(extent, dimensions, data, img_format);

                                    let texture = textures.add(bevy_image);

                                    for mut handle in &mut sprite.iter_mut() {
                                        *handle = texture.clone();
                                    }
                                }
                                Err(e) => {
                                    info!("Failed to read the image: {:?}", e);
                                }
                            };
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
}
