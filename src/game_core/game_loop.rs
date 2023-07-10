use crate::game_util::{
    components::LocalPlayer,
    resources::{ClientTick, Dots},
};
use bevy::prelude::*;

use super::player::Player;

pub fn game_loop(
    mut query: Query<(&mut Transform, &mut Player), With<LocalPlayer>>,
    mut dots: ResMut<Dots>,
    mut client_tick: ResMut<ClientTick>,
) {
    client_tick.tick += 1;
    //info!("tick: {}", client_tick.tick);
    for (mut t, mut player) in query.iter_mut() {
        player.client_side_prediction(&mut t);
        if client_tick.tick % 100 == 0 {
            info!(
                "player pos: {:?}, tick {:?}",
                t.translation.x, client_tick.tick
            );
        }

        for i in (0..dots.pos.len()).rev() {
            let dot = &dots.pos[i];
            if (dot.x - t.translation.x).abs() < 1.0 && (dot.y - t.translation.y).abs() < 1.0 {
                player.score += 1;
                dots.pos.remove(i);
            }
        }
    }
}
