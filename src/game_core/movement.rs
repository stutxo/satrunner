use crate::{
    game_core::dots::{PLAYER_SPEED, WORLD_BOUNDS},
    game_util::components::Player,
};
use bevy::{prelude::*, utils::Instant};

pub fn move_players(mut query: Query<(&mut Transform, &Player)>) {
    for (mut t, player) in query.iter_mut() {
        let direction = player.target - Vec2::new(t.translation.x, t.translation.y);
        let distance_to_target = direction.length();

        if distance_to_target > 0.0 {
            let movement = if distance_to_target <= PLAYER_SPEED {
                direction
            } else {
                direction.normalize() * PLAYER_SPEED
            };

            let new_position = Vec2::new(t.translation.x, t.translation.y) + movement;

            if new_position.x.abs() <= WORLD_BOUNDS && new_position.y.abs() <= WORLD_BOUNDS {
                t.translation += Vec2::new(movement.x, 0.0).extend(0.0);
            }
        }

        if Instant::now()
            .duration_since(player.last_input_time)
            .as_millis()
            > 200
            && player.server_index != player.index
        {
            info!("ROLL BACK: {:?} -> {:?}", player.index, player.server_index,);
            t.translation.x = player.server_pos;
        }
    }
}
