use crate::game_util::{components::Player, resources::Dots};
use bevy::prelude::*;

pub fn game_loop(mut query: Query<(&mut Transform, &mut Player)>, mut dots: ResMut<Dots>) {
    for (mut t, mut player) in query.iter_mut() {
        player.client_tick += 1;

        player.apply_input(&mut t);

        for i in (0..dots.pos.len()).rev() {
            let dot = &dots.pos[i];
            if (dot.x - t.translation.x).abs() < 1.0 && (dot.y - t.translation.y).abs() < 1.0 {
                player.score += 1;
                dots.pos.remove(i);
            }
        }
    }
}
