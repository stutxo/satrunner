use bevy::prelude::*;

use names::Generator;

use bevy_egui::{
    egui::{self, pos2, RichText, TextEdit},
    EguiContexts,
};

use crate::{
    game_util::resources::{ClientTick, NetworkStuff, PingTimer, PlayerName},
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

    let mut score_list: Vec<(String, i32, egui::Color32)> = Vec::new();

    for player in query_player.iter() {
        if player_name.submitted {
            score_list.push((
                player_name.name.clone(),
                player.score.try_into().unwrap(),
                egui::Color32::GREEN,
            ));
        }
    }

    for enemy in query_enemy.iter() {
        if !enemy.name.is_empty() {
            score_list.push((
                enemy.name.to_string(),
                enemy.score.try_into().unwrap(),
                egui::Color32::WHITE,
            ));
        }
    }

    score_list.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    egui::Area::new("score_board")
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            for (id, score, color) in score_list {
                ui.label(RichText::new(format!(" {}: {}", id, score)).color(color));
                ui.add_space(5.0);
            }
        });
}

pub fn setup_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameStage>>,
    mut player_name: ResMut<PlayerName>,
    mut network_stuff: ResMut<NetworkStuff>,
    query_player: Query<&Player>,
    client_tick: Res<ClientTick>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("satrunner")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
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
                    for player in query_player.iter() {
                        let input =
                            PlayerInput::new([0.0, 0.0], player.id, client_tick.tick.unwrap());

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
                    for player in query_player.iter() {
                        let input =
                            PlayerInput::new([0.0, 0.0], player.id, client_tick.tick.unwrap());

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

    egui::Window::new("satrunner")
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

pub fn add_nameplates(
    mut contexts: EguiContexts,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    query_player: Query<(&Transform, &Player)>,
    player_name: Res<PlayerName>,
) {
    let ctx = contexts.ctx_mut();

    for (player_transform, player) in query_player.iter() {
        let text_pos = get_sceen_transform_and_visibility(&camera_query, player_transform);
        let text = RichText::new(format!("{}: {}", player_name.name, player.score));
        let name_len = player_name.name.len();

        // Apply the same transformation as for the player
        egui::Area::new(format!("nameplate_{}", player.id))
            .current_pos(pos2(
                text_pos.translation.x / 10.0 / name_len as f32,
                text_pos.translation.y - 20.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(text);
                });
            });
    }
}

fn get_sceen_transform_and_visibility(
    camera_q: &Query<(&Camera, &GlobalTransform)>,
    transform: &Transform,
) -> Transform {
    let (camera, cam_gt) = camera_q.single();

    let pos = camera.world_to_viewport(cam_gt, transform.translation);

    if let Some(pos) = pos {
        Transform::from_xyz(pos.x, pos.y, 1.)
    } else {
        Transform::from_xyz(0., 0., 0.)
    }
}
