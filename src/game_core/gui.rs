use bevy::{prelude::*, utils::Instant};

use names::Generator;

use bevy_egui::{
    egui::{self, RichText, TextEdit},
    EguiContexts,
};

use crate::{
    game_util::{
        components::NamePlatesLocal,
        resources::{ClientTick, NetworkStuff, PingTimer, PlayerName},
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
            let seconds = duration.as_secs();
            let minutes = seconds / 60;

            score_list.push((
                player_name.name.clone(),
                player.score.try_into().unwrap(),
                egui::Color32::GREEN,
                seconds,
                minutes,
            ));
        }
    }

    for enemy in query_enemy.iter() {
        if !enemy.name.is_empty() {
            let duration = Instant::now() - enemy.spawn_time;
            let seconds = duration.as_secs();
            let minutes = seconds / 60;
            score_list.push((
                enemy.name.to_string(),
                enemy.score.try_into().unwrap(),
                egui::Color32::GRAY,
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
                        "[{:02}:{:02}] {}: {:02}/21⚡",
                        mins % 60,
                        secs % 60,
                        id,
                        score,
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
    mut query_player: Query<(&mut Player, &mut Sprite)>,
    client_tick: Res<ClientTick>,
) {
    let ctx = contexts.ctx_mut();

    for (_, mut sprite) in query_player.iter_mut() {
        sprite.color = Color::GRAY;
    }

    egui::Window::new("☔ rain.run              ")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label("Weekly Challenge 🏆:");
            ui.label("Collect 21 bolts as fast as you can!");
            ui.label("May the gods be in your favor...");
            ui.horizontal(|ui| {
                ui.add(
                    TextEdit::singleline(&mut player_name.name)
                        .char_limit(25)
                        .desired_width(100.0)
                        .hint_text("Enter Name"),
                );
                if ui.button("play").clicked() && !player_name.name.is_empty() {
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

                    //send fake input to sync client and server before game starts
                    for (mut player, _) in query_player.iter_mut() {
                        player.spawn_time = Some(Instant::now());
                        let input =
                            PlayerInput::new([0.0, 0.0], player.id, client_tick.tick.unwrap());

                        player.name = player_name.name.clone();

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

                    next_state.set(GameStage::InGame);
                }
            });

            ui.horizontal(|ui| {
                let mut rand_name = Generator::default();
                if ui.button("play as guest").clicked() {
                    player_name.name = rand_name.next().unwrap();
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

                    //send fake input to sync client and server before game starts
                    for (mut player, _) in query_player.iter_mut() {
                        player.spawn_time = Some(Instant::now());
                        let input =
                            PlayerInput::new([0.0, 0.0], player.id, client_tick.tick.unwrap());

                        player.name = player_name.name.clone();

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

pub fn game_over(
    mut contexts: EguiContexts,
    mut player_name: ResMut<PlayerName>,
    mut network_stuff: ResMut<NetworkStuff>,
    mut query_player: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    mut next_state: ResMut<NextState<GameStage>>,
    mut query_text: Query<&mut Text, With<NamePlatesLocal>>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("☔ rain.run              ")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            for (mut transform, mut player, mut sprite) in query_player.iter_mut() {
                let seconds = player.death_time.unwrap();
                let minutes = seconds / 60;
                if player.score == 21 {
                    ui.label("Challenge Complete! 🏆");
                } else {
                    ui.label("Challenge Failed! 🇱");
                }
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{}: {:02}/21⚡ ({:02}:{:02})",
                        player.name,
                        player.score,
                        minutes % 60,
                        seconds % 60,
                    ));
                    player_name.submitted = false;
                    sprite.color = Color::GRAY;
                    player.target = Vec2::ZERO;
                    player.pending_inputs.clear();
                    transform.translation = Vec3::new(0.0, -150.0, 0.1);

                    for mut text in query_text.iter_mut() {
                        text.sections[0].value = String::new();
                    }

                    if ui.button("play again").clicked() {
                        match network_stuff
                            .write
                            .as_mut()
                            .unwrap()
                            .try_send(ClientMessage::PlayerName(player_name.name.clone()))
                        {
                            Ok(()) => {}
                            Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
                        };
                        player.score = 0;
                        player_name.submitted = true;
                        player.spawn_time = Some(Instant::now());
                        next_state.set(GameStage::InGame);
                    }
                });
            }
        });
}
