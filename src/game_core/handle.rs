use bevy::{prelude::*, utils::HashSet};
use uuid::Uuid;

use crate::{
    game_util::resources::{ClientTick, Dots, NetworkStuff, PlayerInit},
    network::messages::{NetworkMessage, PlayerInput},
};

use super::{player::Player, sprites::spawn_local};

pub fn handle_server(
    mut incoming: ResMut<NetworkStuff>,
    mut local_player: ResMut<PlayerInit>,
    mut query: Query<(Entity, &mut Player, &mut Transform)>,
    mut commands: Commands,
    mut client_tick: ResMut<ClientTick>,
    //mut dots: ResMut<Dots>,
) {
    if let Some(ref mut receive_rx) = incoming.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            match serde_json::from_str::<NetworkMessage>(&message) {
                Ok(NetworkMessage::GameUpdate(game_update)) => {
                    let mut existing_players: HashSet<Uuid> = HashSet::new();

                    for (_entity, mut player, mut t) in query.iter_mut() {
                        existing_players.insert(player.id);

                        player.server_tick = game_update.tick;
                        //if we are ahead of the server, then pause the game for how many ticks we are ahead.
                        if game_update.tick_adjustment > 0.0 {
                            player.pause = game_update.tick_adjustment - 4.0;
                            player.adjust_iter = game_update.adjustment_iteration;
                        // if we are behind the server, then apply the new adjustment iteration. we know its a new iter if the number is higher than the one we have saved.
                        } else if game_update.tick_adjustment < -0.0
                            && player.adjust_iter < game_update.adjustment_iteration
                        {
                            let mut ticks_behind = game_update.tick_adjustment - 4.0;
                            player.adjust_iter = game_update.adjustment_iteration;
                            while ticks_behind < -0.0 {
                                player.apply_input(&mut t);
                                ticks_behind += 1.0;
                                info!(
                                    "adjusting: {}, player iter {:?}",
                                    ticks_behind, player.adjust_iter
                                );
                                client_tick.tick += 1;
                            }
                        } else {
                            player.server_reconciliation(&mut t, client_tick.tick, game_update.pos);
                        }
                    }
                    // client_tick.tick = game_update.tick + 5;

                    // if !existing_players.contains(&game_update.id)
                    //     && Some(game_update.id) != local_player.id
                    // {
                    //     spawn_players(
                    //         &mut commands,
                    //         game_update.tick,
                    //         &game_update.id,
                    //         game_update.pos,
                    //     );
                    // }

                    // dots.server_tick = game_update.game_tick;
                    // if dots.client_tick == 0 {
                    //     dots.client_tick = game_update.game_tick;
                    // }
                    // dots.rng_seed = Some(game_update.rng_seed);
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
                    client_tick.tick = new_game.server_tick + 4;
                    local_player.id = Some(new_game.id);
                }
                Err(_) => {}
            }
        }
    }
}
