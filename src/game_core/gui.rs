use bevy::{prelude::*, utils::Instant};

use names::Generator;

use bevy_egui::{
    egui::{self, Color32, RichText, TextEdit},
    EguiContexts,
};

use crate::{
    game_util::{
        components::NamePlatesLocal,
        resources::{ClientTick, NetworkStuff, Objects, PingTimer, PlayerName},
    },
    network::messages::{ClientMessage, PlayerInput},
    GameStage,
};

use super::player::{Enemy, Player};

pub fn score_board(
    mut contexts: EguiContexts,
    query_player: Query<&Player>,
    query_enemy: Query<&Enemy>,
    player_name: Res<PlayerName>,
) {
    let ctx = contexts.ctx_mut();

    let mut style = (*ctx.style()).clone();

    style.visuals.panel_fill = egui::Color32::TRANSPARENT;

    ctx.set_style(style);

    let mut score_list: Vec<(String, i32, egui::Color32, u64, u64)> = Vec::new();

    if player_name.submitted {
        for player in query_player.iter() {
            let duration = Instant::now() - player.spawn_time.unwrap();

            let seconds = if let Some(death_time) = player.death_time {
                death_time
            } else {
                duration.as_secs()
            };
            let minutes = seconds / 60;

            score_list.push((
                player.name.to_string(),
                player.score.try_into().unwrap(),
                egui::Color32::GREEN,
                seconds,
                minutes,
            ));
        }
    }

    for enemy in query_enemy.iter() {
        if !enemy.name.is_empty() {
            let duration = &enemy.spawn_time;
            let seconds = duration.elapsed_secs() as u64;
            let minutes = seconds / 60;
            score_list.push((
                enemy.name.to_string(),
                enemy.score.try_into().unwrap(),
                egui::Color32::WHITE,
                seconds,
                minutes,
            ));
        }
    }

    score_list.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    egui::Area::new("score_board")
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            for (id, score, color, secs, mins) in score_list {
                ui.label(
                    RichText::new(format!(
                        "{}: {:02}/21⚡ ({:02}:{:02})",
                        id,
                        score,
                        mins % 60,
                        secs % 60,
                    ))
                    .color(color),
                );
                ui.add_space(5.0);
            }
        });
}

pub fn setup_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameStage>>,
    mut player_name: ResMut<PlayerName>,
    mut network_stuff: ResMut<NetworkStuff>,
    mut query_player: Query<(&mut Player, &mut Sprite, &Transform)>,
    client_tick: Res<ClientTick>,
    objects: Res<Objects>,
) {
    if client_tick.tick.unwrap_or(0) % 25 == 0 {
        for (player, _, t) in query_player.iter_mut() {
            let input = PlayerInput::new(
                t.translation.truncate().into(),
                player.id,
                client_tick.tick.unwrap(),
            );

            match network_stuff
                .write
                .as_mut()
                .unwrap()
                .try_send(ClientMessage::PlayerInput(input))
            {
                Ok(()) => {}
                Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
            };
        }
    }

    let ctx = contexts.ctx_mut();

    for (_, mut sprite, _) in query_player.iter_mut() {
        sprite.color = Color::GRAY;
    }

    egui::Window::new("☔ rain.run              ")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label("High Scores");
            ui.label(format!(
                "{}\n{}\n{}\n{}\n{}",
                objects
                    .high_scores
                    .get(0)
                    .map(|(name, score)| format!(
                        "1: {} ({:02}:{:02})",
                        name,
                        score / 60 % 60,
                        score % 60
                    ))
                    .unwrap_or_else(|| "".to_string()),
                objects
                    .high_scores
                    .get(1)
                    .map(|(name, score)| format!(
                        "2: {} ({:02}:{:02})",
                        name,
                        score / 60 % 60,
                        score % 60
                    ))
                    .unwrap_or_else(|| "".to_string()),
                objects
                    .high_scores
                    .get(2)
                    .map(|(name, score)| format!(
                        "3: {} ({:02}:{:02})",
                        name,
                        score / 60 % 60,
                        score % 60
                    ))
                    .unwrap_or_else(|| "".to_string()),
                objects
                    .high_scores
                    .get(3)
                    .map(|(name, score)| format!(
                        "4: {} ({:02}:{:02})",
                        name,
                        score / 60 % 60,
                        score % 60
                    ))
                    .unwrap_or_else(|| "".to_string()),
                objects
                    .high_scores
                    .get(4)
                    .map(|(name, score)| format!(
                        "5: {} ({:02}:{:02})",
                        name,
                        score / 60 % 60,
                        score % 60
                    ))
                    .unwrap_or_else(|| "".to_string()),
            ));

            ui.label("Weekly Challenge 🏆");
            ui.label("Collect 21 bolts as fast as you can!");
            ui.add(
                TextEdit::singleline(&mut player_name.name)
                    .char_limit(25)
                    .desired_width(125.0)
                    .hint_text("Enter Name/LN Addr"),
            );
            ui.horizontal(|ui| {
                let mut rand_name = Generator::default();
                if ui.button("Random Name").clicked() {
                    player_name.name = rand_name.next().unwrap();
                }
                if ui.button("Play").clicked() && !player_name.name.is_empty() {
                    player_name.submitted = true;
                    match network_stuff
                        .write
                        .as_mut()
                        .unwrap()
                        .try_send(ClientMessage::PlayerName(player_name.name.clone()))
                    {
                        Ok(()) => {}
                        Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
                    };

                    for (mut player, _, _) in query_player.iter_mut() {
                        player.spawn_time = Some(Instant::now());
                        player.name = player_name.name.clone();
                    }

                    next_state.set(GameStage::InGame);
                }
            });
        });
}

