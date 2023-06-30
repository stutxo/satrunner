use bevy::{prelude::*, utils::Instant};

use crate::game_util::{
    components::Enemies,
    resources::{DotPos, EnemyState, LocalPlayerPos, Server},
};

#[derive(Clone)]
pub struct EnemyPos {
    pub prev_pos: f32,
    pub current_pos: f32,
    pub last_update_time: Instant,
}

pub fn handle_server(
    mut server: ResMut<Server>,
    mut local_player: ResMut<LocalPlayerPos>,
    mut enemies_state: ResMut<EnemyState>,
    mut dots: ResMut<DotPos>,
    // Add a query to get access to enemy entities
    enemies: Query<Entity, With<Enemies>>,
) {
    if let Some(ref mut receive_rx) = server.read {
        while let Ok(message) = receive_rx.try_next() {
            if let Some(server_msg) = message {
                // Assume server_msg.other_pos now contains positions of all enemies
                for (entity, &pos) in enemies.iter().zip(server_msg.other_pos.iter()) {
                    let enemy_state = enemies_state.0.entry(entity).or_insert_with(|| EnemyPos {
                        prev_pos: pos,
                        current_pos: pos,
                        last_update_time: Instant::now(),
                    });

                    // Update previous position and current position
                    enemy_state.prev_pos = enemy_state.current_pos;
                    enemy_state.current_pos = pos;
                    enemy_state.last_update_time = Instant::now();
                }

                dots.0 = server_msg.dots;
                local_player.x = server_msg.local_pos;
                local_player.index = server_msg.index;
            }
        }
    }
}
