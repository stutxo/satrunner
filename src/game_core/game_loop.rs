use std::time::Duration;

use crate::game_util::{
    components::{NamePlates, NamePlatesLocal},
    resources::ClientTick,
};
use bevy::{prelude::*, utils::Instant};

use super::player::{Enemy, Player};

pub fn player_loop(
    mut query_player: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    mut query_text: Query<&mut Text, With<NamePlatesLocal>>,
    client_tick: Res<ClientTick>,
) {
    for (mut t, mut player, mut sprite) in query_player.iter_mut() {
        t.translation.z = 1.0;
        sprite.color = default();

        let duration = Instant::now() - player.spawn_time.unwrap();
        let seconds = duration.as_secs();
        let minutes = seconds / 60;

        for mut text in query_text.iter_mut() {
            text.sections[0].value = format!(
                "{:02}/21\n({:02}:{:02})",
                player.score,
                minutes % 60,
                seconds % 60,
            );
        }

        player.apply_input(&mut t, &client_tick);
    }
}

pub fn enemy_loop(
    mut query_enemy: Query<(&mut Transform, &mut Enemy)>,
    mut query_text: Query<(&mut Text, &NamePlates)>,
    client_tick: Res<ClientTick>,
) {
    for (mut t, mut enemy) in query_enemy.iter_mut() {
        enemy.spawn_time.tick(Duration::from_millis(100));
        let duration = &enemy.spawn_time;
        let seconds = duration.elapsed_secs() as u64;

        let minutes = seconds / 60;

        for (mut text, plates) in query_text.iter_mut() {
            if plates.id == enemy.id {
                text.sections[0].value = format!(
                    "{:02}/21\n({:02}:{:02})\n{}",
                    enemy.score,
                    minutes % 60,
                    seconds % 60,
                    enemy.name
                );
            }
        }

        enemy.apply_input(&mut t, &client_tick);
    }
}

pub fn tick(mut client_tick: ResMut<ClientTick>) {
    if client_tick.pause > 0 {
        client_tick.pause -= 1;
    } else if let Some(tick) = &mut client_tick.tick {
        *tick += 1;
    }
}
