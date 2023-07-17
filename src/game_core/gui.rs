use bevy::prelude::*;

use bevy_egui::{
    egui::{self, RichText},
    EguiContexts,
};

use super::player::{Enemy, Player};

pub fn score_board(
    mut contexts: EguiContexts,
    query_player: Query<&Player>,
    query_enemy: Query<&Enemy>,
) {
    let ctx = contexts.ctx_mut();

    let mut style = (*ctx.style()).clone();

    style.visuals.panel_fill = egui::Color32::TRANSPARENT;

    ctx.set_style(style);

    egui::Area::new("score_board")
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            for player in query_player.iter() {
                ui.label(
                    RichText::new(format!(" {} score: {}", player.id, player.score))
                        .color(egui::Color32::GREEN),
                );
            }
            for enemy in query_enemy.iter() {
                ui.label(
                    RichText::new(format!(" {} score: {}", enemy.id, enemy.score))
                        .color(egui::Color32::RED),
                );
            }
        });
}
