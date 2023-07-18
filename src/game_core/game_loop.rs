use crate::game_util::resources::{ClientTick, Dots};
use bevy::prelude::*;

use super::player::{Enemy, Player};

pub fn player_loop(
    mut query_player: Query<(&mut Transform, &mut Player, &mut Visibility)>,
    mut dots: ResMut<Dots>,
    client_tick: Res<ClientTick>,
) {
    for (mut t, mut player, mut visibility) in query_player.iter_mut() {
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        }

        //always set local player above other players
        t.translation.z = 0.1;

        player.apply_input(&mut t, &client_tick);

        for i in (0..dots.pos.len()).rev() {
            let dot = &dots.pos[i];
            if (dot.x - t.translation.x).abs() < 1.0 && (dot.y - t.translation.y).abs() < 1.0 {
                dots.pos.remove(i);
                // info!(
                //     "PLAYER {:?} HIT A DOT!!!: {}, SCORE {:?}",
                //     player.id, t.translation.x, player.score
                // );
            }
        }
    }
}

pub fn enemy_loop(
    mut query_enemy: Query<(&mut Transform, &mut Enemy)>,
    mut dots: ResMut<Dots>,
    client_tick: Res<ClientTick>,
) {
    for (mut t, mut enemy) in query_enemy.iter_mut() {
        enemy.apply_input(&mut t, &client_tick);

        for i in (0..dots.pos.len()).rev() {
            let dot = &dots.pos[i];
            if (dot.x - t.translation.x).abs() < 1.0 && (dot.y - t.translation.y).abs() < 1.0 {
                dots.pos.remove(i);
                // info!(
                //     "enemy {:?} HIT A DOT!!!: {}, SCORE {:?}",
                //     enemy.id, t.translation.x, enemy.score
                // );
            }
        }
    }
}

pub fn tick(mut client_tick: ResMut<ClientTick>) {
    if client_tick.pause > 0 {
        client_tick.pause -= 1;
    } else {
        client_tick.tick += 1;
    }
}
