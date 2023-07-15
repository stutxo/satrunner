use bevy::{prelude::*, utils::HashSet};
use speedy::Readable;
use uuid::Uuid;

use crate::{
    game_util::{
        components::LocalPlayer,
        resources::{ClientTick, Dots, NetworkStuff},
    },
    network::messages::NetworkMessage,
};

use super::{
    player::Player,
    sprites::{spawn_local, spawn_players},
};

pub fn handle_server(
    mut incoming: ResMut<NetworkStuff>,
    mut query_local: Query<(Entity, &mut Player, &mut Transform), With<LocalPlayer>>,
    mut query_others: Query<(Entity, &mut Player, &mut Transform), Without<LocalPlayer>>,
    mut commands: Commands,
    mut client_tick: ResMut<ClientTick>,
    mut dots: ResMut<Dots>,
) {
    if let Some(ref mut receive_rx) = incoming.read {
        while let Ok(Some(message)) = receive_rx.try_next() {
            match NetworkMessage::read_from_buffer(&message) {
                Ok(NetworkMessage::GameUpdate(game_update)) => {
                    let mut existing_players: HashSet<Uuid> = HashSet::new();

                    for (_entity, mut player, mut t) in query_local.iter_mut() {
                        if game_update.id == player.id {
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
                                player.server_reconciliation(
                                    &mut t,
                                    client_tick.tick,
                                    game_update.pos,
                                    game_update.tick,
                                );
                            }
                        }
                    }
                    for (_entity, mut player, mut t) in query_others.iter_mut() {
                        existing_players.insert(player.id);

                        player.apply_input(&mut t);
                    }

                    if !existing_players.contains(&game_update.id) {
                        spawn_players(&mut commands, &game_update.id, game_update.pos);
                    }
                }
                Ok(NetworkMessage::NewGame(new_game)) => {
                    spawn_local(&mut commands, &new_game);
                    client_tick.tick = new_game.server_tick + 50;
                    dots.rng_seed = Some(new_game.rng_seed);
                }
                Err(_) => {}
            }
        }
    }
}
