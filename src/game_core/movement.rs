use crate::{
    game_core::dots::{PLAYER_SPEED, WORLD_BOUNDS},
    game_util::{
        components::Player,
        resources::{Dots, TickManager},
    },
};
use bevy::prelude::*;
pub fn move_players(
    mut query: Query<(&mut Transform, &mut Player)>,
    mut dots: ResMut<Dots>,
    mut ticks: ResMut<TickManager>,
) {
    ticks.client_tick += 1;
    for (mut t, mut player) in query.iter_mut() {
        apply_input(&player, &mut t);

        for i in (0..dots.pos.len()).rev() {
            let dot = &dots.pos[i];
            if (dot.x - t.translation.x).abs() < 1.0 && (dot.y - t.translation.y).abs() < 1.0 {
                player.score += 1;
                info!(
                    "Player got a dot!  pos {:?}, score {:?}",
                    dot.x, player.score
                );
                dots.pos.remove(i);
            }
        }
    }
}

pub fn apply_input(player: &Player, t: &mut Transform) {
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
}
