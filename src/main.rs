use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use game_core::{
    dots::handle_dots,
    game_loop::{enemy_loop, player_loop, tick},
    gui::{score_board, setup_menu},
    handle::handle_server,
    input::input,
    sprites::pool_dots,
};
use game_util::resources::{ClientTick, Dots, NetworkStuff, ParticlePool, PlayerName};
use network::websockets::websocket;
use std::collections::VecDeque;

mod game_core;
mod game_util;
mod network;

pub const TICK_RATE: f32 = 1. / 30.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "satrunner".to_string(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
        ))
        .add_state::<GameStage>()
        .add_systems(Startup, (websocket, pool_dots))
        .add_systems(Update, setup_menu.run_if(in_state(GameStage::Menu)))
        .add_systems(Update, (handle_server, score_board))
        .add_systems(FixedUpdate, (tick, handle_dots, enemy_loop))
        .add_systems(Update, (input).run_if(in_state(GameStage::InGame)))
        .add_systems(
            FixedUpdate,
            (player_loop).run_if(in_state(GameStage::InGame)),
        )
        .insert_resource(FixedTime::new_from_secs(TICK_RATE))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Dots::new())
        .insert_resource(ParticlePool(VecDeque::new()))
        .insert_resource(NetworkStuff::new())
        .insert_resource(ClientTick::new())
        .insert_resource(PlayerName::new())
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameStage {
    #[default]
    Menu,
    InGame,
}
