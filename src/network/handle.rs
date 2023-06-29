use bevy::prelude::ResMut;

use crate::game_util::resources::{DotPos, EnemiesPos, LocalPlayerPos, Server};

pub fn handle_server(
    mut server: ResMut<Server>,
    mut local_player: ResMut<LocalPlayerPos>,
    mut enemies: ResMut<EnemiesPos>,
    mut dots: ResMut<DotPos>,
) {
    if let Some(ref mut receive_rx) = server.read {
        while let Ok(message) = receive_rx.try_next() {
            if let Some(server_msg) = message {
                enemies.0 = server_msg.other_pos;
                dots.0 = server_msg.dots;
                local_player.0 = server_msg.local_pos;
            }
        }
    }
}