pub fn disconnected(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("☔ rain.run              ")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label("disconnected");
        });
}

pub fn check_disconnected(
    mut ping: ResMut<PingTimer>,
    mut next_state: ResMut<NextState<GameStage>>,
) {
    if let Some(ref mut disconnected) = ping.disconnected_rx {
        while let Ok(Some(_)) = disconnected.try_next() {
            next_state.set(GameStage::Disconnected);
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn game_over(
    mut contexts: EguiContexts,
    player_name: ResMut<PlayerName>,
    mut network_stuff: ResMut<NetworkStuff>,
    mut query_player: Query<(&Transform, &mut Player, &mut Sprite)>,
    mut next_state: ResMut<NextState<GameStage>>,
    mut query_text: Query<&mut Text, With<NamePlatesLocal>>,
    objects: Res<Objects>,
    client_tick: Res<ClientTick>,
) {
    if client_tick.tick.unwrap_or(0) % 50 == 0 {
        for (transform, player, _) in query_player.iter_mut() {
            let input = PlayerInput::new(
                transform.translation.truncate().into(),
                player.id,
                client_tick.tick.unwrap(),
            );

            match network_stuff
                .write
                .as_mut()
                .unwrap()
                .try_send(ClientMessage::PlayerInput(input))
            {
                Ok(()) => {}
                Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
            };
        }
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("☔ rain.run              ")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            for (transform, mut player, mut sprite) in query_player.iter_mut() {
                egui::Area::new("area")
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, -20.0))
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            if player.score == 21 {
                                ui.label(
                                    RichText::new("Challenge Complete! 🏆").color(Color32::WHITE),
                                );
                            }
                            let seconds = player.death_time.unwrap();
                            let minutes = seconds / 60;

                            for mut text in query_text.iter_mut() {
                                text.sections[0].value = format!(
                                    "{:02}/21\n({:02}:{:02})",
                                    player.score,
                                    minutes % 60,
                                    seconds % 60,
                                );
                            }
                            if ui.button("Play Again").clicked() {
                                match network_stuff
                                    .write
                                    .as_mut()
                                    .unwrap()
                                    .try_send(ClientMessage::PlayerName(player_name.name.clone()))
                                {
                                    Ok(()) => {}
                                    Err(e) => {
                                        error!("Error sending message: {} CHANNEL FULL???", e)
                                    }
                                };
                                player.score = 0;
                                player.spawn_time = Some(Instant::now());
                                next_state.set(GameStage::InGame);
                                player.death_time = None;
                            }
                        });
                        player.target = transform.translation.truncate();
                        sprite.color = Color::GRAY;
                    });
                ui.label("High Scores");
                ui.label(format!(
                    "{}\n{}\n{}\n{}\n{}",
                    objects
                        .high_scores
                        .get(0)
                        .map(|(name, score)| format!(
                            "1: {} ({:02}:{:02})",
                            name,
                            score / 60 % 60,
                            score % 60
                        ))
                        .unwrap_or_else(|| "".to_string()),
                    objects
                        .high_scores
                        .get(1)
                        .map(|(name, score)| format!(
                            "2: {} ({:02}:{:02})",
                            name,
                            score / 60 % 60,
                            score % 60
                        ))
                        .unwrap_or_else(|| "".to_string()),
                    objects
                        .high_scores
                        .get(2)
                        .map(|(name, score)| format!(
                            "3: {} ({:02}:{:02})",
                            name,
                            score / 60 % 60,
                            score % 60
                        ))
                        .unwrap_or_else(|| "".to_string()),
                    objects
                        .high_scores
                        .get(3)
                        .map(|(name, score)| format!(
                            "4: {} ({:02}:{:02})",
                            name,
                            score / 60 % 60,
                            score % 60
                        ))
                        .unwrap_or_else(|| "".to_string()),
                    objects
                        .high_scores
                        .get(4)
                        .map(|(name, score)| format!(
                            "5: {} ({:02}:{:02})",
                            name,
                            score / 60 % 60,
                            score % 60
                        ))
                        .unwrap_or_else(|| "".to_string()),
                ));
            }
        });
}
