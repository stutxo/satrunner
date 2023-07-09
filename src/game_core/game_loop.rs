use crate::game_util::resources::Dots;
use bevy::prelude::*;

use super::player::Player;

pub fn game_loop(mut query: Query<(&Transform, &mut Player)>, mut dots: ResMut<Dots>) {
    for (t, mut player) in query.iter_mut() {
        for i in (0..dots.pos.len()).rev() {
            let dot = &dots.pos[i];
            if (dot.x - t.translation.x).abs() < 1.0 && (dot.y - t.translation.y).abs() < 1.0 {
                player.score += 1;
                dots.pos.remove(i);
            }
        }
    }
}
