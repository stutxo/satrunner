use crate::game_util::{
    components::{NamePlates, NamePlatesLocal},
    resources::{ClientTick, Objects},
};
use bevy::prelude::*;

use super::player::{Enemy, Player};

pub fn player_loop(
    mut query_player: Query<(&mut Transform, &mut Player, &mut Visibility)>,
    mut query_text: Query<&mut Text, With<NamePlatesLocal>>,
    mut objects: ResMut<Objects>,
    client_tick: Res<ClientTick>,
) {
    for (mut t, mut player, mut visibility) in query_player.iter_mut() {
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        }

        for mut text in query_text.iter_mut() {
            if text.sections[0].value.is_empty() {
                text.sections[0].value = format!("{}:", player.name.clone());
            }
            text.sections[1].value = player.score.to_string();
        }

        //always set local player above other players
        t.translation.z = 0.1;

        player.apply_input(&mut t, &client_tick);

        for i in (0..objects.bolt_pos.len()).rev() {
            let dot = &objects.bolt_pos[i];
            if (dot.x - t.translation.x).abs() < 10.0 && (dot.y - t.translation.y).abs() < 10.0 {
                objects.bolt_pos.remove(i);
            }
        }
    }
}

pub fn enemy_loop(
    mut query_enemy: Query<(&mut Transform, &mut Enemy)>,
    mut query_text: Query<&mut Text, With<NamePlates>>,
    mut objects: ResMut<Objects>,
    client_tick: Res<ClientTick>,
) {
    for (mut t, mut enemy) in query_enemy.iter_mut() {
        for mut text in query_text.iter_mut() {
            text.sections[1].value = enemy.score.to_string();
        }

        enemy.apply_input(&mut t, &client_tick);

        for i in (0..objects.bolt_pos.len()).rev() {
            let dot = &objects.bolt_pos[i];
            if (dot.x - t.translation.x).abs() < 10.0 && (dot.y - t.translation.y).abs() < 10.0 {
                objects.bolt_pos.remove(i);
            }
        }
    }
}

pub fn tick(mut client_tick: ResMut<ClientTick>) {
    if client_tick.pause > 0 {
        client_tick.pause -= 1;
    } else if let Some(tick) = &mut client_tick.tick {
        *tick += 1;
    }
}
