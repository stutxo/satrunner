use bevy::{prelude::*, utils::HashSet};
use uuid::Uuid;

use crate::{
    game_util::{
        components::Player,
        resources::{Dots, NetworkStuff, PlayerInit},
    },
    network::messages::{NetworkMessage, PlayerInput},
};

use super::sprites::{spawn_local, spawn_players};

pub fn handle_server(
    mut incoming: ResMut<NetworkStuff>,
    mut local_player: ResMut<PlayerInit>,
    mut query: Query<(Entity, &mut Player, &mut Transform)>,
    mut commands: Commands,
    mut dots: ResMut<Dots>,
) {
    if let Some(ref mut receive_rx) = incoming.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            match serde_json::from_str::<NetworkMessage>(&message) {
                Ok(NetworkMessage::GameUpdate(mut game_update)) => {
                    let mut existing_players: HashSet<Uuid> = HashSet::new();

                    for (entity, mut player, mut t) in query.iter_mut() {
                        existing_players.insert(player.id);

                        if let Some(player_info) = game_update.players.get_mut(&player.id) {
                            if player.client_tick < game_update.game_tick {
                                player.client_tick = game_update.game_tick;
                            }

                            player.server_tick = game_update.game_tick;

                            t.translation.x = player_info.pos.x;
                            player.server_reconciliation(&mut t);
                        } else {
                            commands.entity(entity).despawn();
                        }
                    }

                    for player_id in game_update.players.keys() {
                        if !existing_players.contains(player_id)
                            && Some(*player_id) != local_player.id
                        {
                            if let Some(player_info) = game_update.players.get(player_id) {
                                spawn_players(
                                    &mut commands,
                                    game_update.game_tick,
                                    player_id,
                                    player_info,
                                );
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
                    spawn_local(&mut commands, &new_game);

                    local_player.id = Some(new_game.id);
                }
                Err(_) => {}
            }
        }
    }
}
